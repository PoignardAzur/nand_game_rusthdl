mod level_and;
mod level_invert;
mod level_nand;

pub use level_and::AndGate;
pub use level_invert::NotGate;
pub use level_nand::NandGate;

#[allow(unused)]
const CLOCK_SPEED_HZ: u64 = 10_000;

fn main() {
    println!("Hello, world!");
}
