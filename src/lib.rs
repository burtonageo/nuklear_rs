#![cfg_attr(feature = "use_bindgen_plugin", feature(plugin))]
#![cfg_attr(feature = "rust_allocator", feature(alloc, heap_api))]
#![feature(cstr_from_bytes)]

#[cfg(feature = "rust_allocator")]
extern crate alloc;
#[cfg(feature = "use_bindgen_plugin")]
#[macro_use]
#[plugin]
extern crate bindgen_plugin;
#[macro_use]
extern crate bitflags;

#[cfg(feature = "rust_allocator")]
mod rust_allocator;

#[cfg(feature = "rust_allocator")]
pub use rust_allocator::RustAllocator;

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::copy;
use std::sync::{Arc, Mutex, PoisonError};
use sys::*;

#[cfg(not(feature = "use_bindgen_plugin"))]
pub mod sys;

#[cfg(feature = "use_bindgen_plugin")]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(dead_code, uppercase_variables, non_camel_case_types, non_snake_case)]
pub mod sys {
    bindgen!("../nuklear/nuklear.h", match = "nuklear.h", link_static = "nuklear");
}

macro_rules! convertible_enum {
    ($(#[$top_lvl_attrs:meta])* pub enum $enum_nm:ident : $convert:ident {
        $($(#[$arm_attrs:meta])* $arm:ident => $other:ident),*
    }) => (
        $(#[$top_lvl_attrs])*
        pub enum $enum_nm {
            $($(#[$arm_attrs])* $arm),*
        }

        convertible_enum!(@_impl conversion from $enum_nm to $convert {
            $($arm => $other),*
        });
    );

    ($(#[$top_lvl_attrs:meta])* enum $enum_nm:ident : $convert:ident {
        $($(#[$arm_attrs:meta])* $arm:ident => $other:ident),*
    }) => (
        $(#[$top_lvl_attrs])*
        enum $enum_nm {
            $($(#[$arm_attrs])* $arm),*
        }

        convertible_enum!(@_impl conversion from $enum_nm to $convert {
            $($arm => $other),*
        });
    );

    (@_impl conversion from $enum_nm:ident to $convert:ident {
        $($arm:ident => $other:ident),*
    }) => (
        impl ::std::convert::Into<$convert> for $enum_nm {
            fn into(self) -> $convert {
                match self {
                    $($enum_nm::$arm => $convert::$other),*
                }
            }
        }

        impl ::std::convert::From<$convert> for $enum_nm {
            fn from(other: $convert) -> Self {
                match other {
                    $($convert::$other => $enum_nm::$arm),*
                }
            }
        }
    );
}

macro_rules! convertible_flags {
    ($(#[$top_level_attrs:meta])* pub flags $flags_nm:ident : $convert:ident = $int_ty:ty {
         $($(#[$arm_attrs:meta])* $arm:ident => $other:path),*
    }) => (
        bitflags! {
            $(#[$top_level_attrs])*
            pub flags $flags_nm: $int_ty {
                $($(#[$arm_attrs])* const $arm = $other as u32),*
            }
        }

        convertible_flags!(@_impl conversion from $flags_nm to $convert {
            $($arm => $other),*
        });
    );

    ($(#[top_level_attrs:meta])* flags $flags_nm:ident : $convert:ident = $int_ty:ty {
         $($(#[$arm_attrs:meta])* $arm:ident => $other:path),*
    }) => (
        bitflags! {
            $(#[$top_level_attrs])*
            flags $flags_nm: $int_ty {
                $($(#[arm_attrs])* const $arm = $other),*
            }
        }

        convertible_flags!(@_impl conversion from flags_nm to $convert {
            $($arm => $other),*
        });
    );

    (@_impl conversion from $flags_nm:ident to $convert:ident {
        $($arm:ident => $other:path),*
    }) => (
        impl ::std::convert::From<$convert> for $flags_nm {
            fn from(other: $convert) -> Self {
                $flags_nm::from_bits(other as u32).unwrap()
            }
        }

        impl ::std::convert::Into<$convert> for $flags_nm {
            fn into(self) -> $convert {
                match self {
                    $($arm => $other),*
                    , _ => unreachable!()
                }
            }
        }

        impl ::std::convert::Into<u32> for $flags_nm {
            fn into(self) -> u32 {
                self.bits()
            }
        }
    );
}

pub struct DrawCommand {
    _priv: (),
}

pub struct DrawList {
    _priv: (),
}

impl From<bool> for sys::Enum_Unnamed1 {
    fn from(b: bool) -> Self {
        if b {
            sys::Enum_Unnamed1::nk_true
        } else {
            sys::Enum_Unnamed1::nk_false
        }
    }
}

impl Into<bool> for sys::Enum_Unnamed1 {
    fn into(self) -> bool {
        match self {
            sys::Enum_Unnamed1::nk_true => true,
            sys::Enum_Unnamed1::nk_false => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(r: i32, g: i32, b: i32) -> Self {
        unsafe { sys::nk_rgb(r, g, b).into() }
    }

    pub fn rgb_f(r: f32, g: f32, b: f32) -> Self {
        unsafe { sys::nk_rgb_f(r, g, b).into() }
    }

    pub fn from_rgb_hex(hex: &str) -> Self {
        unsafe { sys::nk_rgb_hex(hex.as_ptr() as *const c_char).into() }
    }

    pub fn rgba(r: i32, g: i32, b: i32, a: i32) -> Self {
        unsafe { sys::nk_rgba(r, g, b, a).into() }
    }

    pub fn rgba_f(r: f32, g: f32, b: f32, a: f32) -> Self {
        unsafe { sys::nk_rgba_f(r, g, b, a).into() }
    }

    pub fn from_rgba_hex(hex: &str) -> Self {
        unsafe { sys::nk_rgba_hex(hex.as_ptr() as *const c_char).into() }
    }

    pub fn hsv(r: i32, g: i32, b: i32) -> Self {
        unsafe { sys::nk_hsv(r, g, b).into() }
    }

    pub fn hsv_f(r: f32, g: f32, b: f32) -> Self {
        unsafe { sys::nk_hsv_f(r, g, b).into() }
    }

    pub fn hsva(r: i32, g: i32, b: i32, a: i32) -> Self {
        unsafe { sys::nk_hsva(r, g, b, a).into() }
    }

    pub fn hsva_f(r: f32, g: f32, b: f32, a: f32) -> Self {
        unsafe { sys::nk_hsva_f(r, g, b, a).into() }
    }

    pub fn rgb_hex(&self) -> &str {
        let raw_col: sys::Struct_nk_color = (*self).into();
        let mut string: [c_char; 6] = [0; 6];
        unsafe {
            sys::nk_color_hex_rgb(string.as_mut_ptr(), raw_col);
            CStr::from_ptr(string.as_ptr()).to_str().unwrap()
        }
    }

    pub fn rgba_hex(&self) -> &str {
        let raw_col: sys::Struct_nk_color = (*self).into();
        let mut string: [c_char; 8] = [0; 8];
        unsafe {
            sys::nk_color_hex_rgba(string.as_mut_ptr(), raw_col);
            CStr::from_ptr(string.as_ptr()).to_str().unwrap()
        }
    }

    pub fn to_hsv(self) -> Color {
        let mut out_color = Color::default();
        unsafe { sys::nk_color_hsv_b(&mut out_color.r, &mut out_color.g, &mut out_color.b, self.into()) }
        out_color
    }

    pub fn to_hsva(self) -> Color {
        let mut out_color = Color::default();
        unsafe {
            sys::nk_color_hsva_b(&mut out_color.r, &mut out_color.g, &mut out_color.b, &mut out_color.a, self.into())
        }
        out_color
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        let raw_col: sys::Struct_nk_color = self.into();
        unsafe { sys::nk_color_u32(raw_col) as u32 }
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        let (mut r, mut g, mut b, mut a) = Default::default();
        unsafe { nk_color_f(&mut r, &mut g, &mut b, &mut a, self.into()) }
        [r, g, b, a]
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl From<sys::Struct_nk_color> for Color {
    fn from(raw_color: sys::Struct_nk_color) -> Self {
        Color {
            r: raw_color.r,
            g: raw_color.g,
            b: raw_color.b,
            a: raw_color.a,
        }
    }
}

impl Into<sys::Struct_nk_color> for Color {
    fn into(self) -> sys::Struct_nk_color {
        sys::Struct_nk_color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl From<sys::Struct_nk_vec2> for Vec2 {
    fn from(raw_vec: sys::Struct_nk_vec2) -> Self {
        Vec2 {
            x: raw_vec.x,
            y: raw_vec.y,
        }
    }
}

impl Into<sys::Struct_nk_vec2> for Vec2 {
    fn into(self) -> sys::Struct_nk_vec2 {
        sys::Struct_nk_vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Vec2i {
    pub x: i16,
    pub y: i16,
}

impl From<sys::Struct_nk_vec2i> for Vec2i {
    fn from(raw_vec: sys::Struct_nk_vec2i) -> Self {
        Vec2i {
            x: raw_vec.x,
            y: raw_vec.y,
        }
    }
}

impl Into<sys::Struct_nk_vec2i> for Vec2i {
    fn into(self) -> sys::Struct_nk_vec2i {
        sys::Struct_nk_vec2i {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl From<sys::Struct_nk_rect> for Rect {
    fn from(raw_rect: sys::Struct_nk_rect) -> Self {
        Rect {
            x: raw_rect.x,
            y: raw_rect.y,
            w: raw_rect.w,
            h: raw_rect.h,
        }
    }
}

impl Into<sys::Struct_nk_rect> for Rect {
    fn into(self) -> sys::Struct_nk_rect {
        sys::Struct_nk_rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Recti {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}

impl From<sys::Struct_nk_recti> for Recti {
    fn from(raw_rect: sys::Struct_nk_recti) -> Self {
        Recti {
            x: raw_rect.x,
            y: raw_rect.y,
            w: raw_rect.w,
            h: raw_rect.h,
        }
    }
}

impl Into<sys::Struct_nk_recti> for Recti {
    fn into(self) -> sys::Struct_nk_recti {
        sys::Struct_nk_recti {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Handle {
    Ptr(*mut c_void),
    Id(i32),
}

impl Default for Handle {
    fn default() -> Self {
        Handle::Id(0)
    }
}

impl From<*mut c_void> for Handle {
    fn from(ptr: *mut c_void) -> Self {
        Handle::Ptr(ptr)
    }
}

impl From<i32> for Handle {
    fn from(id: i32) -> Self {
        Handle::Id(id)
    }
}

impl Into<sys::nk_handle> for Handle {
    fn into(self) -> sys::nk_handle {
        match self {
            Handle::Ptr(ptr) => unsafe { sys::nk_handle_ptr(ptr) },
            Handle::Id(id) => unsafe { sys::nk_handle_id(id) },
        }
    }
}

#[cfg(test)]
mod handle_tests {
    use super::*;

    #[test]
    fn test_handle_ptr_conversion() {
        let arb_ptr = 12345 as *mut _;
        let handle = Handle::Ptr(arb_ptr);
        let mut raw_handle: sys::nk_handle = handle.into();
        unsafe { assert_eq!(arb_ptr, *raw_handle.ptr() as *mut _) };
    }

    #[test]
    fn test_handle_int_conversion() {
        let some_int = 19313i32;
        let handle = Handle::from(some_int);
        let mut raw_handle: sys::nk_handle = handle.into();
        unsafe { assert_eq!(some_int, *raw_handle.id()) }
    }
}

#[derive(Default)]
pub struct Image {
    pub handle: Handle,
    pub w: u16,
    pub h: u16,
    pub region: [u16; 4],
}

impl Into<sys::Struct_nk_image> for Image {
    fn into(self) -> sys::Struct_nk_image {
        sys::Struct_nk_image {
            handle: self.handle.into(),
            w: self.w,
            h: self.h,
            region: self.region,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Scroll {
    pub x: u16,
    pub y: u16,
}

impl From<sys::Struct_nk_scroll> for Scroll {
    fn from(raw_vec: sys::Struct_nk_scroll) -> Self {
        Scroll {
            x: raw_vec.x,
            y: raw_vec.y,
        }
    }
}

impl Into<sys::Struct_nk_scroll> for Scroll {
    fn into(self) -> sys::Struct_nk_scroll {
        sys::Struct_nk_scroll {
            x: self.x,
            y: self.y,
        }
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Heading: Enum_nk_heading {
        Up => NK_UP,
        Down => NK_RIGHT,
        Right => NK_DOWN,
        Left => NK_LEFT
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ButtonBehavior: Enum_nk_button_behavior {
        Default => NK_BUTTON_DEFAULT,
        Repeater => NK_BUTTON_REPEATER
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Modify: Enum_nk_modify {
        Fixed => NK_FIXED,
        Modifiable => NK_MODIFIABLE
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Orientation: Enum_nk_orientation {
        Vertical => NK_VERTICAL,
        Horizontal => NK_HORIZONTAL
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CollapseState: Enum_nk_collapse_states {
        Minimized => NK_MINIMIZED,
        Maximized => NK_MAXIMIZED
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ShowState: Enum_nk_show_states {
        Hidden => NK_HIDDEN,
        Shown => NK_SHOWN
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ChartType: Enum_nk_chart_type {
        Lines => NK_CHART_LINES,
        Column => NK_CHART_COLUMN,
        Max => NK_CHART_MAX
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ChartEvent: Enum_nk_chart_event {
        Hovering => NK_CHART_HOVERING,
        Clicked => NK_CHART_CLICKED
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ColorFormat: Enum_nk_color_format {
        Rgb => NK_RGB,
        Rgba => NK_RGBA
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum PopupType: Enum_nk_popup_type {
        Dynamic => NK_POPUP_DYNAMIC,
        Static => NK_POPUP_STATIC
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum LayoutFormat: Enum_nk_layout_format {
        Dynamic => NK_DYNAMIC,
        Static => NK_STATIC
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum TreeType: Enum_nk_tree_type {
        Node => NK_TREE_NODE,
        Tab => NK_TREE_TAB
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum AntiAliasing: Enum_nk_anti_aliasing {
        Off => NK_ANTI_ALIASING_OFF,
        On => NK_ANTI_ALIASING_ON
    }
}

pub trait Allocator {
    unsafe fn allocate(&mut self, size: usize) -> *mut c_void;
    unsafe fn reallocate(&mut self, old_pointer: *mut c_void, size: usize) -> *mut c_void;
    unsafe fn deallocate(&mut self, pointer: *mut c_void);
}


fn into_raw_allocator<A: Allocator>(allocator: &mut A) -> sys::Struct_nk_allocator {
    unsafe extern "C" fn allocate<A>(mut data: sys::nk_handle,
                                     old_pointer: *mut c_void,
                                     size: sys::nk_size) -> *mut c_void
                                     where A: Allocator {
        let allocator_ptr = (*data.ptr()) as *mut A;
        if old_pointer.is_null() {
            (*allocator_ptr).allocate(size as usize)
        } else {
            (*allocator_ptr).reallocate(old_pointer, size as usize)
        }
    }

    unsafe extern "C" fn deallocate<A: Allocator>(mut data: sys::nk_handle, ptr: *mut c_void) {
        let allocator_ptr = (*data.ptr()) as *mut A;
        (*allocator_ptr).deallocate(ptr)
    }

    let allocate_fn: unsafe extern fn(sys::nk_handle, *mut c_void, sys::nk_size) -> *mut c_void = allocate::<A>;
    let dealloc_fn: unsafe extern fn(sys::nk_handle, *mut c_void) = deallocate::<A>;
    let allocator_data: *mut c_void = (allocator as *mut A) as *mut _;

    sys::Struct_nk_allocator {
        alloc: Some(allocate_fn),
        free: Some(dealloc_fn),
        userdata: Handle::Ptr(allocator_data).into()
    }
}

#[derive(Debug, Default)]
pub struct DrawNullTexture {
    pub texture: Handle,
    pub uv: Vec2,
}

impl Into<sys::Struct_nk_draw_null_texture> for DrawNullTexture {
    fn into(self) -> sys::Struct_nk_draw_null_texture {
        sys::Struct_nk_draw_null_texture {
            texture: self.texture.into(),
            uv: self.uv.into(),
        }
    }
}

pub struct ConvertConfig {
    pub global_alpha: f32,
    pub line_aa: AntiAliasing,
    pub shape_aa: AntiAliasing,
    pub circle_segment_count: usize,
    pub arc_segment_count: usize,
    pub curve_segment_count: usize,
    pub null: DrawNullTexture,
}

impl Default for ConvertConfig {
    fn default() -> Self {
        ConvertConfig {
            global_alpha: 1.0,
            line_aa: AntiAliasing::Off,
            shape_aa: AntiAliasing::Off,
            circle_segment_count: 50,
            arc_segment_count: 50,
            curve_segment_count: 50,
            null: Default::default(),
        }
    }
}

impl Into<sys::Struct_nk_convert_config> for ConvertConfig {
    fn into(self) -> sys::Struct_nk_convert_config {
        use std::os::raw::{c_float, c_uint};
        sys::Struct_nk_convert_config {
            global_alpha: self.global_alpha as c_float,
            line_AA: self.line_aa.into(),
            shape_AA: self.shape_aa.into(),
            circle_segment_count: self.circle_segment_count as c_uint,
            arc_segment_count: self.arc_segment_count as c_uint,
            curve_segment_count: self.curve_segment_count as c_uint,
            null: self.null.into(),
        }
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum SymbolType: Enum_nk_symbol_type {
        None => NK_SYMBOL_NONE,
        X => NK_SYMBOL_X,
        Underscore => NK_SYMBOL_UNDERSCORE,
        Circle => NK_SYMBOL_CIRCLE,
        FilledCircle => NK_SYMBOL_CIRCLE_FILLED,
        Rect => NK_SYMBOL_RECT,
        FilledRect => NK_SYMBOL_RECT_FILLED,
        UpTriangle => NK_SYMBOL_TRIANGLE_UP,
        DownTriangle => NK_SYMBOL_TRIANGLE_DOWN,
        LeftTriangle => NK_SYMBOL_TRIANGLE_LEFT,
        RightTriangle => NK_SYMBOL_TRIANGLE_RIGHT,
        Plus => NK_SYMBOL_PLUS,
        Minus => NK_SYMBOL_MINUS,
        Max => NK_SYMBOL_MAX
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Key: Enum_nk_keys {
        None => NK_KEY_NONE,
        Shift => NK_KEY_SHIFT,
        Ctrl => NK_KEY_CTRL,
        Delete => NK_KEY_DEL,
        Enter => NK_KEY_ENTER,
        Tab => NK_KEY_TAB,
        Backspace => NK_KEY_BACKSPACE,
        CopyKey => NK_KEY_COPY,
        Cut => NK_KEY_CUT,
        Paste => NK_KEY_PASTE,
        Up => NK_KEY_UP,
        Down => NK_KEY_DOWN,
        Left => NK_KEY_LEFT,
        Right => NK_KEY_RIGHT,
        TextInsertMode => NK_KEY_TEXT_INSERT_MODE,
        TextLineStart => NK_KEY_TEXT_LINE_START,
        TextLineEnd => NK_KEY_TEXT_LINE_END,
        TextStart => NK_KEY_TEXT_START,
        TextEnd => NK_KEY_TEXT_END,
        TextUndo => NK_KEY_TEXT_UNDO,
        TextRedo => NK_KEY_TEXT_REDO,
        TextWordLeft => NK_KEY_TEXT_WORD_LEFT,
        TextWordRight => NK_KEY_TEXT_WORD_RIGHT,
        Max => NK_KEY_MAX
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Button: Enum_nk_buttons {
        Left => NK_BUTTON_LEFT,
        Middle => NK_BUTTON_MIDDLE,
        Right => NK_BUTTON_RIGHT,
        Max => NK_BUTTON_MAX
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum StyleColor: Enum_nk_style_colors {
        Text => NK_COLOR_TEXT,
        Window => NK_COLOR_WINDOW,
        Header => NK_COLOR_HEADER,
        Border => NK_COLOR_BORDER,
        Button => NK_COLOR_BUTTON,
        ButtonHover => NK_COLOR_BUTTON_HOVER,
        Active => NK_COLOR_BUTTON_ACTIVE,
        Toggle => NK_COLOR_TOGGLE,
        ToggleHover => NK_COLOR_TOGGLE_HOVER,
        Cursor => NK_COLOR_TOGGLE_CURSOR,
        Select => NK_COLOR_SELECT,
        SelectActive => NK_COLOR_SELECT_ACTIVE,
        Slider => NK_COLOR_SLIDER,
        SliderCursor=> NK_COLOR_SLIDER_CURSOR,
        SliderCursorHover => NK_COLOR_SLIDER_CURSOR_HOVER,
        SliderCursorActive => NK_COLOR_SLIDER_CURSOR_ACTIVE,
        ColorProperty => NK_COLOR_PROPERTY,
        Edit => NK_COLOR_EDIT,
        EditCursor => NK_COLOR_EDIT_CURSOR,
        Combo => NK_COLOR_COMBO,
        Chart => NK_COLOR_CHART,
        ChartColor => NK_COLOR_CHART_COLOR,
        ChartColorHighlight => NK_COLOR_CHART_COLOR_HIGHLIGHT,
        Scrollbar => NK_COLOR_SCROLLBAR,
        ScrollbarCursor => NK_COLOR_SCROLLBAR_CURSOR,
        ScrollbarCursorHover => NK_COLOR_SCROLLBAR_CURSOR_HOVER,
        ScrollbarCursorActive => NK_COLOR_SCROLLBAR_CURSOR_ACTIVE,
        TabHeader => NK_COLOR_TAB_HEADER,
        Count => NK_COLOR_COUNT
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum WidgetLayoutState: Enum_nk_widget_layout_states {
        Invalid => NK_WIDGET_INVALID,
        Valid => NK_WIDGET_VALID,
        Rom => NK_WIDGET_ROM
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum WidgetState: Enum_nk_widget_states {
        Inactive => NK_WIDGET_STATE_INACTIVE,
        Entered => NK_WIDGET_STATE_ENTERED,
        Hovered => NK_WIDGET_STATE_HOVERED,
        Left => NK_WIDGET_STATE_LEFT,
        Active => NK_WIDGET_STATE_ACTIVE
    }
}

convertible_flags! {
    pub flags TextAlign: Enum_nk_text_align = u32 {
        TEXT_ALIGN_LEFT => ::sys::Enum_nk_text_align::NK_TEXT_ALIGN_LEFT,
        TEXT_ALIGN_CENTERED => ::sys::Enum_nk_text_align::NK_TEXT_ALIGN_CENTERED,
        TEXT_ALIGN_RIGHT => ::sys::Enum_nk_text_align::NK_TEXT_ALIGN_RIGHT,
        TEXT_ALIGN_TOP => ::sys::Enum_nk_text_align::NK_TEXT_ALIGN_TOP,
        TEXT_ALIGN_MIDDLE => ::sys::Enum_nk_text_align::NK_TEXT_ALIGN_MIDDLE,
        TEXT_ALIGN_BOTTOM => ::sys::Enum_nk_text_align::NK_TEXT_ALIGN_BOTTOM
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum TextAlignment: Enum_nk_text_alignment {
        Left => NK_TEXT_LEFT,
        Centered => NK_TEXT_CENTERED,
        Right => NK_TEXT_RIGHT
    }
}

convertible_flags! {
    pub flags EditFlags: Enum_nk_edit_flags = u32 {
        EDIT_DEFAULT => ::sys::Enum_nk_edit_flags::NK_EDIT_DEFAULT,
        EDIT_READ_ONLY => ::sys::Enum_nk_edit_flags::NK_EDIT_READ_ONLY,
        EDIT_AUTO_SELECT => ::sys::Enum_nk_edit_flags::NK_EDIT_AUTO_SELECT,
        EDIT_SIG_ENTER => ::sys::Enum_nk_edit_flags::NK_EDIT_SIG_ENTER,
        EDIT_ALLOW_TAB => ::sys::Enum_nk_edit_flags::NK_EDIT_ALLOW_TAB,
        EDIT_NO_CURSOR => ::sys::Enum_nk_edit_flags::NK_EDIT_NO_CURSOR,
        EDIT_SELECTABLE => ::sys::Enum_nk_edit_flags::NK_EDIT_SELECTABLE,
        EDIT_CLIPBOARD => ::sys::Enum_nk_edit_flags::NK_EDIT_CLIPBOARD,
        EDIT_CTRL_ENTER_NEWLINE => ::sys::Enum_nk_edit_flags::NK_EDIT_CTRL_ENTER_NEWLINE,
        EDIT_NO_HORIZONTAL_SCROLL => ::sys::Enum_nk_edit_flags::NK_EDIT_NO_HORIZONTAL_SCROLL,
        EDIT_ALWAYS_INSERT_MODE => ::sys::Enum_nk_edit_flags::NK_EDIT_ALWAYS_INSERT_MODE,
        EDIT_MULTILINE => ::sys::Enum_nk_edit_flags::NK_EDIT_MULTILINE
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum EditTypes: Enum_nk_edit_types {
        Simple => NK_EDIT_SIMPLE,
        Field => NK_EDIT_FIELD,
        Box => NK_EDIT_BOX
    }
}

convertible_flags! {
    pub flags EditEvents: Enum_nk_edit_events = u32 {
        EDIT_ACTIVE => ::sys::Enum_nk_edit_events::NK_EDIT_ACTIVE,
        EDIT_INACTIVE => ::sys::Enum_nk_edit_events::NK_EDIT_INACTIVE,
        EDIT_ACTIVATED => ::sys::Enum_nk_edit_events::NK_EDIT_ACTIVATED,
        EDIT_DEACTIVATED => ::sys::Enum_nk_edit_events::NK_EDIT_DEACTIVATED,
        EDIT_COMMITTED => ::sys::Enum_nk_edit_events::NK_EDIT_COMMITED
    }
}

convertible_flags! {
    pub flags PanelFlags: Enum_nk_panel_flags = u32 {
        WINDOW_BORDER => ::sys::Enum_nk_panel_flags::NK_WINDOW_BORDER,
        WINDOW_BORDER_HEADER => ::sys::Enum_nk_panel_flags::NK_WINDOW_BORDER_HEADER,
        WINDOW_MOVEABLE => ::sys::Enum_nk_panel_flags::NK_WINDOW_MOVABLE,
        WINDOW_SCALABLE => ::sys::Enum_nk_panel_flags::NK_WINDOW_SCALABLE,
        WINDOW_CLOSEABLE => ::sys::Enum_nk_panel_flags::NK_WINDOW_CLOSABLE,
        WINDOW_MINIMIZABLE => ::sys::Enum_nk_panel_flags::NK_WINDOW_MINIMIZABLE,
        WINDOW_DYNAMIC => ::sys::Enum_nk_panel_flags::NK_WINDOW_DYNAMIC,
        WINDOW_NO_SCROLLBAR => ::sys::Enum_nk_panel_flags::NK_WINDOW_NO_SCROLLBAR,
        WINDOW_TITLE => ::sys::Enum_nk_panel_flags::NK_WINDOW_TITLE
    }
}

fn create_nk_string<A: Allocator>(allocator: &mut A, string: &str) -> sys::Struct_nk_str {
    let mut raw_alloc = into_raw_allocator(allocator);
    let mut raw_string = sys::Struct_nk_str::default();

    unsafe {
        sys::nk_str_init(&mut raw_string, &mut raw_alloc, string.len() as nk_size);
    }

    copy_to_nk_string(&mut raw_string, &string);
    raw_string
}

fn copy_to_nk_string(nk_string: &mut sys::Struct_nk_str, string: &str) {
    assert!(nk_string.len > string.len() as c_int);
    nk_string.len = string.len() as c_int;
    unsafe {
        copy(string.as_ptr(), nk_string.buffer.memory.ptr as *mut _, string.len());
    }
}

// TODO(burtonageo): Flesh this out
pub struct MemoryStatus;

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum AllocationType: Enum_nk_allocation_type {
        Fixed => NK_BUFFER_FIXED,
        Dynamic => NK_BUFFER_DYNAMIC
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum BufferAllocationType: Enum_nk_buffer_allocation_type {
        Front => NK_BUFFER_FRONT,
        Back => NK_BUFFER_BACK,
        Max => NK_BUFFER_MAX
    }
}

// TODO(burtonageo): Flesh this out
pub struct BufferMarker {
    active: i32,
    offset: usize
}

pub trait Clipboard {
    fn copy(&mut self, &str);
    fn get_paste_text(&self) -> &str;
}

fn into_raw_clipboard<C: Clipboard>(clipboard: &mut C) -> sys::Struct_nk_clipboard {
    unsafe extern "C" fn copy<C>(mut data: sys::nk_handle,
                                 chars: *const c_char,
                                 len: c_int)
                                 where C: Clipboard {
        use std::slice;
        let bytes = unsafe { slice::from_raw_parts(chars as *const u8, len as usize) };
        let text = CStr::from_bytes_with_nul(bytes).unwrap();
        let clipboard_ptr = (*data.ptr()) as *mut C;
        (*clipboard_ptr).copy(&text.to_string_lossy())
    }

    unsafe extern "C" fn paste<C: Clipboard>(mut data: sys::nk_handle, text_edit: *mut Struct_nk_text_edit) {
        let _clipboard_ptr = (*data.ptr()) as *mut C;
        unimplemented!();
    }

    let copy_fn: unsafe extern fn(sys::nk_handle, *const c_char, c_int) = copy::<C>;
    let paste_fn: unsafe extern fn(sys::nk_handle, *mut Struct_nk_text_edit) = paste::<C>;
    let clipboard_data: *mut c_void = (clipboard as *mut C) as *mut _;

    sys::Struct_nk_clipboard {
        userdata: Handle::Ptr(clipboard_data).into(),
        copy: Some(copy_fn),
        paste: Some(paste_fn)
    }
}

#[cfg(test)]
mod clipboard_tests {
    use super::*;
    use std::os::raw::c_int;

    #[derive(Default)]
    struct TestClipboard(String);

    impl Clipboard for TestClipboard {
        fn copy(&mut self, text: &str) {
            self.0 = text.to_string();
        }

        fn get_paste_text(&self) -> &str {
            unimplemented!();
        }
    }

    #[test]
    fn test_copy_clipboard() {
        const TEXT: &'static str = "Hello, world\0";
        let mut clip = TestClipboard::default();

        {
            let raw_clip = super::into_raw_clipboard(&mut clip);
            let (txt_ptr, txt_len) = (TEXT.as_ptr() as *const _, TEXT.len() as c_int);
            unsafe {
                (raw_clip.copy.unwrap())(raw_clip.userdata, txt_ptr, txt_len);
            }
        }

        // Nul byte isn't copied
        assert_eq!(&clip.0[..], &TEXT[..TEXT.len() - 1]);
    }
}

// TODO(burtonageo): ^^^^^
pub struct TextUndoRecord;

// TODO(burtonageo): ^^^^^
pub struct TextUndoState;

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum TextEditType: Enum_nk_text_edit_type {
        SingleLine => NK_TEXT_EDIT_SINGLE_LINE,
        MultiLine => NK_TEXT_EDIT_MULTI_LINE
    }
}

#[derive(Debug)]
pub struct TextEditError(TextEditErrorInner);

#[derive(Debug)]
enum TextEditErrorInner {
    ArcError,
    MutexPoisoned
}

impl TextEditError {
    fn arc_error() -> Self {
        TextEditError(TextEditErrorInner::ArcError)
    }
}

impl<Guard> From<PoisonError<Guard>> for TextEditError {
    fn from(error: PoisonError<Guard>) -> Self {
        TextEditError(TextEditErrorInner::MutexPoisoned)
    }
}

pub struct TextEdit<A: Allocator, C: Clipboard> {
    raw_edit: Struct_nk_text_edit,
    allocator: Arc<Mutex<A>>,
    clipboard: Arc<Mutex<C>>
}

impl<A: Allocator, C: Clipboard> Drop for TextEdit<A, C> {
    fn drop(&mut self) {
        unsafe {
            nk_textedit_free(&mut self.raw_edit);
        }
    }
}

impl<A: Allocator, C: Clipboard> TextEdit<A, C> {
    fn new<'a>(mut allocator: Arc<Mutex<A>>, clipboard: Arc<Mutex<C>>, initial_text: String)
               -> Result<Self, TextEditError> {
        let mut raw_edit = Struct_nk_text_edit::default();
        let mut raw_alloc = try!(Arc::get_mut(&mut allocator)
                                     .ok_or(TextEditError::arc_error())
                                     .and_then(|m| m.lock().map_err(From::from))
                                     .map(|mut a| into_raw_allocator(&mut *a)));

        unsafe {
            nk_textedit_init(&mut raw_edit, &mut raw_alloc, initial_text.len() as nk_size);
        }

        Ok(TextEdit {
            raw_edit: raw_edit,
            allocator: allocator,
            clipboard: clipboard,
        })
    }

    fn is_active(&self) -> bool {
        unimplemented!();
    }
}
