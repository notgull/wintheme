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
use core::mem::MaybeUninit;
use core::num::NonZeroIsize;
use core::sync::atomic::{AtomicBool, AtomicIsize, Ordering};

use windows_sys::w;

use windows_sys::Win32::Foundation::{HINSTANCE, HWND};

use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

use windows_sys::Win32::UI::WindowsAndMessaging::WNDCLASSEXW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CloseWindow, CreateWindowExW, DefWindowProcW, RegisterClassExW,
};

use windows_sys::Win32::UI::Controls::HTHEME;
use windows_sys::Win32::UI::Controls::{CloseThemeData, OpenThemeData};

macro_rules! define_parts_and_state {
    ($(
        $part:ident($wname:ident) => {
            $($state:ident($wsname:ident)),*
        }
    ),*) => {
        /// The part of the theme to retrieve.
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Part {
            $($part($part)),*
        }

        $(
            #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[repr(i32)]
            pub enum $part {
                None = 0,
                $($state = windows_sys::Win32::UI::Controls::$wsname),*
            }
        )*

        impl Part {
            /// Get the Win32 state and part for this type.
            fn state_and_part(self) -> (i32, i32) {
                match self {
                    $(
                        Self::$part(x) => (
                            windows_sys::Win32::UI::Controls::$wname,
                            x as i32
                        )
                    ),*
                }
            }
        }
    };
}

define_parts_and_state! {
    Checkbox(BP_CHECKBOX) => {
        CheckedDisabled(CBS_CHECKEDDISABLED),
        CheckedHot(CBS_CHECKEDHOT),
        CheckedNormal(CBS_CHECKEDNORMAL),
        CheckedPressed(CBS_CHECKEDPRESSED),
        MixedDisabled(CBS_MIXEDDISABLED),
        MixedHot(CBS_MIXEDHOT),
        MixedNormal(CBS_MIXEDNORMAL),
        MixedPressed(CBS_MIXEDPRESSED),
        UncheckedDisabled(CBS_UNCHECKEDDISABLED),
        UncheckedHot(CBS_UNCHECKEDHOT),
        UncheckedNormal(CBS_UNCHECKEDNORMAL),
        UncheckedPressed(CBS_UNCHECKEDPRESSED)
    },

    CommandLink(BP_COMMANDLINK) => {
        Defaulted(CMDLS_DEFAULTED),
        DefaultedAnimating(CMDLS_DEFAULTED_ANIMATING),
        Disabled(CMDLS_DISABLED),
        Hot(CMDLS_HOT),
        Normal(CMDLS_NORMAL),
        Pressed(CMDLS_PRESSED)
    }
}

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
    pub unsafe fn with_window(hwnd: HWND, class_name: &str) -> Option<Self> {
        let mut name = class_name.encode_utf16().collect::<alloc::vec::Vec<_>>();
        name.push(0u16);

        let handle = OpenThemeData(hwnd, name.as_ptr());

        NonZeroIsize::new(handle).map(|handle| Self { handle })
    }

    /// Create a new theme from a class name.
    pub fn new(class_name: &str) -> Option<Self> {
        let window = DummyWindow::new();

        unsafe { Self::with_window(window.0, class_name) }
    }
}

macro_rules! get_theme {
    ($(
        $fname:ident => ($orig_ty:ty, $ret_ty:ty, $cvt:expr) {
            $($pname:ident($pid:ident)),*
        }
    ),*) => {
        impl Theme {
            $($(
                pub fn $pname(&self, part: Part) -> Option<$ret_ty> {
                    let mut return_value = MaybeUninit::<$orig_ty>::uninit();

                    // SAFETY: Calls a theme function, which is handled safely.
                    let (part, state) = part.state_and_part();
                    let result = unsafe {
                        windows_sys::Win32::UI::Controls::$fname(
                            self.handle(),
                            part,
                            state,
                            windows_sys::Win32::UI::Controls::$pid,
                            return_value.as_mut_ptr()
                        )
                    };

                    if result == windows_sys::Win32::Foundation::S_OK {
                        // SAFETY: return_value is now initialized
                        let return_value = unsafe {
                            return_value.assume_init()
                        };

                        let cvt = $cvt;
                        Some(cvt(return_value))
                    } else {
                        None
                    }
                }
            )*)*
        }
    };
}

get_theme! {
    GetThemeBool => (windows_sys::Win32::Foundation::BOOL, bool, |b| b != 0) {
        always_show_sizing_bar(TMT_ALWAYSSHOWSIZINGBAR),
        autosize(TMT_AUTOSIZE),
        bgfill(TMT_BGFILL),
        border_only(TMT_BORDERONLY),
        composited(TMT_COMPOSITED),
        composited_opaque(TMT_COMPOSITEDOPAQUE),
        draw_borders(TMT_DRAWBORDERS),
        flat_menus(TMT_FLATMENUS),
        glyph_only(TMT_GLYPHONLY),
        glyph_transparent(TMT_GLYPHTRANSPARENT),
        integral_sizing(TMT_INTEGRALSIZING),
        localized_mirror_image(TMT_LOCALIZEDMIRRORIMAGE),
        mirror_image(TMT_MIRRORIMAGE),
        no_etched_effect(TMT_NOETCHEDEFFECT),
        scaled_background(TMT_SCALEDBACKGROUND),
        source_grow(TMT_SOURCEGROW),
        source_shrink(TMT_SOURCESHRINK),
        text_apply_overlay(TMT_TEXTAPPLYOVERLAY),
        text_glow(TMT_TEXTGLOW),
        text_italic(TMT_TEXTITALIC),
        transparent(TMT_TRANSPARENT),
        uniform_sizing(TMT_UNIFORMSIZING),
        user_picture(TMT_USERPICTURE)
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
                core::ptr::null(),
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
