# PID Controller library

Rust Crate: https://crates.io/crates/adriftdev_pid

Proportional, integral, and derivative controller module designed to allow easy calculation of outputs based on feedback loop from plant equipment (sensors / microcontrollers). output of PID controller can control motors, servos, or any component that can output a varities of outputs to achieve targeted outcome.

## RoadMap

- Smoothing of output curve.
- PID Stack control - can use any variety of controller pattern, not just PID, eg PD, PI, P, or PID controller configurations.
- General microcontroller optimisations

## Example Usage

Tuning of the PID Controllers is as simple as changing the gain for each controller component for a module.

below is a small example of creating a PID Control Module

```rust
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
```

## Possible Applications

There are plenty of uses for PID Controllers this is just a small sample of usages.

### Air Conditioner

- Temperature regulation - through controlled output and feebback loop from temp sensors`

### Quadcopter

- Altitude control
- Speed control
- Rotation control
- Tilt control
- Advanced Navigation

### Electric Skateboard

- Speed Control
- Brake Control
