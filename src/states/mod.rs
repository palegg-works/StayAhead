mod encoding;
mod motivation;
mod platform;
mod serializable_state;
mod serializable_task;
mod state;
mod sync_mode;
mod task;

pub use encoding::{decode, encode};
pub use motivation::MOTIVATIONAL_MSGS;
pub use serializable_state::SerializableState;
pub use state::{AppState, NoSaveAppState};
pub use sync_mode::SyncMode;
pub use task::MyTask;

/*
 * Desktop and Web
 */

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub use platform::import_data;
