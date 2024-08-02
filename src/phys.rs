struct Box {
    position: f64,
    velocity: f64,
    mass: f64,
    width: f64,
}
fn predict_collision(ob1: Box, ob2: Box) -> (f64, f64) {
    let time_to_wall = -1.0 * (ob1.position + ob1.width) / ob1.velocity;
    let box_distance = ob2.position - ob2.width - (ob1.position + ob1.width);
    let time_to_box = box_distance / (ob1.velocity - ob2.velocity);
    (time_to_wall, time_to_box)
}
fn calculate_collision(v1: f64, v2: f64, m1: f64, m2: f64) -> f64 {
    v1 * ((m1 - m2) + 2.0 * m2 * v2) / (m1 + m2)
}
fn calculate_after_collision_speeds(ob1: Box, ob2: Box) -> (f64, f64) {
    let u1 = ob1.velocity;
    let u2 = ob2.velocity;
    let m1 = ob1.mass;
    let m2 = ob2.mass;
    let v1 = calculate_collision(u1, u2, m1, m2);
    let v2 = calculate_collision(u2, u1, m2, m1);
    (v1, v2)
}
