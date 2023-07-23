use rust_hdl::prelude::*;

#[derive(Default, LogicBlock)]
pub struct NandGate {
    pub input_1: Signal<In, Bits<1>>,
    pub input_2: Signal<In, Bits<1>>,
    pub output_signal: Signal<Out, Bits<1>>,
}

impl Logic for NandGate {
    #[hdl_gen]
    fn update(&mut self) {
        self.output_signal.next = !(self.input_1.val() & self.input_2.val());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut sim = Simulation::new();
        sim.add_testbench(move |mut endpoint: Sim<NandGate>| {
            let mut x = endpoint.init()?;

            x.input_1.next = true.into();
            x.input_2.next = true.into();

            let mut x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), false, x);

            x.input_1.next = false.into();
            x.input_2.next = true.into();

            let mut x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), true, x);

            x.input_1.next = false.into();
            x.input_2.next = false.into();

            let x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), true, x);

            endpoint.done(x)
        });
        sim.run_to_file(
            Box::new(NandGate::default()),
            5 * sim_time::ONE_SEC,
            "nandgate.vcd",
        )
        .unwrap();
    }
}
