use anyhow::anyhow;
use itertools::Itertools;
use thiserror::Error as ThisError;
#[derive(Debug, ThisError)]
pub enum CreateError {
    #[error("Can't create directory: {0}")]
    CantCreateDirectory(#[source] anyhow::Error),
    #[error("Can't git init: {0}")]
    CantGitInit(#[source] anyhow::Error),
    #[error("Not found home directory")]
    NotFoundHomeDir,
}

#[derive(Debug, ThisError)]
pub enum ListError {
    #[error("Can't read directories: {0}")]
    CantReadDirectories(String),
    #[error("Not found home directory")]
    NotFoundHomeDir,
}

pub fn create() -> Result<(), CreateError> {
    let path = {
        // get home directory
        let home = dirs::home_dir().ok_or(CreateError::NotFoundHomeDir)?;
        // create a new string with a now time.
        let now = chrono::Local::now();
        let formatted_now = now.format("%Y-%m-%d_%H:%M:%S").to_string();
        home.join("work").join(formatted_now)
    };

    // create a new directory with now_str
    std::fs::create_dir_all(&path)
        .map_err(|e| CreateError::CantCreateDirectory(anyhow::Error::from(e)))?;

    git2::Repository::init(&path).map_err(|e| CreateError::CantGitInit(anyhow::Error::from(e)))?;

    println!(
        "{}",
        path.to_str()
            .ok_or(CreateError::CantCreateDirectory(anyhow!("error")))?
    );

    Ok(())
}

pub fn list() -> Result<(), ListError> {
    let home = dirs::home_dir().ok_or(ListError::NotFoundHomeDir)?;
    let path = home.join("work");
    let (paths, errors): (Vec<_>, Vec<_>) = std::fs::read_dir(path)
        .map_err(|e| ListError::CantReadDirectories(e.to_string()))?
        .map(|p| p.map_err(|e| ListError::CantReadDirectories(e.to_string())))
        .partition(Result::is_ok);
    let (paths, errors): (Vec<_>, Vec<_>) = (
        paths
            .into_iter()
            .map(Result::unwrap)
            .map(|p| p.path().to_str().unwrap().to_string())
            .sorted()
            .collect(),
        errors.into_iter().map(Result::unwrap_err).collect(),
    );
    if !errors.is_empty() {
        eprintln!("{:?}", errors);
    }

    for p in paths {
        println!("{}", p);
    }

    Ok(())
}
