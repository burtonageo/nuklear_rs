#![feature(alloc, heap_api)]

extern crate alloc;

pub mod sys;

pub struct DrawCommand {
    _priv: ()
}

pub struct DrawList {
    _priv: ()
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
    pub a: u8
}

impl From<sys::Struct_nk_color> for Color {
    fn from(raw_color: sys::Struct_nk_color) -> Self {
        Color {
            r: raw_color.r,
            g: raw_color.g,
            b: raw_color.b,
            a: raw_color.a
        }
    }
}

impl Into<sys::Struct_nk_color> for Color {
    fn into(self) -> sys::Struct_nk_color {
        sys::Struct_nk_color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl From<sys::Struct_nk_vec2> for Vec2 {
    fn from(raw_vec: sys::Struct_nk_vec2) -> Self {
        Vec2 {
            x: raw_vec.x,
            y: raw_vec.y
        }
    }
}

impl Into<sys::Struct_nk_vec2> for Vec2 {
    fn into(self) -> sys::Struct_nk_vec2 {
        sys::Struct_nk_vec2 {
            x: self.x,
            y: self.y
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Vec2i {
    pub x: i16,
    pub y: i16
}

impl From<sys::Struct_nk_vec2i> for Vec2i {
    fn from(raw_vec: sys::Struct_nk_vec2i) -> Self {
        Vec2i {
            x: raw_vec.x,
            y: raw_vec.y
        }
    }
}

impl Into<sys::Struct_nk_vec2i> for Vec2i {
    fn into(self) -> sys::Struct_nk_vec2i {
        sys::Struct_nk_vec2i {
            x: self.x,
            y: self.y
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

impl From<sys::Struct_nk_rect> for Rect {
    fn from(raw_rect: sys::Struct_nk_rect) -> Self {
        Rect {
            x: raw_rect.x,
            y: raw_rect.y,
            w: raw_rect.w,
            h: raw_rect.h
        }
    }
}

impl Into<sys::Struct_nk_rect> for Rect {
    fn into(self) -> sys::Struct_nk_rect {
        sys::Struct_nk_rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Recti {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16
}

impl From<sys::Struct_nk_recti> for Recti {
    fn from(raw_rect: sys::Struct_nk_recti) -> Self {
        Recti {
            x: raw_rect.x,
            y: raw_rect.y,
            w: raw_rect.w,
            h: raw_rect.h
        }
    }
}

impl Into<sys::Struct_nk_recti> for Recti {
    fn into(self) -> sys::Struct_nk_recti {
        sys::Struct_nk_recti {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Handle {
    Ptr(*mut std::os::raw::c_void),
    Id(i32)
}

impl Default for Handle {
    fn default() -> Self {
        Handle::Id(0)
    }
}

impl Into<sys::nk_handle> for Handle {
    fn into(self) -> sys::nk_handle {
        match self {
            Handle::Ptr(ptr) => unsafe { sys::nk_handle_ptr(ptr) },
            Handle::Id(id) => unsafe { sys::nk_handle_id(id) }
        }
    }
}

#[derive(Default)]
pub struct Image {
    pub handle: Handle,
    pub w: u16,
    pub h: u16,
    pub region: [u16; 4]
}

impl Into<sys::Struct_nk_image> for Image {
    fn into(self) -> sys::Struct_nk_image {
        sys::Struct_nk_image {
            handle: self.handle.into(),
            w: self.w,
            h: self.h,
            region: self.region
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Scroll {
    pub x: u16,
    pub y: u16
}

impl From<sys::Struct_nk_scroll> for Scroll {
    fn from(raw_vec: sys::Struct_nk_scroll) -> Self {
        Scroll {
            x: raw_vec.x,
            y: raw_vec.y
        }
    }
}

impl Into<sys::Struct_nk_scroll> for Scroll {
    fn into(self) -> sys::Struct_nk_scroll {
        sys::Struct_nk_scroll {
            x: self.x,
            y: self.y
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Heading {
    Up = 0,
    Down = 1,
    Right = 2,
    Left = 3
}

impl From<sys::Enum_nk_heading> for Heading {
    fn from(raw_heading: sys::Enum_nk_heading) -> Self {
        match raw_heading {
            sys::Enum_nk_heading::NK_UP => Heading::Up,
            sys::Enum_nk_heading::NK_RIGHT => Heading::Down,
            sys::Enum_nk_heading::NK_DOWN => Heading::Right,
            sys::Enum_nk_heading::NK_LEFT => Heading::Left
        }
    }
}

impl Into<sys::Enum_nk_heading> for Heading {
    fn into(self) -> sys::Enum_nk_heading {
        match self {
            Heading::Up => sys::Enum_nk_heading::NK_UP,
            Heading::Down => sys::Enum_nk_heading::NK_RIGHT,
            Heading::Right => sys::Enum_nk_heading::NK_DOWN,
            Heading::Left => sys::Enum_nk_heading::NK_LEFT
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonBehavior {
    Default = 0,
    Repeater = 1
}

impl From<sys::Enum_nk_button_behavior> for ButtonBehavior {
    fn from(raw_button_behavior: sys::Enum_nk_button_behavior) -> Self {
        match raw_button_behavior {
            sys::Enum_nk_button_behavior::NK_BUTTON_DEFAULT => ButtonBehavior::Default,
            sys::Enum_nk_button_behavior::NK_BUTTON_REPEATER => ButtonBehavior::Repeater
        }
    }
}

impl Into<sys::Enum_nk_button_behavior> for ButtonBehavior {
    fn into(self) -> sys::Enum_nk_button_behavior {
        match self {
            ButtonBehavior::Default => sys::Enum_nk_button_behavior::NK_BUTTON_DEFAULT,
            ButtonBehavior::Repeater => sys::Enum_nk_button_behavior::NK_BUTTON_REPEATER,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Modify {
    Fixed = 0,
    Modifiable = 1
}

impl From<sys::Enum_nk_modify> for Modify {
    fn from(raw_modify: sys::Enum_nk_modify) -> Self {
        match raw_modify {
            sys::Enum_nk_modify::NK_FIXED => Modify::Fixed,
            sys::Enum_nk_modify::NK_MODIFIABLE => Modify::Modifiable
        }
    }
}

impl Into<sys::Enum_nk_modify> for Modify {
    fn into(self) -> sys::Enum_nk_modify {
        match self {
            Modify::Fixed => sys::Enum_nk_modify::NK_FIXED,
            Modify::Modifiable => sys::Enum_nk_modify::NK_MODIFIABLE
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    Vertical = 0,
    Horizontal = 1
}

impl From<sys::Enum_nk_orientation> for Orientation {
    fn from(raw_modify: sys::Enum_nk_orientation) -> Self {
        match raw_modify {
            sys::Enum_nk_orientation::NK_VERTICAL => Orientation::Vertical,
            sys::Enum_nk_orientation::NK_HORIZONTAL => Orientation::Horizontal
        }
    }
}

impl Into<sys::Enum_nk_orientation> for Orientation {
    fn into(self) -> sys::Enum_nk_orientation {
        match self {
            Orientation::Vertical => sys::Enum_nk_orientation::NK_VERTICAL,
            Orientation::Horizontal => sys::Enum_nk_orientation::NK_HORIZONTAL
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollapseState {
    Minimized = 0,
    Maximized = 1
}

impl From<sys::Enum_nk_collapse_states> for CollapseState {
    fn from(raw_modify: sys::Enum_nk_collapse_states) -> Self {
        match raw_modify {
            sys::Enum_nk_collapse_states::NK_MINIMIZED => CollapseState::Minimized,
            sys::Enum_nk_collapse_states::NK_MAXIMIZED => CollapseState::Maximized
        }
    }
}

impl Into<sys::Enum_nk_collapse_states> for CollapseState {
    fn into(self) -> sys::Enum_nk_collapse_states {
        match self {
            CollapseState::Minimized => sys::Enum_nk_collapse_states::NK_MINIMIZED,
            CollapseState::Maximized => sys::Enum_nk_collapse_states::NK_MAXIMIZED
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShowState {
    Hidden = 0,
    Shown = 1
}

impl From<sys::Enum_nk_show_states> for ShowState {
    fn from(raw_show_state: sys::Enum_nk_show_states) -> Self {
        match raw_show_state {
            sys::Enum_nk_show_states::NK_HIDDEN => ShowState::Hidden,
            sys::Enum_nk_show_states::NK_SHOWN => ShowState::Shown
        }
    }
}

impl Into<sys::Enum_nk_show_states> for ShowState {
    fn into(self) -> sys::Enum_nk_show_states {
        match self {
            ShowState::Hidden => sys::Enum_nk_show_states::NK_HIDDEN,
            ShowState::Shown => sys::Enum_nk_show_states::NK_SHOWN
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChartType {
    Lines,
    Column,
    Max
}

impl From<sys::Enum_nk_chart_type> for ChartType {
    fn from(raw_chart_type: sys::Enum_nk_chart_type) -> Self {
        match raw_chart_type {
            sys::Enum_nk_chart_type::NK_CHART_LINES => ChartType::Lines,
            sys::Enum_nk_chart_type::NK_CHART_COLUMN => ChartType::Column,
            sys::Enum_nk_chart_type::NK_CHART_MAX => ChartType::Max
        }
    }
}

impl Into<sys::Enum_nk_chart_type> for ChartType {
    fn into(self) -> sys::Enum_nk_chart_type {
        match self {
            ChartType::Lines => sys::Enum_nk_chart_type::NK_CHART_LINES,
            ChartType::Column => sys::Enum_nk_chart_type::NK_CHART_COLUMN,
            ChartType::Max => sys::Enum_nk_chart_type::NK_CHART_MAX
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChartEvent {
    Hovering = 1,
    Clicked = 2
}

impl From<sys::Enum_nk_chart_event> for ChartEvent {
    fn from(raw_show_state: sys::Enum_nk_chart_event) -> Self {
        match raw_show_state {
            sys::Enum_nk_chart_event::NK_CHART_HOVERING => ChartEvent::Hovering,
            sys::Enum_nk_chart_event::NK_CHART_CLICKED => ChartEvent::Clicked
        }
    }
}

impl Into<sys::Enum_nk_chart_event> for ChartEvent {
    fn into(self) -> sys::Enum_nk_chart_event {
        match self {
            ChartEvent::Hovering => sys::Enum_nk_chart_event::NK_CHART_HOVERING,
            ChartEvent::Clicked => sys::Enum_nk_chart_event::NK_CHART_CLICKED
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorFormat {
    Rgb = 0,
    Rgba = 1
}

impl From<sys::Enum_nk_color_format> for ColorFormat {
    fn from(raw_color_format: sys::Enum_nk_color_format) -> Self {
        match raw_color_format {
            sys::Enum_nk_color_format::NK_RGB => ColorFormat::Rgb,
            sys::Enum_nk_color_format::NK_RGBA => ColorFormat::Rgba
        }
    }
}

impl Into<sys::Enum_nk_color_format> for ColorFormat {
    fn into(self) -> sys::Enum_nk_color_format {
        match self {
            ColorFormat::Rgb => sys::Enum_nk_color_format::NK_RGB,
            ColorFormat::Rgba => sys::Enum_nk_color_format::NK_RGBA
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PopupType {
    Dynamic = 0,
    Static = 1
}

impl From<sys::Enum_nk_popup_type> for PopupType {
    fn from(raw_popup_type: sys::Enum_nk_popup_type) -> Self {
        match raw_popup_type {
            sys::Enum_nk_popup_type::NK_POPUP_DYNAMIC => PopupType::Dynamic,
            sys::Enum_nk_popup_type::NK_POPUP_STATIC => PopupType::Static
        }
    }
}

impl Into<sys::Enum_nk_popup_type> for PopupType {
    fn into(self) -> sys::Enum_nk_popup_type {
        match self {
            PopupType::Dynamic => sys::Enum_nk_popup_type::NK_POPUP_DYNAMIC,
            PopupType::Static => sys::Enum_nk_popup_type::NK_POPUP_STATIC
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutFormat {
    Dynamic = 0,
    Static = 1
}

impl From<sys::Enum_nk_layout_format> for LayoutFormat {
    fn from(raw_layout_format: sys::Enum_nk_layout_format) -> Self {
        match raw_layout_format {
            sys::Enum_nk_layout_format::NK_DYNAMIC => LayoutFormat::Dynamic,
            sys::Enum_nk_layout_format::NK_STATIC => LayoutFormat::Static
        }
    }
}

impl Into<sys::Enum_nk_layout_format> for LayoutFormat {
    fn into(self) -> sys::Enum_nk_layout_format {
        match self {
            LayoutFormat::Dynamic => sys::Enum_nk_layout_format::NK_DYNAMIC,
            LayoutFormat::Static => sys::Enum_nk_layout_format::NK_STATIC
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TreeType {
    Node = 0,
    Tab = 1
}

impl From<sys::Enum_nk_tree_type> for TreeType {
    fn from(raw_layout_format: sys::Enum_nk_tree_type) -> Self {
        match raw_layout_format {
            sys::Enum_nk_tree_type::NK_TREE_NODE => TreeType::Node,
            sys::Enum_nk_tree_type::NK_TREE_TAB => TreeType::Tab
        }
    }
}

impl Into<sys::Enum_nk_tree_type> for TreeType {
    fn into(self) -> sys::Enum_nk_tree_type {
        match self {
            TreeType::Node => sys::Enum_nk_tree_type::NK_TREE_NODE,
            TreeType::Tab => sys::Enum_nk_tree_type::NK_TREE_TAB
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AntiAliasing {
    Off = 0,
    On = 1
}

impl From<sys::Enum_nk_anti_aliasing> for AntiAliasing {
    fn from(raw_layout_format: sys::Enum_nk_anti_aliasing) -> Self {
        match raw_layout_format {
            sys::Enum_nk_anti_aliasing::NK_ANTI_ALIASING_OFF => AntiAliasing::Off,
            sys::Enum_nk_anti_aliasing::NK_ANTI_ALIASING_ON => AntiAliasing::On
        }
    }
}

impl Into<sys::Enum_nk_anti_aliasing> for AntiAliasing {
    fn into(self) -> sys::Enum_nk_anti_aliasing {
        match self {
            AntiAliasing::Off => sys::Enum_nk_anti_aliasing::NK_ANTI_ALIASING_OFF,
            AntiAliasing::On => sys::Enum_nk_anti_aliasing::NK_ANTI_ALIASING_ON
        }
    }
}


fn rust_allocator() -> sys::Struct_nk_allocator {
    use alloc::heap;
    unsafe extern "C" fn allocate(mut data: sys::nk_handle,
                                  ptr: *mut std::os::raw::c_void,
                                  size: sys::nk_size)
                                  -> *mut std::os::raw::c_void {
        let alloc_data = data.ptr() as *mut usize;
        let allocation = heap::reallocate(ptr as *mut u8, *alloc_data as usize, size as usize, 4);
        *alloc_data = size as usize;
        allocation as *mut _
    }

    unsafe extern "C" fn free(mut data: sys::nk_handle, ptr: *mut std::os::raw::c_void) {
        let allocated = *(data.ptr() as *mut usize);
        heap::deallocate(ptr as *mut u8, allocated as usize, 4)
    }

    let bytes_allocated = Box::new(0usize);
    let data = Handle::Ptr(Box::into_raw(bytes_allocated) as *mut _).into();

    sys::Struct_nk_allocator {
        alloc: Some(allocate),
        free: Some(free),
        userdata: data
    }
}
