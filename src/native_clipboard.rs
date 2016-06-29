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

    pub struct WindowsClipboard;

    impl ::Clipboard for WindowsClipboard {
        fn copy(&mut self, text: &str) {
            unimplemented!();
        }

        fn get_paste_text(&self) -> &str {
            unimplemented!();
        }
    }

    pub type NativeClipboard = WindowsClipboard;
}

#[cfg(target_os = "macos")]
pub type NativeClipboard = mac_clipboard::CocoaClipboard;

#[cfg(target_os = "windows")]
pub type NativeClipboard = windows_clipboard::WindowsClipboard;
