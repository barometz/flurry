// Generic math helpers, mostly for the model. Much of this is re-exporting parts of the piston
// graphics::math module.

pub use graphics::math::{Matrix2d,Vec2d,orient,sub,add};
use std::ops::Mul;
use uom::si::f64::*;
use vecmath::vec2_normalized;

pub fn normalized(v: Vec2d<Length>) -> Vec2d<f64> {
    let v: Vec2d = [v[0].value, v[1].value];
    if v[0] == 0.0 && v[1] == 0.0 {
        v
    } else {
        vec2_normalized(v)
    }
}

pub fn mul_scalar<T, U, V>(t: Vec2d<T>, u: U) -> Vec2d<V>
    where
        T: Copy + Mul<U, Output = V>,
        U: Copy,
{
    [t[0] * u, t[1] * u]
}
