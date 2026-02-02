use std::path::PathBuf;

pub const INVENTORY_FILE_NAME: &str = "csgo_gc/inventory.txt";

#[derive(Debug)]
pub struct GameDir {
    path: PathBuf,
    inventory_path: PathBuf,
}

impl GameDir {
    pub fn new() -> Result<Self, GameDirError> {
        let exe_path = std::env::current_exe()?;
        let game_dir = exe_path.parent()
            .ok_or_else(|| GameDirError::NotFound {
                reason: "Cannot determine executable directory".to_string(),
            })?;

        let inventory_path = game_dir.join(INVENTORY_FILE_NAME);

        if !inventory_path.exists() {
            return Err(GameDirError::NotFound {
                reason: format!(
                    "inventory.txt not found at: {}",
                    inventory_path.display()
                ),
            });
        }

        Ok(Self {
            path: game_dir.to_path_buf(),
            inventory_path,
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn inventory_path(&self) -> &PathBuf {
        &self.inventory_path
    }

    pub fn inventory_exists(&self) -> bool {
        self.inventory_path.exists()
    }
}

#[derive(Debug)]
pub enum GameDirError {
    NotFound { reason: String },
    Io(std::io::Error),
}

impl std::fmt::Display for GameDirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameDirError::NotFound { reason } => {
                write!(f, "Game directory not detected: {}", reason)
            }
            GameDirError::Io(e) => write!(f, "IO Error: {}", e),
        }
    }
}

impl std::error::Error for GameDirError {}

impl From<std::io::Error> for GameDirError {
    fn from(e: std::io::Error) -> Self {
        GameDirError::Io(e)
    }
}
