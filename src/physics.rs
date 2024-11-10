use crate::vectors::Vector2;

const G: f64 = 1.0;

/// Computes the gravitational force that a body with mass `mass1` at position `pos1`
/// experiences due to the presence of another body with mass `mass2` at position `pos2`.
pub fn gravitational_force(pos1: &Vector2, mass1: &f64, pos2: &Vector2, mass2: &f64) -> Vector2 {
    let direction = pos2 - pos1;
    let r: f64 = direction.magnitude();
    if r == 0.0 {
        // If the bodies overlap, do not apply any force.
        return Vector2::zero();
    }
    let r2: f64 = r * r;
    let magnitude: f64 = G * mass1 * mass2 / r2;
    direction.normalize() * magnitude
}
