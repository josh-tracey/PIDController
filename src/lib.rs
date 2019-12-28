pub mod control {

    pub struct PController {
        pub error: f64,
        pub gain: f64,
    }
    pub struct IController {
        pub error_sum: f64,
        pub gain: f64,
    }
    pub struct DController {
        pub gain: f64,
        pub last_input: f64,
    }

    impl PController {
        pub fn new(gain: f64) -> PController {
            PController {
                gain: gain,
                error: 0.0,
            }
        }
        pub fn calc(&self) -> f64 {
            self.gain * self.error
        }
        pub fn set_error(&mut self, value: f64) {
            self.error = value;
        }
        pub fn set_gain(&mut self, value: f64) {
            self.gain = value;
        }
    }
    impl IController {
        pub fn new(gain: f64) -> IController {
            IController {
                gain: gain,
                error_sum: 0.0,
            }
        }
        pub fn calc(&self) -> f64 {
            self.gain * self.error_sum
        }
        pub fn set_error(&mut self, value: f64) {
            self.error_sum += value;
        }
        pub fn set_gain(&mut self, value: f64) {
            self.gain = value;
        }
    }
    impl DController {
        pub fn new(gain: f64) -> DController {
            DController {
                gain: gain,
                last_input: 0.0,
            }
        }
        pub fn calc(&mut self, input: f64) -> f64 {
            let dinput = input - self.last_input;
            self.last_input = input;
            self.gain * dinput
        }
        pub fn set_gain(&mut self, value: f64) {
            self.gain = value;
        }
    }

    pub struct Module {
        pub kp: PController,
        pub ki: IController,
        pub kd: DController,
        pub setpoint: f64,
        pub input: f64,
        pub output: f64,
        pub last_input: f64,
        pub max_output: f64,
        pub min_output: f64,
        pub time_change: std::time::Duration,
        pub last_time: std::time::Instant,
        pub sample_time: std::time::Duration,
    }

    impl Module {
        pub fn new(kp: PController, ki: IController, kd: DController) -> Module {
            Module {
                kp: kp,
                ki: ki,
                kd: kd,
                setpoint: 0.0,
                input: 0.0,
                last_input: 0.0,
                output: 0.0,
                min_output: 0.0,
                max_output: 0.0,
                time_change: std::time::Duration::new(0, 0),
                last_time: std::time::Instant::now(),
                sample_time: std::time::Duration::new(0, 100),
            }
        }
        pub fn compute(&mut self) -> f64 {
            let now = std::time::Instant::now();
            self.time_change = now - self.last_time;
            if self.time_change >= self.sample_time {
                self.kp.set_error(self.setpoint - self.input);
                self.ki.set_error(self.setpoint - self.input);
                let kp = self.kp.calc();
                let ki = self.ki.calc();
                let kd = self.kd.calc(self.input);
                self.output = kp + ki + kd;
                self.last_time = now;
            }
            self.last_input = self.input;
            self.input = self.output;
            self.output
        }
        pub fn set_setpoint(&mut self, value: f64) {
            self.setpoint = value;
        }
        pub fn set_output(&mut self, value: f64) {
            self.input += value;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
