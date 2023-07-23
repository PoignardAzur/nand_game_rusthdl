use rust_hdl::prelude::*;

use crate::{NandGate, NotGate};

#[derive(LogicBlock)]
pub struct AndGate {
    pub input_1: Signal<In, Bits<1>>,
    pub input_2: Signal<In, Bits<1>>,
    pub output_signal: Signal<Out, Bits<1>>,
    nand: NandGate,
    invert: NotGate,
}

impl Default for AndGate {
    fn default() -> Self {
        Self {
            input_1: Default::default(),
            input_2: Default::default(),
            output_signal: Default::default(),
            nand: Default::default(),
            invert: Default::default(),
        }
    }
}

impl Logic for AndGate {
    #[hdl_gen]
    fn update(&mut self) {
        self.nand.input_1.next = self.input_1.val();
        self.nand.input_2.next = self.input_2.val();
        self.invert.input_signal.next = self.nand.output_signal.val();
        self.output_signal.next = self.invert.output_signal.val();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut sim = Simulation::new();
        sim.add_testbench(move |mut endpoint: Sim<AndGate>| {
            let mut x = endpoint.init()?;

            x.input_1.next = true.into();
            x.input_2.next = true.into();

            let mut x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), true, x);

            x.input_1.next = false.into();
            x.input_2.next = true.into();

            let mut x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), false, x);

            x.input_1.next = false.into();
            x.input_2.next = false.into();

            let x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), false, x);

            endpoint.done(x)
        });
        sim.run_to_file(
            Box::new(AndGate::default()),
            5 * sim_time::ONE_SEC,
            "andgate.vcd",
        )
        .unwrap();
    }
}
