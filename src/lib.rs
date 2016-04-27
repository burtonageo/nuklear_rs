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

#[derive(Clone, Copy, Default, Eq, PartialEq)]
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

#[derive(Clone, Copy, Default, PartialEq)]
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

#[derive(Clone, Copy, Default, Eq, PartialEq)]
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

#[derive(Clone, Copy, Default, PartialEq)]
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

#[derive(Clone, Copy, Default, Eq, PartialEq)]
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

pub type Glyph = [::std::os::raw::c_char; 4];
pub enum Handle {
    Ptr(*mut u8),
    Id(i32)
}

impl From<sys::nk_handle> for Handle {
    fn from(raw_handle: nk_handle) -> Handle {
        unimplemented!()
    }
}

impl Into<sys::nk_handle> for Handle {
    fn into(self) -> Handle {
        unimplemented!()
    }
}

#[derive(Default)]
pub struct Image {
    pub handle: *mut Handle,
    pub w: u16,
    pub h: u16,
    pub region: [u16; 4]
}
