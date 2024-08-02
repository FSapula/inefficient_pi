mod phys;
fn main() {
    // first box mass is 1, for second box mass equal to 100^n the number of collisions will be the first n + 1
    // digits of pi
    let second_box_mass = 100000000000000.0;
    // shows info about every collision
    let verbose = false;
    phys::get_pi(second_box_mass, verbose);
}
