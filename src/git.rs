use crate::cfg::Config;
use git2::Repository;

pub fn sync(cfg: &Config) -> Result<(), git2::Error> {
    let repo: Repository = match Repository::open(cfg.dir()) {
        Ok(repo) => repo,
        Err(_) => Repository::init(cfg.git_dir())?,
    };
    println!("Using repository {:?}", repo.path());
    // 1. Check for changes
    // 2. Add and commit any changes if present
    // 3. Pull form remote (if any, else stop)
    // 4. Push to remote (if no conflict)
    Ok(())
}
