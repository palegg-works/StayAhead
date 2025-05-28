use super::STORAGE_KEY;
use crate::states::{decode, encode};
use crate::{AppState, SerializableState};
use std::path::PathBuf;

/*
 * AppState method for exporting data
 */

impl AppState {
    pub fn export_data(&self) -> bool {
        use js_sys::Array;
        use wasm_bindgen::{JsCast, JsValue};
        use web_sys::{window, Blob, BlobPropertyBag, HtmlAnchorElement, Url};

        let mut exported = false;

        let mut serializable: SerializableState = self.into();

        if let Some(github_pat) = serializable.github_pat {
            let encrypted_pat = encode(&github_pat);
            serializable.github_pat = Some(encrypted_pat);
        }

        if let Ok(json) = serde_json::to_string_pretty(&serializable) {
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

/*
 * SerializableState method for importing data
 */

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
                            if let Ok(mut parsed) = serde_json::from_str::<SerializableState>(&text)
                            {
                                if let Some(github_pat) = parsed.github_pat {
                                    let decoded_pat = decode(&github_pat);
                                    parsed.github_pat = Some(decoded_pat);
                                }

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
