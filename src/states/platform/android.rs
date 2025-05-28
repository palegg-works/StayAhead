use super::STORAGE_KEY;
use crate::states::{decode, encode};
use crate::{AppState, SerializableState};
use std::path::PathBuf;

pub fn state_file_path() -> PathBuf {
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
