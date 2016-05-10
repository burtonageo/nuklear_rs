#![cfg_attr(feature = "rust_allocator", feature(alloc, heap_api))]

#[cfg(feature = "rust_allocator")]
extern crate alloc;
extern crate core;

pub mod sys;

#[cfg(feature = "rust_allocator")]
use alloc::heap;
use core::marker;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;
use sys::*;

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
    unsafe fn allocate(&mut self, old_pointer: *mut c_void, size: usize) -> *mut c_void;
    unsafe fn deallocate(&mut self, pointer: *mut c_void);
}

struct BindLifetime<'a, T> {
    data: T,
    marker: marker::PhantomData<&'a mut ()>
}

impl<'a, T> Deref for BindLifetime<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T> DerefMut for BindLifetime<'a, T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.data
    }
}

fn into_raw_allocator<A: Allocator>(allocator: &mut A) -> BindLifetime<sys::Struct_nk_allocator> {
    unsafe extern "C" fn allocate<A>(mut data: sys::nk_handle,
                                     old_pointer: *mut c_void,
                                     size: sys::nk_size) -> *mut c_void
        where A: Allocator {
        let allocator_ptr = data.ptr() as *mut A;
        (*allocator_ptr).allocate(old_pointer, size as usize)
    }

    unsafe extern "C" fn deallocate<A: Allocator>(mut data: sys::nk_handle, ptr: *mut c_void) {
        let allocator_ptr = data.ptr() as *mut A;
        (*allocator_ptr).deallocate(ptr)
    }

    let allocate_fn: unsafe extern fn(sys::nk_handle, *mut c_void, sys::nk_size) -> *mut c_void = allocate::<A>;
    let dealloc_fn: unsafe extern fn(sys::nk_handle, *mut c_void) = deallocate::<A>;
    let allocator_data: *mut c_void = (allocator as *mut A) as *mut _;

    let raw_alloc = sys::Struct_nk_allocator {
        alloc: Some(allocate_fn),
        free: Some(dealloc_fn),
        userdata: Handle::Ptr(allocator_data).into()
    };

    BindLifetime {
        data: raw_alloc,
        marker: marker::PhantomData
    }
}

#[cfg(feature = "rust_allocator")]
#[derive(Default)]
pub struct RustAllocator {
    /// Map of alloc locations to number of bytes allocated
    allocations: HashMap<*mut c_void, usize>
}

#[cfg(feature = "rust_allocator")]
const ALIGN: usize = 4;

#[cfg(feature = "rust_allocator")]
impl Allocator for RustAllocator {
    unsafe fn allocate(&mut self, old_pointer: *mut c_void, size: usize) -> *mut c_void {
        use std::ptr;
        let allocation = if old_pointer.is_null() || !self.allocations.contains_key(&old_pointer) {
            heap::allocate(size as usize, ALIGN) as *mut c_void
        } else {
            let old_alloced = self.allocations.remove(&old_pointer).unwrap();
            heap::reallocate(old_pointer as *mut u8, old_alloced, size, ALIGN) as *mut c_void
        };
        self.allocations.insert(allocation, size);
        allocation
    }

    unsafe fn deallocate(&mut self, pointer: *mut c_void) {
        if let Some(bytes_allocated) = self.allocations.remove(&pointer) {
            heap::deallocate(pointer as *mut u8, bytes_allocated as usize, ALIGN);
        }
    }
}

#[cfg(feature = "rust_allocator")]
fn rust_allocator() -> sys::Struct_nk_allocator {
    use alloc::heap;
    const ALIGN: usize = 4;
    unsafe extern "C" fn allocate(mut data: sys::nk_handle,
                                  ptr: *mut c_void,
                                  size: sys::nk_size)
                                  -> *mut c_void {
        let alloc_data = data.ptr() as *mut usize;
        let allocation = if *alloc_data == 0 {
            heap::allocate(size as usize, ALIGN)
        } else {
            heap::reallocate(ptr as *mut u8, *alloc_data as usize, size as usize, ALIGN)
        };
        *alloc_data = size as usize;
        allocation as *mut _
    }

    unsafe extern "C" fn free(mut data: sys::nk_handle, ptr: *mut c_void) {
        let allocated = *(data.ptr() as *mut usize);
        heap::deallocate(ptr as *mut u8, allocated as usize, ALIGN)
    }

    let bytes_allocated = Box::new(0usize);
    let data = Handle::from(Box::into_raw(bytes_allocated) as *mut _).into();

    sys::Struct_nk_allocator {
        alloc: Some(allocate),
        free: Some(free),
        userdata: data,
    }
}

#[test]
#[cfg(feature = "rust_allocator")]
fn test_rust_allocation() {
    use std::ptr;
    let mut allocator = RustAllocator::default();
    let alloced = unsafe {
        allocator.allocate(ptr::null_mut(), 20)
    };
    assert_eq!(*allocator.allocations.get(&alloced).unwrap(), 20);

    unsafe { allocator.deallocate(alloced) };
    assert!(allocator.allocations.get(&alloced).is_none());
}

// TODO: fix this
#[ignore]
#[test]
#[cfg(feature = "rust_allocator")]
fn test_raw_allocation() {
    use std::ptr;
    const USE_TRAIT: bool = true;
    let mut allocator = RustAllocator::default();
    let raw_alloc = if USE_TRAIT { *into_raw_allocator(&mut allocator) } else { rust_allocator() };
    let alloced = unsafe {
        (raw_alloc.alloc.unwrap())(raw_alloc.userdata, ptr::null_mut(), 32)
    };

    unsafe { (raw_alloc.free.unwrap())(raw_alloc.userdata, alloced) };
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

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum TextAlign: Enum_nk_text_align {
        Left => NK_TEXT_ALIGN_LEFT,
        Centered => NK_TEXT_ALIGN_CENTERED,
        Right => NK_TEXT_ALIGN_RIGHT,
        Top => NK_TEXT_ALIGN_TOP,
        Middle => NK_TEXT_ALIGN_MIDDLE,
        Bottom => NK_TEXT_ALIGN_BOTTOM
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

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum EditFlags: Enum_nk_edit_flags {
        Default => NK_EDIT_DEFAULT,
        ReadOnly => NK_EDIT_READ_ONLY,
        AutoSelect => NK_EDIT_AUTO_SELECT,
        SigEnter => NK_EDIT_SIG_ENTER,
        AllowTab => NK_EDIT_ALLOW_TAB,
        NoCursor => NK_EDIT_NO_CURSOR,
        Selectable => NK_EDIT_SELECTABLE,
        ClipBoard => NK_EDIT_CLIPBOARD,
        CtrlEnterNewline => NK_EDIT_CTRL_ENTER_NEWLINE,
        NoHorizontalScroll => NK_EDIT_NO_HORIZONTAL_SCROLL,
        AlwaysInsertMode => NK_EDIT_ALWAYS_INSERT_MODE,
        Multiline => NK_EDIT_MULTILINE
    }
}
