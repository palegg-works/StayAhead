mod config;
mod persistence;

use config::STORAGE_KEY;

/*
 * Web
 */

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
pub use web::import_data;

/*
 * Android
 */

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "android")]
use android::state_file_path;

/*
 * IOS
 */

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "ios")]
use ios::state_file_path;

/*
 * Desktop
 */

#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
mod desktop;

#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
use desktop::state_file_path;

#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
pub use desktop::import_data;
