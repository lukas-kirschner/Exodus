#[derive(Copy, Clone, PartialEq)]
pub struct Movement {
    pub velocity: (f32, f32),
    pub target: (i32, i32),
}