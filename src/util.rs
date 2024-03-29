use crate::Vec3;

pub fn dist_2d(a: &Vec3, b: &Vec3) -> f32 {
    ((b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y)).sqrt()
}
