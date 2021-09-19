use std::{
    ops::BitAnd,
    path::{Path, PathBuf},
};

use crate::cfg::Config;
use git2::{Commit, Index, Oid, Repository, Status, StatusEntry, StatusOptions, Tree};

pub fn sync(cfg: &Config) -> Result<usize, git2::Error> {
    let repo: Repository = match Repository::open(cfg.dir()) {
        Ok(repo) => repo,
        Err(_) => Repository::init(cfg.git_dir())?,
    };
    println!("Using repository {:?}", repo.path());

    // 1. Check for changes
    let mut options = StatusOptions::new();
    options.include_untracked(true).recurse_untracked_dirs(true);

    let mut index: Index = repo.index()?;

    let changes: usize = repo
        .statuses(Some(&mut options))
        .unwrap()
        .iter()
        .filter(|f| filter_status(&f.status()))
        .inspect(|f| println!("{:?}: {:?}", f.path(), f.status()))
        // 2. Add and commit any changes if present
        .map(|f| try_add(&mut index, f))
        .filter_map(|f| f.ok())
        .inspect(|file| println!("Added file to index: {:?}", file.path()))
        .count();

    if changes > 0 {
        let oid: Oid = index.write_tree()?;
        let sign = repo.signature()?;
        let message = format!("Add/update {} files", changes);
        let tree: Tree = repo.find_tree(oid)?;

        //index.read_tree(&tree)?;
        index.add_all(&["."], git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        /*         let last_commit: git2::Commit = repo
        .head()
        .unwrap()
        .resolve()
        .unwrap()
        .peel(git2::ObjectType::Commit)
        .unwrap()
        .into_commit()
        .unwrap(); */

        let parent_commit: Commit = repo.head()?.peel_to_commit()?;

        let commit: Oid = repo.commit(None, &sign, &sign, &message, &tree, &[&parent_commit])?;
        println!("Created commit {}", commit);
        //.inspect(|f| println!("{:?}: {:?}", f.path(), f.status()))
        //.for_each(f);
    }

    // 3. Pull form remote (if any, else stop)
    // 4. Push to remote (if no conflict)

    Ok(changes)
}

fn filter_status(status: &Status) -> bool {
    let include = Status::all() ^ Status::CURRENT ^ Status::IGNORED ^ Status::CONFLICTED;
    status.intersects(include)
}

fn try_add<'a>(index: &mut Index, file: StatusEntry<'a>) -> Result<StatusEntry<'a>, git2::Error> {
    let path: PathBuf = PathBuf::from(file.path().unwrap());
    index.add_path(&path)?;
    index.write_tree()?;
    Ok(file)
}
