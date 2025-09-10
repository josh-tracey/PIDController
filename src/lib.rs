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
        pub fn update_error_sum(&mut self, error: f64, dt: f64) {
            self.error_sum += error * dt;
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
        pub fn calc(&mut self, input: f64, dt: f64) -> f64 {
            let d_input = input - self.last_input;
            self.last_input = input;
            if dt > 0.0 {
                return self.gain * (d_input / dt);
            }
            0.0
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
        pub output: f64,
        pub output_min: f64,
        pub output_max: f64,
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
                output: 0.0,
                output_min: -1.0,
                output_max: 1.0,
                last_time: std::time::Instant::now(),
                sample_time: std::time::Duration::from_millis(10),
            }
        }
        pub fn compute(&mut self, input: f64) -> f64 {
            let now = std::time::Instant::now();
            let dt = (now - self.last_time).as_secs_f64();

            if dt >= self.sample_time.as_secs_f64() {
                let error = self.setpoint - input;
                self.kp.set_error(error);
                self.ki.update_error_sum(error, dt);

                let p_term = self.kp.calc();
                let i_term = self.ki.calc();
                let d_term = self.kd.calc(input, dt);

                let mut output = p_term + i_term + d_term;

                // Clamp output
                output = output.max(self.output_min).min(self.output_max);

                self.output = output;
                self.last_time = now;
            }

            self.output
        }
        pub fn set_setpoint(&mut self, value: f64) {
            self.setpoint = value;
        }
        pub fn set_output_limits(&mut self, min: f64, max: f64) {
            self.output_min = min;
            self.output_max = max;
        }
        pub fn set_sample_time(&mut self, millis: u64) {
            self.sample_time = std::time::Duration::from_millis(millis);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::control::{DController, IController, Module, PController};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn p_controller_calc() {
        let mut p = PController::new(0.5);
        p.set_error(10.0);
        assert_eq!(p.calc(), 5.0);
    }

    #[test]
    fn i_controller_update_error_sum() {
        let mut i = IController::new(0.5);
        i.update_error_sum(10.0, 0.1);
        assert_eq!(i.error_sum, 1.0);
        assert_eq!(i.calc(), 0.5);
    }

    #[test]
    fn d_controller_calc() {
        let mut d = DController::new(0.5);
        d.last_input = 5.0;
        let d_term = d.calc(10.0, 0.1);
        assert_eq!(d_term, 25.0);
    }

    #[test]
    fn module_computes_within_limits() {
        let mut pid = Module::new(
            PController::new(1.0),
            IController::new(1.0),
            DController::new(1.0),
        );
        pid.set_output_limits(-0.5, 0.5);
        pid.set_setpoint(100.0);
        let output = pid.compute(0.0);
        assert!(output >= -0.5 && output <= 0.5);
    }

    #[test]
    fn module_setpoint() {
        let mut pid = Module::new(
            PController::new(1.0),
            IController::new(1.0),
            DController::new(1.0),
        );
        pid.set_setpoint(50.0);
        assert_eq!(pid.setpoint, 50.0);
    }

    #[test]
    fn module_output_direction() {
        let mut pid = Module::new(
            PController::new(0.5),
            IController::new(0.2),
            DController::new(0.0), // D gain is zero for this test
        );
        pid.set_setpoint(100.0);
        pid.set_sample_time(10);
        thread::sleep(Duration::from_millis(20));

        // Input is below setpoint, output should be positive
        let output1 = pid.compute(0.0);
        assert!(output1 > 0.0);

        thread::sleep(Duration::from_millis(20));

        // Input is above setpoint, output should be negative
        let output2 = pid.compute(200.0);
        assert!(output2 < 0.0);
    }
}
