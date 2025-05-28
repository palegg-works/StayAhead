use super::STORAGE_KEY;
use crate::states::{decode, encode};
use crate::{AppState, SerializableState};
use std::path::PathBuf;

pub fn state_file_path() -> PathBuf {
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
