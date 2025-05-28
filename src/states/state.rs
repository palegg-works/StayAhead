use super::sync_mode::SyncMode;
use super::task::{MyTask, SerializableTask};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, write};
use std::path::PathBuf;

const STORAGE_KEY: &str = "PaleggWorks_StayAhead_AppState";

#[derive(Debug, Clone)]
pub struct AppState {
    pub tasks: Signal<Option<Vec<MyTask>>>,
    pub sync_mode: Signal<SyncMode>,
    pub github_pat: Signal<Option<String>>,
    pub gist_id: Signal<Option<String>>,
    pub gist_file_name: Signal<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableState {
    pub tasks: Option<Vec<SerializableTask>>,

    #[serde(default)]
    pub github_pat: Option<String>,

    #[serde(default)]
    pub gist_id: Option<String>,

    #[serde(default)]
    pub gist_file_name: Option<String>,
}

impl From<&AppState> for SerializableState {
    fn from(state: &AppState) -> Self {
        let tasks = (state.tasks)();
        if let Some(tasks) = tasks {
            let tasks: Vec<SerializableTask> = tasks.iter().map(SerializableTask::from).collect();
            SerializableState {
                tasks: Some(tasks),
                github_pat: (state.github_pat)(),
                gist_id: (state.gist_id)(),
                gist_file_name: (state.gist_file_name)(),
            }
        } else {
            SerializableState {
                tasks: None,
                github_pat: (state.github_pat)(),
                gist_id: (state.gist_id)(),
                gist_file_name: (state.gist_file_name)(),
            }
        }
    }
}

impl TryFrom<SerializableState> for AppState {
    type Error = chrono::ParseError;

    fn try_from(state: SerializableState) -> Result<Self, Self::Error> {
        if let Some(tasks) = state.tasks.clone() {
            let tasks = tasks
                .iter()
                .map(|t| {
                    MyTask::try_from(t).expect("Conversion failed from SerializableTask to MyTask")
                })
                .collect();

            Ok(AppState {
                tasks: Signal::new(Some(tasks)),
                github_pat: Signal::new(state.github_pat),
                gist_id: Signal::new(state.gist_id),
                gist_file_name: Signal::new(state.gist_file_name),
                sync_mode: Signal::new(SyncMode::NotSynced),
            })
        } else {
            Ok(AppState {
                tasks: Signal::new(None),
                github_pat: Signal::new(state.github_pat),
                gist_id: Signal::new(state.gist_id),
                gist_file_name: Signal::new(state.gist_file_name),
                sync_mode: Signal::new(SyncMode::NotSynced),
            })
        }
    }
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn state_file_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(STORAGE_KEY.to_string() + ".json")
}

#[cfg(target_os = "ios")]
fn state_file_path() -> PathBuf {
    use objc::runtime::{Class, Object};
    use objc::{msg_send, sel, sel_impl};
    use std::ffi::CStr;
    use std::path::PathBuf;

    unsafe {
        // NSFileManager* fileManager = [NSFileManager defaultManager];
        let file_manager: *mut Object =
            msg_send![Class::get("NSFileManager").unwrap(), defaultManager];

        // NSArray<NSURL*>* urls = [fileManager URLsForDirectory:NSDocumentDirectory inDomains:NSUserDomainMask];
        let urls: *mut Object = msg_send![
            file_manager,
            URLsForDirectory: 9u64 /* NSDocumentDirectory */
            inDomains: 1u64 /* NSUserDomainMask */
        ];

        // NSURL* documentsURL = [urls firstObject];
        let url: *mut Object = msg_send![urls, firstObject];

        // NSString* path = [url path];
        let nsstring: *mut Object = msg_send![url, path];

        // const char* cstr = [path UTF8String];
        let cstr: *const std::os::raw::c_char = msg_send![nsstring, UTF8String];

        let path = CStr::from_ptr(cstr)
            .to_str()
            .expect("UTF8 conversion failed");
        PathBuf::from(path).join(STORAGE_KEY.to_string() + ".json")
    }
}

// Reference: https://github.com/DioxusLabs/dioxus/discussions/3475
#[cfg(target_os = "android")]
fn state_file_path() -> PathBuf {
    use jni::objects::{JObject, JString};
    use jni::JNIEnv;

    let (tx, rx) = std::sync::mpsc::channel();

    fn run(env: &mut JNIEnv<'_>, activity: &JObject<'_>) -> Result<PathBuf, jni::errors::Error> {
        let files_dir = env
            .call_method(activity, "getFilesDir", "()Ljava/io/File;", &[])?
            .l()?;
        let files_dir: JString<'_> = env
            .call_method(files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])?
            .l()?
            .into();
        let files_dir: String = env.get_string(&files_dir)?.into();
        Ok(PathBuf::from(files_dir))
    }

    dioxus::mobile::wry::prelude::dispatch(move |env, activity, _webview| {
        tx.send(run(env, activity)).unwrap()
    });

    rx.recv()
        .unwrap()
        .expect("Cannot get a valid path to save data on Android")
        .join(STORAGE_KEY.to_string() + ".json")
}

impl AppState {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save(&self) {
        let serilizable: SerializableState = self.into();
        if let Ok(json) = serde_json::to_string_pretty(&serilizable) {
            let _ = write(state_file_path(), json);
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save(&self) {
        use gloo_storage::{LocalStorage, Storage};
        let serializable: SerializableState = self.into();
        let _ = LocalStorage::set(STORAGE_KEY, &serializable);
    }

    #[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
    pub fn export_data(&self) -> bool {
        let mut exported = false;

        let serilizable: SerializableState = self.into();
        if let Ok(json) = serde_json::to_string_pretty(&serilizable) {
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

    #[cfg(target_arch = "wasm32")]
    pub fn export_data(&self) -> bool {
        use js_sys::Array;
        use wasm_bindgen::{JsCast, JsValue};
        use web_sys::{window, Blob, BlobPropertyBag, HtmlAnchorElement, Url};

        let mut exported = false;

        let serilizable: SerializableState = self.into();
        if let Ok(json) = serde_json::to_string_pretty(&serilizable) {
            let data_array = Array::new();
            data_array.push(&JsValue::from_str(&json));

            let mut blob_options = BlobPropertyBag::new();
            blob_options.type_("application/json");

            if let Ok(blob) = Blob::new_with_str_sequence_and_options(&data_array, &blob_options) {
                if let Ok(url) = Url::create_object_url_with_blob(&blob) {
                    if let Some(window) = window() {
                        if let Some(document) = window.document() {
                            if let Ok(elem_a) = document.create_element("a") {
                                if let Ok(a) = elem_a.dyn_into::<HtmlAnchorElement>() {
                                    a.set_href(&url);
                                    a.set_download("ExportData_StayAhead.json");
                                    //a.style().set_property("display", "none").ok();
                                    document.body().unwrap().append_child(&a).unwrap();
                                    a.click();
                                    document.body().unwrap().remove_child(&a).unwrap();
                                    Url::revoke_object_url(&url).unwrap();

                                    exported = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        exported
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LoadError {
    #[error("Failed to read state file: {0}")]
    FileNotFound(#[from] std::io::Error),

    #[error("Failed to parse JSON state: {0}")]
    InvalidJson(#[from] serde_json::Error),
}

impl SerializableState {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load() -> Result<Self, LoadError> {
        let data = read_to_string(state_file_path())?;
        let parsed = serde_json::from_str::<SerializableState>(&data)?;
        Ok(parsed)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn load() -> Result<Self, LoadError> {
        use gloo_storage::{LocalStorage, Storage};
        LocalStorage::get(STORAGE_KEY).map_err(|e| {
            LoadError::FileNotFound(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("LocalStorage error: {}", e),
            ))
        })
    }
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
pub fn import_data() -> Option<SerializableState> {
    let import_file_path = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_title("Select a JSON file to import")
        .pick_file()?;

    let data = read_to_string(import_file_path).ok()?;
    let parsed: SerializableState = serde_json::from_str(&data).ok()?;
    Some(parsed)
}

#[cfg(target_arch = "wasm32")]
pub fn import_data<F>(on_success: F)
where
    F: 'static + FnMut(SerializableState),
{
    use gloo_file::{callbacks::read_as_text, File};
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    use web_sys::{window, HtmlInputElement};

    let document = window().unwrap().document().unwrap();
    let input: HtmlInputElement = document
        .create_element("input")
        .unwrap()
        .dyn_into()
        .unwrap();

    input.set_type("file");
    input.set_accept(".json");
    input.set_hidden(true);
    document.body().unwrap().append_child(&input).unwrap();

    // Nested move happens later so I need to manual wrap it
    let mut on_success = Some(on_success);

    let onchange = Closure::<dyn FnMut(web_sys::Event)>::new({
        let input = input.clone();
        move |_event: web_sys::Event| {
            let file_list = input.files();
            if let Some(files) = file_list {
                if let Some(file) = files.get(0) {
                    let file = File::from(file);
                    let mut on_success = on_success.take().unwrap();
                    let _reader = read_as_text(&file, move |res| {
                        if let Ok(text) = res {
                            if let Ok(parsed) = serde_json::from_str::<SerializableState>(&text) {
                                on_success(parsed);
                            }
                        }
                    });

                    // Leak on purpose assuming this action does not happen many times
                    Box::leak(Box::new(_reader));
                }
            }
        }
    });

    input.set_onchange(Some(onchange.as_ref().unchecked_ref()));
    onchange.forget();
    input.click();
}
