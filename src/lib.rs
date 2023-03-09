// SPDX-License-Identifier: BSL-1.0 OR Apache-2.0
//               Copyright John Nunley, 2023.
// Distributed under the Boost Software License, Version 1.0 or the Apache
//                 License, Version 2.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

//! A brief wrapper around Windows themes in Rust.

#![cfg(windows)]
#![no_std]

extern crate alloc;

use core::fmt;
use core::num::NonZeroIsize;
use core::sync::atomic::{AtomicBool, Ordering, AtomicIsize};

use windows_sys::Win32::Graphics::Gdi::FF_DONTCARE;
use windows_sys::w;

use windows_sys::Win32::Foundation::{HINSTANCE, HWND};

use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

use windows_sys::Win32::UI::WindowsAndMessaging::WNDCLASSEXW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CloseWindow, CreateWindowExW, DefWindowProcW, RegisterClassExW,
};

use windows_sys::Win32::UI::Controls::HTHEME;
use windows_sys::Win32::UI::Controls::{CloseThemeData, OpenThemeData};

/// A theme that can be applied to a window.
pub struct Theme {
    /// The theme handle.
    ///
    /// This cannot be zero, so we use `NonZeroIsize` to ensure that.
    /// Semantically, this is an `HTHEME`.
    handle: NonZeroIsize,
}

impl fmt::Debug for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct PtrWrapper(HTHEME);

        impl fmt::Debug for PtrWrapper {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Pointer::fmt(&(self.0 as *const ()), f)
            }
        }

        f.debug_tuple("Theme")
            .field(&PtrWrapper(self.handle() as _))
            .finish()
    }
}

impl Drop for Theme {
    fn drop(&mut self) {
        // SAFETY: `handle` is a valid `HTHEME`, and we need to drop it at
        // the end of its lifetime.
        unsafe {
            CloseThemeData(self.handle());
        }
    }
}

impl Theme {
    /// Get the `HTHEME`.
    fn handle(&self) -> HTHEME {
        self.handle.get() as HTHEME
    }

    /// Create a new theme from a window handle and a class name.
    /// 
    /// # Safety
    /// 
    /// `window` must be a valid `HWND`.
    pub unsafe fn with_window(
        hwnd: HWND,
        class_name: &str,
    ) -> Option<Self> {
        let mut name = class_name.encode_utf16().collect::<alloc::vec::Vec<_>>();
        name.push(0u16);

        let handle = 
            OpenThemeData(
                hwnd,
                name.as_ptr(),
            );

        NonZeroIsize::new(handle).map(|handle| {
            Self { handle }
        })
    }

    /// Create a new theme from a class name.
    pub fn new(class_name: &str) -> Option<Self> {
        let window = DummyWindow::new();

        unsafe {
            Self::with_window(
                window.0,
                class_name,
            )
        }
    }
}

/// A dummy window for getting a theme.
struct DummyWindow(HWND);

impl Drop for DummyWindow {
    fn drop(&mut self) {
        // SAFETY: `self.0` is a valid `HWND`.
        unsafe {
            CloseWindow(self.0);
        }
    }
}

impl DummyWindow {
    /// Create a new dummy window.
    fn new() -> Self {
        static CLASS_CREATED: AtomicBool = AtomicBool::new(false);
        const CLASS_NAME: *const u16 = w!("notgull::wintheme::DummyWindow");

        // If the class hasn't been registered yet, register it.
        if !CLASS_CREATED.load(Ordering::Relaxed) {
            let class = WNDCLASSEXW {
                cbSize: core::mem::size_of::<WNDCLASSEXW>() as u32,
                style: 0,
                lpfnWndProc: Some(DefWindowProcW),
                lpszClassName: CLASS_NAME,
                ..unsafe { core::mem::zeroed() }
            };

            // SAFETY: `CLASS_NAME` is a valid pointer to a null-terminated
            // string, and `class` is a valid `WNDCLASSEXW`.
            unsafe {
                RegisterClassExW(&class);
            }

            CLASS_CREATED.store(true, Ordering::Release);
        }

        // Create a window from the class.
        let hwnd = unsafe {
            CreateWindowExW(
                0,
                CLASS_NAME,
                CLASS_NAME,
                0,
                0,
                0,
                1,
                1,
                0,
                0,
                instance(),
                core::ptr::null()
            )
        };

        if hwnd == 0 {
            panic!("CreateWindowExW failed");
        }

        Self(hwnd)
    }
}

fn instance() -> HINSTANCE {
    static INSTANCE: AtomicIsize = AtomicIsize::new(0);

    let instance = INSTANCE.load(Ordering::Relaxed);
    if instance != 0 {
        return instance;
    }

    // Load the current instance handle.
    let handle = unsafe { GetModuleHandleW(core::ptr::null()) };

    if handle == 0 {
        panic!("GetModuleHandleW failed");
    }

    // Install it in our cached variable.
    let instance = INSTANCE
        .compare_exchange(instance, handle, Ordering::SeqCst, Ordering::SeqCst)
        .unwrap_or_else(|x| x);

    instance
}
