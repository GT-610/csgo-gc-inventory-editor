use crate::inventory::vdf::{VdfParser, VdfValue};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
#[allow(clippy::derivable_impls)]
pub struct Config {
    pub competitive_rank: u32,
    pub competitive_wins: u32,
    pub wingman_rank: u32,
    pub wingman_wins: u32,
    pub dangerzone_rank: u32,
    pub dangerzone_wins: u32,
    pub vac_banned: bool,
    pub cmd_friendly: u32,
    pub cmd_teaching: u32,
    pub cmd_leader: u32,
    pub player_level: u32,
    pub player_cur_xp: u32,
    pub destroy_used_items: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            competitive_rank: 18,
            competitive_wins: 666,
            wingman_rank: 18,
            wingman_wins: 777,
            dangerzone_rank: 15,
            dangerzone_wins: 888,
            vac_banned: false,
            cmd_friendly: 666,
            cmd_teaching: 777,
            cmd_leader: 888,
            player_level: 39,
            player_cur_xp: 4999,
            destroy_used_items: true,
        }
    }
}

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load(path: &Path) -> Result<Config, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

        let vdf = VdfParser::parse(&content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;

        let mut config = Config::default();

        if let Some(VdfValue::Object(ranks)) = vdf.get("ranks") {
            if let Some(VdfValue::String(s)) = ranks.get("competitive_rank") {
                config.competitive_rank = s.parse().unwrap_or(config.competitive_rank);
            }
            if let Some(VdfValue::String(s)) = ranks.get("competitive_wins") {
                config.competitive_wins = s.parse().unwrap_or(config.competitive_wins);
            }
            if let Some(VdfValue::String(s)) = ranks.get("wingman_rank") {
                config.wingman_rank = s.parse().unwrap_or(config.wingman_rank);
            }
            if let Some(VdfValue::String(s)) = ranks.get("wingman_wins") {
                config.wingman_wins = s.parse().unwrap_or(config.wingman_wins);
            }
            if let Some(VdfValue::String(s)) = ranks.get("dangerzone_rank") {
                config.dangerzone_rank = s.parse().unwrap_or(config.dangerzone_rank);
            }
            if let Some(VdfValue::String(s)) = ranks.get("dangerzone_wins") {
                config.dangerzone_wins = s.parse().unwrap_or(config.dangerzone_wins);
            }
        }

        if let Some(VdfValue::String(s)) = vdf.get("vac_banned") {
            config.vac_banned = s == "1";
        }
        if let Some(VdfValue::String(s)) = vdf.get("cmd_friendly") {
            config.cmd_friendly = s.parse().unwrap_or(config.cmd_friendly);
        }
        if let Some(VdfValue::String(s)) = vdf.get("cmd_teaching") {
            config.cmd_teaching = s.parse().unwrap_or(config.cmd_teaching);
        }
        if let Some(VdfValue::String(s)) = vdf.get("cmd_leader") {
            config.cmd_leader = s.parse().unwrap_or(config.cmd_leader);
        }
        if let Some(VdfValue::String(s)) = vdf.get("player_level") {
            config.player_level = s.parse().unwrap_or(config.player_level);
        }
        if let Some(VdfValue::String(s)) = vdf.get("player_cur_xp") {
            config.player_cur_xp = s.parse().unwrap_or(config.player_cur_xp);
        }
        if let Some(VdfValue::String(s)) = vdf.get("destroy_used_items") {
            config.destroy_used_items = s == "1";
        }

        Ok(config)
    }

    pub fn save(config: &Config, path: &Path) -> Result<(), String> {
        let mut ranks = std::collections::HashMap::new();
        ranks.insert(
            "competitive_rank".to_string(),
            VdfValue::String(config.competitive_rank.to_string()),
        );
        ranks.insert(
            "competitive_wins".to_string(),
            VdfValue::String(config.competitive_wins.to_string()),
        );
        ranks.insert(
            "wingman_rank".to_string(),
            VdfValue::String(config.wingman_rank.to_string()),
        );
        ranks.insert(
            "wingman_wins".to_string(),
            VdfValue::String(config.wingman_wins.to_string()),
        );
        ranks.insert(
            "dangerzone_rank".to_string(),
            VdfValue::String(config.dangerzone_rank.to_string()),
        );
        ranks.insert(
            "dangerzone_wins".to_string(),
            VdfValue::String(config.dangerzone_wins.to_string()),
        );

        let mut rarity_weights = std::collections::HashMap::new();
        rarity_weights.insert("1".to_string(), VdfValue::String("10000000".to_string()));
        rarity_weights.insert("2".to_string(), VdfValue::String("2000000".to_string()));
        rarity_weights.insert("3".to_string(), VdfValue::String("400000".to_string()));
        rarity_weights.insert("4".to_string(), VdfValue::String("80000".to_string()));
        rarity_weights.insert("5".to_string(), VdfValue::String("16000".to_string()));
        rarity_weights.insert("6".to_string(), VdfValue::String("3200".to_string()));
        rarity_weights.insert("99".to_string(), VdfValue::String("1280".to_string()));

        let mut root = std::collections::HashMap::new();
        root.insert("ranks".to_string(), VdfValue::Object(ranks));
        root.insert(
            "vac_banned".to_string(),
            VdfValue::String(if config.vac_banned {
                "1".to_string()
            } else {
                "0".to_string()
            }),
        );
        root.insert(
            "cmd_friendly".to_string(),
            VdfValue::String(config.cmd_friendly.to_string()),
        );
        root.insert(
            "cmd_teaching".to_string(),
            VdfValue::String(config.cmd_teaching.to_string()),
        );
        root.insert(
            "cmd_leader".to_string(),
            VdfValue::String(config.cmd_leader.to_string()),
        );
        root.insert(
            "player_level".to_string(),
            VdfValue::String(config.player_level.to_string()),
        );
        root.insert(
            "player_cur_xp".to_string(),
            VdfValue::String(config.player_cur_xp.to_string()),
        );
        root.insert(
            "rarity_weights".to_string(),
            VdfValue::Object(rarity_weights),
        );
        root.insert(
            "destroy_used_items".to_string(),
            VdfValue::String(if config.destroy_used_items {
                "1".to_string()
            } else {
                "0".to_string()
            }),
        );

        let content = VdfParser::to_string(&VdfValue::Object(root));

        fs::write(path, content).map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }
}
