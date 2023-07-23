mod arithmetics;
mod logic_gates;

pub use logic_gates::level_and::AndGate;
pub use logic_gates::level_invert::NotGate;
pub use logic_gates::level_nand::NandGate;
pub use logic_gates::level_or::OrGate;
pub use logic_gates::level_xor::XorGate;

#[allow(unused)]
const CLOCK_SPEED_HZ: u64 = 10_000;

fn main() {
    println!("Hello, world!");
}
