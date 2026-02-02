use crate::inventory::models::Inventory;
use crate::inventory::parser::{InventoryParser, VdfInventoryParser};
use std::path::Path;

static DEFAULT_PARSER: VdfInventoryParser = VdfInventoryParser;

pub struct InventoryLoader;

impl InventoryLoader {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Inventory, InventoryLoadError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| InventoryLoadError::Io(e))?;

        Self::parse_from_str(&content)
    }

    pub fn load_from_game_dir<P: AsRef<Path>>(game_dir: P) -> Result<Inventory, InventoryLoadError> {
        let inventory_path = game_dir.as_ref().join("csgo_gc").join("inventory.txt");
        Self::load(&inventory_path)
    }

    pub fn parse_from_str(content: &str) -> Result<Inventory, InventoryLoadError> {
        DEFAULT_PARSER
            .parse(content)
            .map_err(InventoryLoadError::Parse)
    }

    pub fn save<P: AsRef<Path>>(
        inventory: &Inventory,
        path: P,
    ) -> Result<(), InventorySaveError> {
        let content = DEFAULT_PARSER
            .serialize(inventory)
            .map_err(InventorySaveError::Serialize)?;

        std::fs::write(path, content).map_err(InventorySaveError::Io)
    }

    pub fn save_to_game_dir<P: AsRef<Path>>(
        inventory: &Inventory,
        game_dir: P,
    ) -> Result<(), InventorySaveError> {
        let inventory_path = game_dir.as_ref().join("csgo_gc").join("inventory.txt");
        Self::save(inventory, &inventory_path)
    }
}

pub struct InventoryLoaderRef<'a> {
    parser: &'a dyn InventoryParser,
}

impl<'a> InventoryLoaderRef<'a> {
    pub fn new(parser: &'a dyn InventoryParser) -> Self {
        Self { parser }
    }

    pub fn load<P: AsRef<Path>>(&self, path: P) -> Result<Inventory, InventoryLoadError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| InventoryLoadError::Io(e))?;

        self.parse_from_str(&content)
    }

    pub fn parse_from_str(&self, content: &str) -> Result<Inventory, InventoryLoadError> {
        self.parser
            .parse(content)
            .map_err(InventoryLoadError::Parse)
    }

    pub fn save<P: AsRef<Path>>(
        &self,
        inventory: &Inventory,
        path: P,
    ) -> Result<(), InventorySaveError> {
        let content = self
            .parser
            .serialize(inventory)
            .map_err(InventorySaveError::Serialize)?;

        std::fs::write(path, content).map_err(InventorySaveError::Io)
    }
}

#[derive(Debug)]
pub enum InventoryLoadError {
    Io(std::io::Error),
    Parse(Box<dyn std::error::Error + Send + Sync>),
}

impl std::fmt::Display for InventoryLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryLoadError::Io(e) => write!(f, "IO Error: {}", e),
            InventoryLoadError::Parse(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl std::error::Error for InventoryLoadError {}

#[derive(Debug)]
pub enum InventorySaveError {
    Io(std::io::Error),
    Serialize(Box<dyn std::error::Error + Send + Sync>),
}

impl std::fmt::Display for InventorySaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventorySaveError::Io(e) => write!(f, "IO Error: {}", e),
            InventorySaveError::Serialize(e) => write!(f, "Serialize Error: {}", e),
        }
    }
}

impl std::error::Error for InventorySaveError {}
