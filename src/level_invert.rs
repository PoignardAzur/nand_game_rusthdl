use rust_hdl::prelude::*;

#[derive(Default, LogicBlock)]
pub struct NotGate {
    pub input_signal: Signal<In, Bits<1>>,
    pub output_signal: Signal<Out, Bits<1>>,
}

impl Logic for NotGate {
    #[hdl_gen]
    fn update(&mut self) {
        self.output_signal.next = !self.input_signal.val();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut sim = Simulation::new();
        sim.add_testbench(move |mut endpoint: Sim<NotGate>| {
            let mut x = endpoint.init()?;

            x.input_signal.next = true.into();
            let mut x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), false, x);

            x.input_signal.next = false.into();
            let x = endpoint.wait(10 * sim_time::ONE_MICROSECOND, x).unwrap();
            sim_assert_eq!(endpoint, x.output_signal.val(), true, x);

            endpoint.done(x)
        });
        sim.run_to_file(
            Box::new(NotGate::default()),
            5 * sim_time::ONE_SEC,
            "notgate.vcd",
        )
        .unwrap();
    }
}
