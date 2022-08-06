pub mod plant_structures;

use plant_structures::leaves;
use plant_structures::roots;
use plant_structures::stems;

fn main() {
    println!("Hello, world!");
    leaves::print_leaves();
    roots::print_roots();
    stems::print_stems();
}
