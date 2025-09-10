# PID Controller library

Rust Crate: https://crates.io/crates/adriftdev_pid

Proportional, integral, and derivative controller module designed to allow easy calculation of outputs based on feedback loop from plant equipment (sensors / microcontrollers). output of PID controller can control motors, servos, or any component that can output a varities of outputs to achieve targeted outcome.

## RoadMap

- Smoothing of output curve.
- PID Stack control - can use any variety of controller pattern, not just PID, eg PD, PI, P, or PID controller configurations.
- General microcontroller optimisations

## Example Usage

The following example shows how to use the PID controller to bring a simulated system to a target setpoint.

```rust
use adriftdev_pid::control;
use std::{thread, time::Duration};

fn main() {
    // Create a new PID controller module
    let mut pid = control::Module::new(
        control::PController::new(0.1),
        control::IController::new(0.05),
        control::DController::new(0.01),
    );

    // Set the target value for the PID controller
    let setpoint = 100.0;
    pid.set_setpoint(setpoint);
    println!("Setpoint: {}", setpoint);

    // Set the output limits for the PID controller
    pid.set_output_limits(-10.0, 10.0);

    // Simulate a simple plant (e.g., a heater, a motor, etc.)
    let mut plant_state = 0.0; // The current state of our system (e.g., temperature)

    println!("--- Starting simulation ---");
    // Loop for a fixed number of iterations to simulate the control process
    for i in 0..200 {
        // The PID controller computes the output based on the current plant state
        let output = pid.compute(plant_state);

        // The output of the PID controller affects the plant's state.
        // This is a very simple plant model. In a real system, this would be
        // the physics of the system you are controlling.
        plant_state += output * 0.1;

        println!(
            "Iteration {}: Plant State = {:.2}, PID Output = {:.2}",
            i, plant_state, output
        );

        // In a real application, you would have a delay here that matches
        // the sample time of your controller.
        thread::sleep(Duration::from_millis(10));

        // For this example, we'll break if we are close to the setpoint
        if (plant_state - setpoint).abs() < 0.1 {
            println!("--- Setpoint reached ---");
            break;
        }
    }
    println!("--- Simulation finished ---");
}
```

### API Notes
- The `compute` method now takes the current sensor reading (process variable) as an `input: f64`.
- You can set output limits using `set_output_limits(min: f64, max: f64)`.

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
