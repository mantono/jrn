use std::{env, path::PathBuf};

use crate::cfg::Config;
use git2::{
    Commit, Cred, FetchOptions, Index, Oid, PushOptions, Remote, RemoteCallbacks, Repository,
    Status, StatusEntry, StatusOptions, Tree,
};

pub fn sync(cfg: &Config) -> Result<usize, git2::Error> {
    // Read or init repo if none is present
    let repo: Repository = match Repository::open(cfg.dir()) {
        Ok(repo) => repo,
        Err(_) => Repository::init(cfg.git_dir())?,
    };
    println!("Using repository {:?}", repo.path());

    // Check for changes in tree (unstaged changes)
    let mut options = StatusOptions::new();
    options.include_untracked(true).recurse_untracked_dirs(true);

    let mut index: Index = repo.index()?;

    let changes: usize = repo
        .statuses(Some(&mut options))
        .unwrap()
        .iter()
        .filter(|f| filter_status(&f.status()))
        // Add any changes to index if present
        .map(|f| try_add(&mut index, f))
        .filter_map(|f| f.ok())
        .count();

    if changes > 0 {
        let oid: Oid = index.write_tree()?;
        let sign = repo.signature()?;
        let message = format!("Add/update {} files", changes);

        let tree: Tree = repo.find_tree(oid)?;
        index.add_all(&["."], git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let parent_commit: Commit = repo.head()?.peel_to_commit()?;

        // Commit changes
        let commit: Oid =
            repo.commit(Some("HEAD"), &sign, &sign, &message, &tree, &[&parent_commit])?;
        println!("Created commit {}", commit);
    }

    // Pull from remote (if remote exists)
    pull(&repo)?
        // 4. Push to remote (if remote exists)
        .map(|remote| push(remote))
        .transpose()?;

    Ok(changes)
}

fn filter_status(status: &Status) -> bool {
    if status.is_conflicted() {
        panic!("File has a conflict that needs to be resolved manually")
    }
    let include = Status::all() ^ Status::CURRENT ^ Status::IGNORED ^ Status::CONFLICTED;
    status.intersects(include)
}

fn try_add<'a>(index: &mut Index, file: StatusEntry<'a>) -> Result<StatusEntry<'a>, git2::Error> {
    let path: PathBuf = PathBuf::from(file.path().unwrap());
    index.add_path(&path)?;
    index.write_tree()?;
    Ok(file)
}

fn pull(repo: &Repository) -> Result<Option<Remote>, git2::Error> {
    let mut remote: Remote = match repo.find_remote("origin") {
        Ok(remote) => remote,
        Err(_) => return Ok(None),
    };
    let branch: &str = "master";
    let mut options = FetchOptions::new();
    options.remote_callbacks(callback());
    remote.fetch(&[branch], Some(&mut options), None)?;

    Ok(Some(remote))
}

fn push(mut remote: Remote) -> Result<(), git2::Error> {
    let branch: &str = "master";
    let refspc = format!("refs/heads/{0}:refs/heads/{0}", branch);
    let refs: [&str; 1] = [&refspc];
    let mut options = PushOptions::new();
    options.remote_callbacks(callback());
    remote.push(&refs, Some(&mut options))?;

    Ok(())
}

fn callback() -> RemoteCallbacks<'static> {
    let mut cb = RemoteCallbacks::new();
    cb.credentials(|_url, username, _allowed_types| {
        Cred::ssh_key(
            username.unwrap(),
            None,
            std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });

    cb
}
