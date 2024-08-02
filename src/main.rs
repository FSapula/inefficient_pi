mod phys;
fn main() {
    let mut box1 = phys::build_box(5.0, -2.0, 1.0, 1.0);
    let mut box2 = phys::build_box(10.0, 1.0, 1.0, 1.0);
    let mut i: u32 = 0;
    while i < 10 {
        if phys::no_more_collisions(&box1, &box2) {
            println!("end");
            break;
        }
        let boxes = phys::run_sim(&box1, &box2);
        box1 = boxes.0;
        box2 = boxes.1;
        println!(
            "box1: {},box1 speed: {}, box2: {},box2 speed: {}, time: {}",
            box1.position, box1.velocity, box2.position, box2.velocity, boxes.2
        );
        i += 1;
    }
}
