use rust_hdl::prelude::*;

use crate::{NandGate, NotGate};

#[derive(LogicBlock)]
pub struct OrGate {
    pub input_1: Signal<In, Bits<1>>,
    pub input_2: Signal<In, Bits<1>>,
    pub output_signal: Signal<Out, Bits<1>>,
    invert_1: NotGate,
    invert_2: NotGate,
    nand: NandGate,
}

impl Default for OrGate {
    fn default() -> Self {
        Self {
            input_1: Default::default(),
            input_2: Default::default(),
            output_signal: Default::default(),
            invert_1: Default::default(),
            invert_2: Default::default(),
            nand: Default::default(),
        }
    }
}

impl Logic for OrGate {
    #[hdl_gen]
    fn update(&mut self) {
        self.invert_1.input_1.next = self.input_1.val();
        self.invert_2.input_1.next = self.input_2.val();
        self.nand.input_1.next = self.invert_1.output_signal.val();
        self.nand.input_2.next = self.invert_2.output_signal.val();
        self.output_signal.next = self.nand.output_signal.val();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut sim = Simulation::new();
        sim.add_testbench(move |mut endpoint: Sim<OrGate>| {
            let mut x = endpoint.init()?;

            x.input_1.next = true.into();
            x.input_2.next = true.into();

            let mut x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), true, x);

            x.input_1.next = false.into();
            x.input_2.next = true.into();

            let mut x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), true, x);

            x.input_1.next = false.into();
            x.input_2.next = false.into();

            let x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), false, x);

            endpoint.done(x)
        });
        sim.run_to_file(
            Box::new(OrGate::default()),
            5 * sim_time::ONE_SEC,
            "orgate.vcd",
        )
        .unwrap();
    }
}
