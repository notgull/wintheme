// SPDX-License-Identifier: BSL-1.0 OR Apache-2.0
//               Copyright John Nunley, 2023.
// Distributed under the Boost Software License, Version 1.0 or the Apache
//                 License, Version 2.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

//! A brief wrapper around Windows themes in Rust.
//!
//! To start, create a [`Theme`].

#![cfg(windows)]

extern crate alloc;

#[rustfmt::skip]
mod auto;

pub use auto::*;

use core::fmt;
use core::mem::MaybeUninit;
use core::num::NonZeroIsize;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, AtomicIsize, Ordering};

use alloc::vec::Vec;

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use windows_sys::w;

use windows_sys::Win32::Foundation::{COLORREF, HINSTANCE, HWND, POINT, RECT, SIZE, S_OK};

use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

use windows_sys::Win32::Graphics::Gdi::{HBITMAP, LOGFONTW};

use windows_sys::Win32::UI::WindowsAndMessaging::WNDCLASSEXW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CloseWindow, CreateWindowExW, DefWindowProcW, RegisterClassExW,
};

use windows_sys::Win32::UI::Controls::{
    CloseThemeData, GetThemeBitmap, GetThemeBool, GetThemeColor, GetThemeEnumValue,
    GetThemeFilename, GetThemeFont, GetThemeInt, GetThemeIntList, GetThemeMargins,
    GetThemePosition, GetThemeRect, GetThemeStream, GetThemeString, GetThemeSysBool,
    GetThemeSysString, OpenThemeData,
};
use windows_sys::Win32::UI::Controls::{HTHEME, MARGINS};

/// A color.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Color(COLORREF);

impl Color {
    /// Create a new color.
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self((red as u32) | ((green as u32) << 8) | ((blue as u32) << 16))
    }

    /// Get the red component.
    pub fn red(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Get the green component.
    pub fn green(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Get the blue component.
    pub fn blue(&self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    /// Get the raw color.
    pub fn raw(&self) -> COLORREF {
        self.0
    }

    /// Create from a raw color.
    pub const fn from_raw(raw: COLORREF) -> Self {
        Self(raw)
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Color")
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .finish()
    }
}

/// A font object.
#[derive(Clone, Copy)]
pub struct LogicalFont(LOGFONTW);

impl LogicalFont {
    /// Get the height of the font.
    pub fn height(&self) -> i32 {
        self.0.lfHeight
    }

    /// Get the width of the font.
    pub fn width(&self) -> i32 {
        self.0.lfWidth
    }

    /// Get the escapement of the font.
    pub fn escapement(&self) -> i32 {
        self.0.lfEscapement
    }

    /// Get the orientation of the font.
    pub fn orientation(&self) -> i32 {
        self.0.lfOrientation
    }

    /// Get the weight of the font.
    pub fn weight(&self) -> i32 {
        self.0.lfWeight
    }

    /// Get if the font is italic.
    pub fn italic(&self) -> bool {
        self.0.lfItalic != 0
    }

    /// Get if the font is underlined.
    pub fn underline(&self) -> bool {
        self.0.lfUnderline != 0
    }

    /// Get if the font is struck out.
    pub fn strike_out(&self) -> bool {
        self.0.lfStrikeOut != 0
    }

    /// Get the character set of the font.
    pub fn char_set(&self) -> u8 {
        self.0.lfCharSet
    }

    /// Get the output precision of the font.
    pub fn output_precision(&self) -> OutputPrecision {
        use windows_sys::Win32::Graphics::Gdi::OUT_DEFAULT_PRECIS;

        match self.0.lfOutPrecision {
            OUT_DEFAULT_PRECIS => OutputPrecision::Default,
            _ => OutputPrecision::Unknown(self.0.lfOutPrecision),
        }
    }

    /// Get the clip precision of the font.
    pub fn clip_precision(&self) -> ClipPrecision {
        use windows_sys::Win32::Graphics::Gdi::CLIP_DEFAULT_PRECIS;

        match self.0.lfClipPrecision {
            CLIP_DEFAULT_PRECIS => ClipPrecision::Default,
            _ => ClipPrecision::Unknown(self.0.lfClipPrecision),
        }
    }

    /// Get the output quality of the font.
    pub fn output_quality(&self) -> OutputQuality {
        use windows_sys::Win32::Graphics::Gdi::{
            ANTIALIASED_QUALITY, CLEARTYPE_QUALITY, DEFAULT_QUALITY, DRAFT_QUALITY,
        };

        match self.0.lfQuality {
            ANTIALIASED_QUALITY => OutputQuality::AntiAliased,
            CLEARTYPE_QUALITY => OutputQuality::ClearType,
            DEFAULT_QUALITY => OutputQuality::Default,
            DRAFT_QUALITY => OutputQuality::Draft,
            _ => panic!("Unknown output quality"),
        }
    }

    /// The typeface name of the font.
    pub fn typeface_name(&self) -> OsString {
        let len = self.0.lfFaceName.iter().position(|&c| c == 0).unwrap();
        OsString::from_wide(&self.0.lfFaceName[..len])
    }
}

impl fmt::Debug for LogicalFont {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LogicalFont")
            .field("height", &self.height())
            .field("width", &self.width())
            .field("escapement", &self.escapement())
            .field("orientation", &self.orientation())
            .field("weight", &self.weight())
            .field("italic", &self.italic())
            .field("underline", &self.underline())
            .field("strike_out", &self.strike_out())
            .field("char_set", &self.char_set())
            .field("output_precision", &self.output_precision())
            .field("clip_precision", &self.clip_precision())
            .field("output_quality", &self.output_quality())
            .field("typeface_name", &self.typeface_name())
            .finish()
    }
}

/// The output precision of the font.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum OutputPrecision {
    /// The default output precision.
    Default,

    /// Unknown.
    Unknown(u8),
}

/// The clip precision of the font.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ClipPrecision {
    /// The default clip precision.
    Default,

    /// Unknown.
    Unknown(u8),
}

/// The output quality.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum OutputQuality {
    /// Quality is anti-aliased.
    AntiAliased,

    /// Quality is clear type.
    ClearType,

    /// Quality is default.
    Default,

    /// Quality is draft.
    Draft,

    /// Quality is proof.
    Proof,

    /// Quality is non-anti-aliased.
    NonAntiAliased,
}

/// A handle to a bitmap.
pub struct Bitmap {
    /// The bitmap handle.
    ///
    /// This cannot be zero, so we use `NonZeroIsize` to ensure that.
    /// Semantically, this is an `HBITMAP`.
    handle: NonZeroIsize,
}

impl fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct PtrWrapper(HBITMAP);

        impl fmt::Debug for PtrWrapper {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Pointer::fmt(&(self.0 as *const ()), f)
            }
        }

        f.debug_tuple("Bitmap")
            .field(&PtrWrapper(self.handle() as _))
            .finish()
    }
}

impl Bitmap {
    /// Get the handle to the bitmap.
    pub fn handle(&self) -> HBITMAP {
        self.handle.get() as _
    }
}

/// The margins of a window.
#[derive(Clone, Copy)]
pub struct Margins(MARGINS);

impl Margins {
    /// Get the left margin.
    pub fn left(&self) -> i32 {
        self.0.cxLeftWidth
    }

    /// Get the right margin.
    pub fn right(&self) -> i32 {
        self.0.cxRightWidth
    }

    /// Get the top margin.
    pub fn top(&self) -> i32 {
        self.0.cyTopHeight
    }

    /// Get the bottom margin.
    pub fn bottom(&self) -> i32 {
        self.0.cyBottomHeight
    }
}

impl fmt::Debug for Margins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Margins")
            .field("left", &self.left())
            .field("right", &self.right())
            .field("top", &self.top())
            .field("bottom", &self.bottom())
            .finish()
    }
}

/// A point.
#[derive(Clone, Copy)]
pub struct Point(POINT);

impl Point {
    /// Get the x-coordinate.
    pub fn x(&self) -> i32 {
        self.0.x
    }

    /// Get the y-coordinate.
    pub fn y(&self) -> i32 {
        self.0.y
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}

/// A rectangle.
#[derive(Clone, Copy)]
pub struct Rect(RECT);

impl Rect {
    /// Get the left coordinate.
    pub fn left(&self) -> i32 {
        self.0.left
    }

    /// Get the top coordinate.
    pub fn top(&self) -> i32 {
        self.0.top
    }

    /// Get the right coordinate.
    pub fn right(&self) -> i32 {
        self.0.right
    }

    /// Get the bottom coordinate.
    pub fn bottom(&self) -> i32 {
        self.0.bottom
    }
}

impl fmt::Debug for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rect")
            .field("left", &self.left())
            .field("top", &self.top())
            .field("right", &self.right())
            .field("bottom", &self.bottom())
            .finish()
    }
}

/// The size of a window.
#[derive(Clone, Copy)]
pub struct Size(SIZE);

impl Size {
    /// Get the width.
    pub fn width(&self) -> i32 {
        self.0.cx
    }

    /// Get the height.
    pub fn height(&self) -> i32 {
        self.0.cy
    }
}

impl fmt::Debug for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Size")
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
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

type GetThemeBoolRet = Option<bool>;
type GetThemeColorRet = Option<Color>;
type GetThemeStreamRet = Option<NonNull<[u8]>>;
type GetThemeEnumValueRet = Option<i32>;
type GetThemeFilenameRet = Option<PathBuf>;
type GetThemeFontRet = Option<LogicalFont>;
type GetThemeBitmapRet = Option<Bitmap>;
type GetThemeIntRet = Option<i32>;
type GetThemeIntListRet = Option<Vec<i32>>;
type GetThemeMarginsRet = Option<Margins>;
type GetThemePositionRet = Option<Point>;
type GetThemeRectRet = Option<Rect>;
type GetThemeStringRet = Option<OsString>;

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

    /// Get a theme boolean.
    fn get_theme_bool(&self, part: Part, prop_id: u32) -> Option<bool> {
        use windows_sys::Win32::Foundation::BOOL;
        let mut return_value = MaybeUninit::<BOOL>::uninit();

        // SAFETY: `handle` is a valid `HTHEME`, `part` is a valid `Part`.
        let (part, state) = part.part_and_state();
        let result = unsafe {
            GetThemeBool(
                self.handle(),
                part,
                state,
                prop_id,
                return_value.as_mut_ptr(),
            )
        };

        if result == S_OK {
            // SAFETY: `GetThemeBool` succeeded, so `return_value` is
            // initialized.
            let return_value = unsafe { return_value.assume_init() };
            Some(return_value != 0)
        } else {
            None
        }
    }

    /// Get a theme's system boolean.
    fn get_theme_sys_bool(&self, prop_id: u32) -> Option<bool> {
        // SAFETY: `handle` is a valid `HTHEME`.
        let result = unsafe { GetThemeSysBool(self.handle(), prop_id) };

        if result == S_OK {
            Some(result != 0)
        } else {
            None
        }
    }

    /// Get a theme color.
    fn get_theme_color(&self, part: Part, prop_id: u32) -> Option<Color> {
        let mut return_value = MaybeUninit::<COLORREF>::uninit();

        // SAFETY: `handle` is a valid `HTHEME`, `part` is a valid `Part`.
        let (part, state) = part.part_and_state();
        let result = unsafe {
            GetThemeColor(
                self.handle(),
                part,
                state,
                prop_id,
                return_value.as_mut_ptr(),
            )
        };

        if result == S_OK {
            // SAFETY: `GetThemeColor` succeeded, so `return_value` is
            // initialized.
            let return_value = unsafe { return_value.assume_init() };
            Some(Color::from_raw(return_value))
        } else {
            None
        }
    }

    /// Get a theme stream.
    fn get_theme_stream(&self, part: Part, prop_id: u32) -> Option<NonNull<[u8]>> {
        let mut return_ptr = MaybeUninit::<*mut u8>::uninit();
        let mut return_len = MaybeUninit::<u32>::uninit();

        // SAFETY: `handle` is a valid `HTHEME`, `part` is a valid `Part`.
        let (part, state) = part.part_and_state();
        let result = unsafe {
            GetThemeStream(
                self.handle(),
                part,
                state,
                prop_id as _,
                return_ptr.as_mut_ptr() as _,
                return_len.as_mut_ptr(),
                0,
            )
        };

        if result == S_OK {
            let (return_ptr, return_len) = unsafe {
                // SAFETY: `GetThemeStream` succeeded, so `return_ptr` and
                // `return_len` are initialized.
                (return_ptr.assume_init(), return_len.assume_init())
            };

            // SAFETY: `return_ptr` is a valid pointer to `return_len` bytes.
            NonNull::new(return_ptr).map(|ptr| unsafe {
                NonNull::new_unchecked(core::ptr::slice_from_raw_parts(
                    ptr.as_ptr(),
                    return_len as usize,
                ) as _)
            })
        } else {
            None
        }
    }

    /// Gets a theme enum value.
    fn get_theme_enum_value(&self, part: Part, prop_id: u32) -> Option<i32> {
        let mut return_value = MaybeUninit::<i32>::uninit();

        // SAFETY: `handle` is a valid `HTHEME`, `part` is a valid `Part`.
        let (part, state) = part.part_and_state();
        let result = unsafe {
            GetThemeEnumValue(
                self.handle(),
                part,
                state,
                prop_id,
                return_value.as_mut_ptr(),
            )
        };

        if result == S_OK {
            // SAFETY: `GetThemeEnumValue` succeeded, so `return_value` is
            // initialized.
            let return_value = unsafe { return_value.assume_init() };
            Some(return_value)
        } else {
            None
        }
    }

    /// Get a theme filename.
    fn get_theme_filename(&self, part: Part, prop_id: u32) -> Option<PathBuf> {
        // Should be long enough in most cases.
        const MAX_FILENAME_LENGTH: usize = 256;
        let mut return_value = [0u16; MAX_FILENAME_LENGTH];

        // SAFETY: `handle` is a valid `HTHEME`, `part` is a valid `Part`.
        let (part, state) = part.part_and_state();
        let result = unsafe {
            GetThemeFilename(
                self.handle(),
                part,
                state,
                prop_id,
                return_value.as_mut_ptr() as _,
                MAX_FILENAME_LENGTH as _,
            )
        };

        if result == S_OK {
            let filename = {
                let len = return_value
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(MAX_FILENAME_LENGTH);

                OsString::from_wide(&return_value[..len])
            };

            Some(filename.into())
        } else {
            None
        }
    }

    /// Get a theme font.
    fn get_theme_font(&self, part: Part, prop_id: u32) -> Option<LogicalFont> {
        let mut return_value = MaybeUninit::<LOGFONTW>::uninit();

        // SAFETY: `handle` is a valid `HTHEME`, `part` is a valid `Part`.
        let (part, state) = part.part_and_state();
        let result = unsafe {
            GetThemeFont(
                self.handle(),
                0,
                part,
                state,
                prop_id as _,
                return_value.as_mut_ptr(),
            )
        };

        if result == S_OK {
            // SAFETY: `GetThemeFont` succeeded, so `return_value` is
            // initialized.
            let return_value = unsafe { return_value.assume_init() };
            Some(LogicalFont(return_value))
        } else {
            None
        }
    }

    /// Get a theme bitmap.
    fn get_theme_bitmap(&self, part: Part, prop_id: u32) -> Option<Bitmap> {
        let mut return_value = MaybeUninit::<HBITMAP>::uninit();

        // SAFETY: `handle` is a valid `HTHEME`, `part` is a valid `Part`.
        let (part, state) = part.part_and_state();
        let result = unsafe {
            GetThemeBitmap(
                self.handle(),
                part,
                state,
                prop_id,
                0,
                return_value.as_mut_ptr(),
            )
        };

        if result == S_OK {
            // SAFETY: `GetThemeBitmap` succeeded, so `return_value` is
            // initialized.
            let return_value = unsafe { return_value.assume_init() };
            NonZeroIsize::new(return_value).map(|handle| Bitmap { handle })
        } else {
            None
        }
    }

    /// Get a theme int.
    fn get_theme_int(&self, part: Part, prop_id: u32) -> Option<i32> {
        let mut return_value = MaybeUninit::<i32>::uninit();

        // SAFETY: `handle` is a valid `HTHEME`, `part` is a valid `Part`.
        let (part, state) = part.part_and_state();
        let result = unsafe {
            GetThemeInt(
                self.handle(),
                part,
                state,
                prop_id,
                return_value.as_mut_ptr(),
            )
        };

        if result == S_OK {
            // SAFETY: `GetThemeInt` succeeded, so `return_value` is
            // initialized.
            let return_value = unsafe { return_value.assume_init() };
            Some(return_value)
        } else {
            None
        }
    }

    /// Get ta theme int list.
    fn get_theme_int_list(&self, part: Part, prop_id: u32) -> Option<Vec<i32>> {
        use windows_sys::Win32::UI::Controls::INTLIST;

        let mut return_list = MaybeUninit::<INTLIST>::uninit();

        // SAFETY: `handle` is a valid `HTHEME`, `part` is a valid `Part`.
        let (part, state) = part.part_and_state();
        let result = unsafe {
            GetThemeIntList(
                self.handle(),
                part,
                state,
                prop_id,
                return_list.as_mut_ptr(),
            )
        };

        if result == S_OK {
            // SAFETY: `GetThemeIntList` succeeded
            let return_list = unsafe { return_list.assume_init() };
            let ints = &return_list.iValues[..return_list.iValueCount as _];
            Some(ints.to_vec())
        } else {
            None
        }
    }

    /// Get the margins for a theme.
    fn get_theme_margins(&self, part: Part, prop_id: u32) -> Option<Margins> {
        let mut return_value = MaybeUninit::<MARGINS>::uninit();

        // SAFETY: `handle` is a valid `HTHEME`, `part` is a valid `Part`.
        let (part, state) = part.part_and_state();
        let result = unsafe {
            GetThemeMargins(
                self.handle(),
                0,
                part,
                state,
                prop_id,
                std::ptr::null(),
                return_value.as_mut_ptr(),
            )
        };

        if result == S_OK {
            // SAFETY: `GetThemeMargins` succeeded, so `return_value` is
            // initialized.
            let return_value = unsafe { return_value.assume_init() };
            Some(Margins(return_value))
        } else {
            None
        }
    }

    /// Get the position of a theme part.
    fn get_theme_position(&self, part: Part, prop_id: u32) -> Option<Point> {
        let mut return_value = MaybeUninit::<POINT>::uninit();

        // SAFETY: `handle` is a valid `HTHEME`, `part` is a valid `Part`.
        let (part, state) = part.part_and_state();
        let result = unsafe {
            GetThemePosition(
                self.handle(),
                part,
                state,
                prop_id,
                return_value.as_mut_ptr(),
            )
        };

        if result == S_OK {
            // SAFETY: `GetThemePosition` succeeded, so `return_value` is
            // initialized.
            let return_value = unsafe { return_value.assume_init() };
            Some(Point(return_value))
        } else {
            None
        }
    }

    /// Get the rectangle of a theme part.
    fn get_theme_rect(&self, part: Part, prop_id: u32) -> Option<Rect> {
        let mut return_value = MaybeUninit::<RECT>::uninit();

        // SAFETY: `handle` is a valid `HTHEME`, `part` is a valid `Part`.
        let (part, state) = part.part_and_state();
        let result = unsafe {
            GetThemeRect(
                self.handle(),
                part,
                state,
                prop_id as _,
                return_value.as_mut_ptr(),
            )
        };

        if result == S_OK {
            // SAFETY: `GetThemeRect` succeeded, so `return_value` is
            // initialized.
            let return_value = unsafe { return_value.assume_init() };
            Some(Rect(return_value))
        } else {
            None
        }
    }

    /// Get a theme string.
    fn get_theme_string(&self, part: Part, prop_id: u32) -> Option<OsString> {
        const MAX_STRING_LENGTH: usize = 1024;
        let mut return_value = MaybeUninit::<[u16; MAX_STRING_LENGTH]>::uninit();

        // SAFETY: `handle` is a valid `HTHEME`, `part` is a valid `Part`.
        let (part, state) = part.part_and_state();
        let result = unsafe {
            GetThemeString(
                self.handle(),
                part,
                state,
                prop_id as _,
                return_value.as_mut_ptr() as _,
                MAX_STRING_LENGTH as _,
            )
        };

        if result == S_OK {
            // SAFETY: `GetThemeString` succeeded, so `return_value` is
            // initialized.
            let return_value = unsafe { return_value.assume_init() };
            let slice = {
                let len = return_value
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(MAX_STRING_LENGTH);

                &return_value[..len]
            };

            Some(OsString::from_wide(slice))
        } else {
            None
        }
    }

    /// Get a theme system string.
    fn get_theme_sys_string(&self, prop_id: u32) -> Option<OsString> {
        const MAX_STRING_LENGTH: usize = 1024;
        let mut return_value = MaybeUninit::<[u16; MAX_STRING_LENGTH]>::uninit();

        // SAFETY: `handle` is a valid `HTHEME`.
        let result = unsafe {
            GetThemeSysString(
                self.handle(),
                prop_id as _,
                return_value.as_mut_ptr() as _,
                MAX_STRING_LENGTH as _,
            )
        };

        if result == S_OK {
            // SAFETY: `GetThemeSysString` succeeded, so `return_value` is
            // initialized.
            let return_value = unsafe { return_value.assume_init() };
            let slice = {
                let len = return_value
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(MAX_STRING_LENGTH);

                &return_value[..len]
            };

            Some(OsString::from_wide(slice))
        } else {
            None
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
    INSTANCE
        .compare_exchange(instance, handle, Ordering::SeqCst, Ordering::SeqCst)
        .unwrap_or_else(|x| x)
}
