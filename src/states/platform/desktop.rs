use super::STORAGE_KEY;
use crate::states::{decode, encode};
use crate::{AppState, SerializableState};
use std::path::PathBuf;

pub fn state_file_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(STORAGE_KEY.to_string() + ".json")
}

/*
 * AppState method for exporting data
 */

impl AppState {
    pub fn export_data(&self) -> bool {
        let mut exported = false;

        let mut serializable: SerializableState = self.into();

        if let Some(github_pat) = serializable.github_pat {
            let encrypted_pat = encode(&github_pat);
            serializable.github_pat = Some(encrypted_pat);
        }

        if let Ok(json) = serde_json::to_string_pretty(&serializable) {
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name("ExportData_StayAhead.json")
                .save_file()
            {
                std::fs::write(&path, json).expect("Failed to write to output file!");
                exported = true;
            }
        }

        exported
    }
}

/*
 * SerializableState method for importing data
 */

pub fn import_data() -> Option<SerializableState> {
    let import_file_path = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_title("Select a JSON file to import")
        .pick_file()?;

    let data = std::fs::read_to_string(import_file_path).ok()?;
    let mut parsed: SerializableState = serde_json::from_str(&data).ok()?;

    if let Some(github_pat) = parsed.github_pat {
        let decoded_pat = decode(&github_pat);
        parsed.github_pat = Some(decoded_pat);
    }

    Some(parsed)
}
