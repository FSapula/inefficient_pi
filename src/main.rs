mod phys;
fn main() {
    let mut box1 = phys::build_box(5.0, -2.0, 1.0, 1.0);
    let mut box2 = phys::build_box(10.0, 1.0, 1.0, 1.0);
    let boxes = phys::run_sim(&box1, &box2);
    box1 = boxes.0;
    box2 = boxes.1;
    println!(
        "box1: {}, box2: {}, time: {}",
        box1.position, box2.position, boxes.2
    );
}
