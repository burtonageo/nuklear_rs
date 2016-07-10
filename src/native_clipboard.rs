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
    use std::ffi::CStr;
    use std::ptr;
    use kernel32::{GlobalAlloc, GlobalLock, GlobalUnlock};
    use user32::{CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData};
    use winapi::minwindef::{FALSE, HGLOBAL};

    const GMEM_MOVEABLE: usize = 0x0002;
    const CF_UNICODETEXT: usize = 0x000C;

    pub struct WindowsClipboard {
        _priv: ()
    }

    impl Default for WindowsClipboard {
        fn default() -> Self {
            WindowsClipboard {
                _priv: ()
            }
        }
    }

    struct ClipboardGuard;
    impl Default for ClipboardGuard {
        fn default() -> Self {
            let result = OpenClipboard(ptr::null_mut());
            assert!(result != FALSE);
            ClipboardGuard
        }
    }

    impl Drop for ClipboardGuard {
        fn drop(&mut self) {
            CloseClipboard();
        }
    }

    struct GlobalLockGuard {
        data: HANDLE,
        ptr: *mut u8
    }

    impl GlobalLockGuard {
        fn new(data: HANDLE) -> Self {
            assert!(data != ptr::null_mut());
            let ptr = GlobalLock(data) as *mut u8;
            assert!(ptr != ptr::null_mut());
            GlobalLockGuard {
                data: data,
                ptr: ptr
            }
        }

        fn get(&self) -> *mut u8 {
            self.ptr
        }
    }

    impl Drop for GlobalLockGuard {
        fn drop(&mut self) {
            GlobalUnlock(self.data);
            self.ptr = ptr::null_mut();
        }
    }

    impl ::Clipboard for WindowsClipboard {
        fn copy(&mut self, text: &str) {
            unsafe {
                let _guard = ClipboardGuard::default();

                let clip_buf = GlobalLockHandle::new(GlobalAlloc(GMEM_MOVEABLE, text.len()));
                ptr::copy_nonoverlapping(text.as_ptr(), clip_buf.get(), text.len());

                let empty_result = EmptyClipboard();
                assert!(empty_result != FALSE);
                SetClipboardData(CF_UNICODETEXT, clip_buf);
            }
        }

        fn get_paste_text(&self) -> &str {
            use std::slice;
            unsafe {
                let _guard = ClipboardGuard::default();
                let clip_buf = GlobalLockGuard::new(GetClipboardData(CF_UNICODETEXT));

                CStr::from_ptr(clip_buf.get()).to_str().unwrap_or("")
            }
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
mod unix_clipboard {
    use x11_dl::Atom;

    pub struct UnixClipboard;

    impl Default for UnixClipboard {
        fn default() -> Self {
            unimplemented!();
        }
    }

    impl ::Clipboard for UnixClipboard {
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

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
pub type NativeClipboard = unix_clipboard::UnixClipboard;

#[cfg(all(test, any(target_os = "macos", target_os = "windows")))]
mod native_clipboard_tests {
    use super::NativeClipboard;
    use ::Clipboard;
    #[test]
    fn test_native_clipboard() {
        const TEST_TEXT: &'static str = "BoomShakalaka";
        let mut clipboard = NativeClipboard::default();

        // Save the current clipboard text
        let current_clipboard_text = clipboard.get_paste_text().to_string();

        clipboard.copy(TEST_TEXT);
        assert_eq!(clipboard.get_paste_text(), TEST_TEXT);

        // And restore the clipboard to its previous state after the test
        clipboard.copy(&current_clipboard_text);
    }
}
