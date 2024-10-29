use body::Body;

mod body;

fn main() {
    let body1 = Body::new(0.0, 0.0, 1.0, 1.0, 1.0);
    let body2 = Body::new(1.0, 1.0, 1.0, 1.0, 1.0);
    body1.print();
    body2.print();
    let distance = body1.distance(&body2);
    println!("{distance}");
}
