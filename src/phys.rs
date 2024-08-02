pub struct physBox {
    pub position: f64,
    pub velocity: f64,
    mass: f64,
    width: f64,
}
pub fn build_box(position: f64, velocity: f64, mass: f64, width: f64) -> physBox {
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
fn calculate_collision(v1: f64, v2: f64, m1: f64, m2: f64) -> f64 {
    v1 * ((m1 - m2) + 2.0 * m2 * v2) / (m1 + m2)
}
fn calculate_after_collision_speeds(ob1: &physBox, ob2: &physBox) -> (f64, f64) {
    let u1 = ob1.velocity;
    let u2 = ob2.velocity;
    let m1 = ob1.mass;
    let m2 = ob2.mass;
    let v1 = calculate_collision(u1, u2, m1, m2);
    let v2 = calculate_collision(u2, u1, m2, m1);
    (v1, v2)
}
fn box_mov_offset(ob1: &physBox, ob2: &physBox, time_elapse: f64) -> (f64, f64) {
    (ob1.velocity * time_elapse, ob2.velocity * time_elapse)
}
pub fn no_more_collisions(ob1: &physBox, ob2: &physBox) -> bool {
    if ob1.velocity > 0.0 && ob2.velocity >= ob1.velocity {
        return true;
    }
    false
}
pub fn run_sim(ob1: &physBox, ob2: &physBox) -> (physBox, physBox, f64) {
    let times = predict_collision(ob1, ob2);
    let time_elapsed: f64;
    let mut moves: (f64, f64);
    let mut speeds: (f64, f64) = (ob1.velocity, ob2.velocity);
    if wall_first(times.0, times.1) {
        time_elapsed = times.0.abs();
        moves = box_mov_offset(ob1, ob2, time_elapsed);
        moves.0 += 0.001;
        speeds = (-speeds.0, speeds.1);
    } else {
        time_elapsed = times.1.abs();
        moves = box_mov_offset(ob1, ob2, time_elapsed);
        moves.1 += 0.001;
        speeds = calculate_after_collision_speeds(ob1, ob2);
    }
    (
        build_box(ob1.position + moves.0, speeds.0, ob1.mass, ob1.width),
        build_box(ob2.position + moves.1, speeds.1, ob2.mass, ob2.width),
        time_elapsed,
    )
}
