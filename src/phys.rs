struct physBox {
    position: f64,
    velocity: f64,
    mass: f64,
    width: f64,
}
fn build_box(position: f64, velocity: f64, mass: f64, width: f64) -> physBox {
    physBox {
        position,
        velocity,
        mass,
        width,
    }
}
fn wall_first(time1: f64, time2: f64) -> bool {
    if time1 < 0.0 {
        return false;
    }
    if time2 < 0.0 || time1 < time2 {
        return true;
    }
    false
}
fn predict_collision(ob1: &physBox, ob2: &physBox) -> (f64, f64) {
    let time_to_wall = -1.0 * (ob1.position - ob1.width) / ob1.velocity;
    let box_distance = ob2.position - ob2.width - (ob1.position + ob1.width);
    let time_to_box = box_distance / (ob1.velocity - ob2.velocity);
    (time_to_wall, time_to_box)
}
fn calculate_after_collision_speeds(ob1: &physBox, ob2: &physBox) -> (f64, f64) {
    let u1 = ob1.velocity;
    let u2 = ob2.velocity;
    let m1 = ob1.mass;
    let m2 = ob2.mass;
    let v1 = (u1 * (m1 - m2) + 2.0 * m2 * u2) / (m1 + m2);
    let v2 = (u2 * (m2 - m1) + 2.0 * m1 * u1) / (m1 + m2);
    (v1, v2)
}
fn box_mov_offset(ob1: &physBox, ob2: &physBox, time_elapse: f64) -> (f64, f64) {
    (ob1.velocity * time_elapse, ob2.velocity * time_elapse)
}
fn no_more_collisions(ob1: &physBox, ob2: &physBox) -> bool {
    if ob1.velocity >= 0.0 && ob2.velocity >= ob1.velocity {
        return true;
    }
    false
}
fn run_sim(ob1: &physBox, ob2: &physBox) -> (physBox, physBox, f64) {
    let times = predict_collision(ob1, ob2);
    let time_elapsed: f64;
    let mut moves: (f64, f64);
    let mut speeds: (f64, f64) = (ob1.velocity, ob2.velocity);
    if wall_first(times.0, times.1) {
        time_elapsed = times.0;
        moves = box_mov_offset(ob1, ob2, time_elapsed.abs());
        moves.0 += 0.001;
        speeds = (-speeds.0, speeds.1);
    } else {
        time_elapsed = times.1;
        moves = box_mov_offset(ob1, ob2, time_elapsed.abs());
        moves.1 += 0.001;
        speeds = calculate_after_collision_speeds(ob1, ob2);
    }
    (
        build_box(ob1.position + moves.0, speeds.0, ob1.mass, ob1.width),
        build_box(ob2.position + moves.1, speeds.1, ob2.mass, ob2.width),
        time_elapsed,
    )
}
pub fn get_pi(second_box_mass: f64, verbose: bool) {
    let mut box1 = build_box(5.0, 0.0, 1.0, 1.0);
    let mut box2 = build_box(10.0, -1.0, second_box_mass, 1.0);
    let mut collision: u32 = 0;
    loop {
        if no_more_collisions(&box1, &box2) {
            println!("end");
            break;
        }
        let boxes = run_sim(&box1, &box2);
        collision += 1;
        box1 = boxes.0;
        box2 = boxes.1;
        if verbose {
            println!(
                "Values after collision:{}, box1 pos:{}, box1 vel:{}, box2 pos:{}, box2 vel:{}, time elapsed since last collision:{}",
                collision, box1.position, box1.velocity, box2.position, box2.velocity, boxes.2,
            );
        }
    }
    println!("collision num: {collision}")
}
