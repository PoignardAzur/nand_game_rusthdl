use rust_hdl::prelude::*;

use crate::{AndGate, NandGate, OrGate};

#[derive(Default, LogicBlock)]
pub struct XorGate {
    pub input_1: Signal<In, Bits<1>>,
    pub input_2: Signal<In, Bits<1>>,
    pub output_signal: Signal<Out, Bits<1>>,
    nand: NandGate,
    or: OrGate,
    and: AndGate,
}

impl Logic for XorGate {
    #[hdl_gen]
    fn update(&mut self) {
        self.or.input_1.next = self.input_1.val();
        self.or.input_2.next = self.input_2.val();
        self.nand.input_1.next = self.input_1.val();
        self.nand.input_2.next = self.input_2.val();

        self.and.input_1.next = self.or.output_signal.val();
        self.and.input_2.next = self.nand.output_signal.val();
        self.output_signal.next = self.and.output_signal.val();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut sim = Simulation::new();
        sim.add_testbench(move |mut endpoint: Sim<XorGate>| {
            let mut x = endpoint.init()?;

            x.input_1.next = true.into();
            x.input_2.next = true.into();

            let mut x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), false, x);

            x.input_1.next = false.into();
            x.input_2.next = true.into();

            let mut x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), true, x);

            x.input_1.next = true.into();
            x.input_2.next = false.into();

            let mut x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), true, x);

            x.input_1.next = false.into();
            x.input_2.next = false.into();

            let x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), false, x);

            endpoint.done(x)
        });
        sim.run_to_file(
            Box::new(XorGate::default()),
            5 * sim_time::ONE_SEC,
            "xorgate.vcd",
        )
        .unwrap();
    }
}
