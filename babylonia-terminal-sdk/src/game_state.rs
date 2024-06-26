use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::{
    fs::{create_dir_all, read_to_string, File},
    io::AsyncWriteExt,
};

use crate::utils::kuro_prod_api::GameInfo;

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    ProtonNotInstalled,
    DXVKNotInstalled,
    FontNotInstalled,
    DependecieNotInstalled,
    GameNotInstalled,
    GameNeedUpdate,
    GameNotPatched,
    GameInstalled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub config_dir: PathBuf,
    pub is_wine_installed: bool,
    pub is_dxvk_installed: bool,
    pub is_font_installed: bool,
    pub is_dependecies_installed: bool,
    pub game_dir: Option<PathBuf>,
    pub is_game_installed: bool,
    pub is_game_patched: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfigPath {
    path: PathBuf,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            config_dir: dirs::home_dir().unwrap().join(".babylonia-terminal"),
            is_wine_installed: false,
            is_dxvk_installed: false,
            is_font_installed: false,
            is_dependecies_installed: false,
            game_dir: None,
            is_game_installed: false,
            is_game_patched: false,
        }
    }
}

impl GameState {
    pub async fn get_config_directory() -> PathBuf {
        let path = home_dir().unwrap().join(".babylonia-terminal"); // I will try to change that to a dynamic one if people want to change the config dir

        let _ = create_dir_all(path.clone()).await;

        path
    }

    async fn get_config_file_path() -> PathBuf {
        GameState::get_config_directory()
            .await
            .join("babylonia-terminal-config")
    }

    pub async fn set_game_dir(path: Option<PathBuf>) -> anyhow::Result<()> {
        let mut config = GameState::get_config().await;
        config.game_dir = path;
        GameState::save_config(config).await?;
        Ok(())
    }

    pub async fn get_game_dir() -> Option<PathBuf> {
        GameState::get_config().await.game_dir
    }

    async fn try_get_config_file() -> anyhow::Result<File> {
        let _ = tokio::fs::create_dir(GameState::get_config_directory().await).await;

        Ok(tokio::fs::File::create(GameState::get_config_file_path().await).await?)
    }

    pub async fn save_config(config: GameConfig) -> anyhow::Result<()> {
        let mut file = GameState::try_get_config_file().await?;
        let content = serde_json::to_string(&config)?;
        file.write_all(content.as_bytes()).await?;

        Ok(())
    }

    pub async fn get_config() -> GameConfig {
        let content = match read_to_string(GameState::get_config_file_path().await).await {
            Err(_) => return GameConfig::default(),
            Ok(c) => c,
        };
        match serde_json::from_str::<GameConfig>(&content) {
            Ok(config) => return config,
            Err(_) => return GameConfig::default(),
        }
    }

    pub async fn get_current_state() -> anyhow::Result<Self> {
        let config = GameState::get_config().await;

        if !config.is_wine_installed {
            return Ok(GameState::ProtonNotInstalled);
        }

        if !config.is_dxvk_installed {
            return Ok(GameState::DXVKNotInstalled);
        }

        if !config.is_font_installed {
            return Ok(GameState::FontNotInstalled);
        }

        if !config.is_dependecies_installed {
            return Ok(GameState::DependecieNotInstalled);
        }

        if !config.is_game_installed {
            return Ok(GameState::GameNotInstalled);
        }

        if GameInfo::get_info().await?.need_update().await? {
            return Ok(GameState::GameNeedUpdate);
        }

        if !config.is_game_patched {
            return Ok(GameState::GameNotPatched);
        }

        Ok(GameState::GameInstalled)
    }
}
