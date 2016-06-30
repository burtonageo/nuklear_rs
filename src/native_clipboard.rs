#[cfg(target_os = "macos")]
mod mac_clipboard {
    use cocoa::base::id;

    pub struct CocoaClipboard(id);

    impl Default for CocoaClipboard {
        fn default() -> Self {
            unimplemented!();
        }
    }

    impl ::Clipboard for CocoaClipboard {
        fn copy(&mut self, text: &str) {
            unimplemented!();
        }

        fn get_paste_text(&self) -> &str {
            unimplemented!();
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
        let clipboard = NativeClipboard::default();
        clipboard.copy(TEST_TEXT);
        assert_eq!(clipboard.get_paste_text(), TEST_TEXT);
    }
}
