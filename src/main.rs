mod body;
mod utils;

use body::Body;
use utils::Vector2D;

fn main() {
    let body1 = Body::new(Vector2D::new(0.0, 0.0), Vector2D::new(1.0, 1.0), 1.0);
    let body2 = Body::new(Vector2D::new(1.0, 1.0), Vector2D::new(1.0, 1.0), 1.0);
    body1.print();
    body2.print();
    let distance = body1.distance(&body2);
    println!("Distance between body1 and body2: {distance}");
    println!("Velocity of body1: {}", body1.velocity.to_str());
    println!(
        "Normalized velocity of body1: {}",
        body1.velocity.normalize().to_str()
    );
}
