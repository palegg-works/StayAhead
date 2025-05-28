use super::STORAGE_KEY;

#[cfg(not(target_arch = "wasm32"))]
use super::state_file_path;

use crate::states::{decode, encode};
use crate::{AppState, SerializableState};

#[derive(thiserror::Error, Debug)]
pub enum LoadError {
    #[error("Failed to read state file: {0}")]
    FileNotFound(#[from] std::io::Error),

    #[error("Failed to parse JSON state: {0}")]
    InvalidJson(#[from] serde_json::Error),
}

/*
 * Methods for persistent app state
 *
 * Use file based method for IOS, Android, and desktop targets
 * Use cache based method for web target
 */

#[cfg(not(target_arch = "wasm32"))]
impl AppState {
    pub fn save(&self) {
        let mut serializable: SerializableState = self.into();

        if let Some(github_pat) = serializable.github_pat {
            let encrypted_pat = encode(&github_pat);
            serializable.github_pat = Some(encrypted_pat);
        }

        if let Ok(json) = serde_json::to_string_pretty(&serializable) {
            let _ = std::fs::write(state_file_path(), json);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl SerializableState {
    pub fn load() -> Result<Self, LoadError> {
        let data = std::fs::read_to_string(state_file_path())?;
        let mut parsed = serde_json::from_str::<SerializableState>(&data)?;

        if let Some(github_pat) = parsed.github_pat {
            let decoded_pat = decode(&github_pat);
            parsed.github_pat = Some(decoded_pat);
        }

        Ok(parsed)
    }
}

#[cfg(target_arch = "wasm32")]
impl AppState {
    pub fn save(&self) {
        use gloo_storage::{LocalStorage, Storage};
        let mut serializable: SerializableState = self.into();

        if let Some(github_pat) = serializable.github_pat {
            let encrypted_pat = encode(&github_pat);
            serializable.github_pat = Some(encrypted_pat);
        }

        let _ = LocalStorage::set(STORAGE_KEY, &serializable);
    }
}

#[cfg(target_arch = "wasm32")]
impl SerializableState {
    pub fn load() -> Result<Self, LoadError> {
        use gloo_storage::{LocalStorage, Storage};

        let mut parsed: Result<SerializableState, _> =
            LocalStorage::get(STORAGE_KEY).map_err(|e| {
                LoadError::FileNotFound(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("LocalStorage error: {}", e),
                ))
            });

        if let Ok(ref mut parsed) = parsed {
            if let Some(github_pat) = &parsed.github_pat {
                let decoded_pat = decode(&github_pat);
                parsed.github_pat = Some(decoded_pat);
            }
        }

        parsed
    }
}
