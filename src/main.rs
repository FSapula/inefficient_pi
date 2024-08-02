mod phys;
fn main() {
    let mut box1 = phys::build_box(1.0, 1.0, 1.0, 1.0);
    let mut box2 = phys::build_box(1.0, 1.0, 1.0, 1.0);
    println!("Hello, world!");
}
