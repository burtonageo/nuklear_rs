#[cfg(target_os = "macos")]
mod mac_clipboard {
    use cocoa::appkit::{NSPasteboard, NSPasteboardTypeString};
    use cocoa::base::{id, nil};
    use cocoa::foundation::{NSArray, NSString};
    use std::ffi::CStr;

    pub struct CocoaClipboard(id);

    impl Default for CocoaClipboard {
        fn default() -> Self {
            let pboard = unsafe { NSPasteboard::generalPasteboard(nil) };
            assert!(pboard != nil);
            CocoaClipboard(pboard)
        }
    }

    impl ::Clipboard for CocoaClipboard {
        fn copy(&mut self, text: &str) {
            unsafe {
                self.0.clearContents();
                let nsstr = NSString::alloc(nil).init_str(text);
                self.0.declareTypes_owner(NSArray::arrayWithObject(nil, NSPasteboardTypeString), nil);
                NSPasteboard::setString_forType(self.0, nsstr, NSPasteboardTypeString);
            }
        }

        fn get_paste_text(&self) -> &str {
            unsafe {
                let text = NSPasteboard::stringForType(self.0, NSPasteboardTypeString);
                CStr::from_ptr(text.UTF8String()).to_str().unwrap_or("")
            }
        }
    }
}

#[cfg(target_os = "windows")]
mod windows_clipboard {
    use winapi;

    pub struct WindowsClipboard {
        _priv: ()
    }

    impl Default for WindowsClipboard {
        fn default() -> Self {
            unimplemented!();
        }
    }

    impl ::Clipboard for WindowsClipboard {
        fn copy(&mut self, text: &str) {
            unimplemented!();
        }

        fn get_paste_text(&self) -> &str {
            unimplemented!();
        }
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod other_unix_clipboard {
    pub struct OtherUnixClipboard;

    impl Default for OtherUnixClipboard {
        fn default() -> Self {
            unimplemented!();
        }
    }

    impl ::Clipboard for OtherUnixClipboard {
        fn copy(&mut self, text: &str) {
            unimplemented!();
        }

        fn get_paste_text(&self) -> &str {
            unimplemented!();
        }
    }
}

#[cfg(target_os = "macos")]
pub type NativeClipboard = mac_clipboard::CocoaClipboard;

#[cfg(target_os = "windows")]
pub type NativeClipboard = windows_clipboard::WindowsClipboard;

#[cfg(all(test, any(target_os = "macos", target_os = "windows")))]
mod native_clipboard_tests {
    use super::NativeClipboard;
    use ::Clipboard;
    #[test]
    fn test_native_clipboard() {
        const TEST_TEXT: &'static str = "BoomShakalaka";
        let mut clipboard = NativeClipboard::default();
        clipboard.copy(TEST_TEXT);
        assert_eq!(clipboard.get_paste_text(), TEST_TEXT);
    }
}
