use super::vector2::Vector2;

const G: f64 = 1.0;

/// Computes the gravitational force that a body with mass `m1` at position `pos1`
/// experiences due to the presence of another body with mass `m2` at position `pos2`.
pub fn gravitational_force(pos1: &Vector2, m1: &f64, pos2: &Vector2, m2: &f64) -> Vector2 {
    let direction = pos2 - pos1;
    let r: f64 = direction.magnitude();
    if r == 0.0 {
        // If the bodies overlap, do not apply any force.
        return Vector2::zero();
    }
    let r2: f64 = r * r;
    let magnitude: f64 = G * m1 * m2 / r2;
    direction.normalize() * magnitude
}

/// Computes the kinetic energy of a body with mass `m` and velocity `vel`.
pub fn kinetic_energy(m: &f64, vel: &Vector2) -> f64 {
    let v = vel.magnitude();
    let v2 = v * v;
    0.5 * m * v2
}

/// Computes the gravitational potential energy between a body of mass `m1` at position `pos1`
/// and a body with mass `m2` at position `pos2`.
pub fn gravitational_potential_energy(pos1: &Vector2, m1: &f64, pos2: &Vector2, m2: &f64) -> f64 {
    let r = (pos2 - pos1).magnitude();
    -G * m1 * m2 / r
}
