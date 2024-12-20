#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VTInt32Point {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VTInt32Size {
    pub width: i32,
    pub height: i32,
}
