#[derive(Copy, Clone, PartialEq)]
pub struct Movement {
    pub velocity: (f32, f32),
    pub target: (f32, f32),
}