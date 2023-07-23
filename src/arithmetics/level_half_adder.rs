use rust_hdl::prelude::*;

use crate::{AndGate, XorGate};

#[derive(Default, LogicBlock)]
pub struct HalfAdder {
    pub input_1: Signal<In, Bits<1>>,
    pub input_2: Signal<In, Bits<1>>,
    pub low_bit: Signal<Out, Bits<1>>,
    pub high_bit: Signal<Out, Bits<1>>,
    xor: XorGate,
    and: AndGate,
}

impl Logic for HalfAdder {
    #[hdl_gen]
    fn update(&mut self) {
        self.xor.input_1.next = self.input_1.val();
        self.xor.input_2.next = self.input_2.val();
        self.and.input_1.next = self.input_1.val();
        self.and.input_2.next = self.input_2.val();
        self.low_bit.next = self.xor.output_signal.val();
        self.high_bit.next = self.and.output_signal.val();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut sim = Simulation::new();
        sim.add_testbench(move |mut endpoint: Sim<HalfAdder>| {
            let mut x = endpoint.init()?;

            x.input_1.next = 0.into();
            x.input_2.next = 0.into();
            let mut x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.low_bit.val(), 0, x);
            sim_assert_eq!(endpoint, x.high_bit.val(), 0, x);

            x.input_1.next = 1.into();
            x.input_2.next = 0.into();
            let mut x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.low_bit.val(), 1, x);
            sim_assert_eq!(endpoint, x.high_bit.val(), 0, x);

            x.input_1.next = 0.into();
            x.input_2.next = 1.into();
            let mut x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.low_bit.val(), 1, x);
            sim_assert_eq!(endpoint, x.high_bit.val(), 0, x);

            x.input_1.next = 1.into();
            x.input_2.next = 1.into();
            let x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.low_bit.val(), 0, x);
            sim_assert_eq!(endpoint, x.high_bit.val(), 1, x);

            endpoint.done(x)
        });
        sim.run_to_file(
            Box::new(HalfAdder::default()),
            5 * sim_time::ONE_SEC,
            "halfadder.vcd",
        )
        .unwrap();
    }
}
