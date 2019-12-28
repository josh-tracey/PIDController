use adriftdev_pid::control;

fn main() {
  let mut pid = control::Module::new(
    control::PController::new(0.2),
    control::IController::new(0.2),
    control::DController::new(0.2),
  ); // Total of 0.6 gain
  pid.set_setpoint(2000.0);
  while pid.output < 1999.0 {
    println!("{}", pid.compute());
  }
}
