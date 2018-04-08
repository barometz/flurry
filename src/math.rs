//! Generic math helpers, mostly for the model. Much of this is re-exporting parts of the piston
//! graphics::math module.

pub use graphics::math::{add, orient, sub, Matrix2d, Vec2d};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_scalar_0() {
        assert_eq!([0, 0], mul_scalar([0, 0], 0));
        assert_eq!([0, 0], mul_scalar([1, 4], 0));
    }

    #[test]
    fn mul_scalar_1() {
        assert_eq!([1, 4], mul_scalar([1, 4], 1));
    }

    #[test]
    fn mul_scalar_values() {
        assert_eq!([10, 15], mul_scalar([2, 3], 5));
    }
}
