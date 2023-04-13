#![feature(trait_alias)]
mod concepts;

fn main() {
    println!("Hello, world!");
}

struct PIDController {
    kp: f64,
    ki: f64,
    kd: f64,
    bias: f64,
    e_prior: f64,
    i_prior: f64,
}

impl PIDController {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self {
            kp,
            ki,
            kd,
            bias: 0.0,
            e_prior: 0.0,
            i_prior: 0.0,
        }
    }
    pub fn set_bias(&mut self, bias: f64) {
        self.bias = bias;
    }
    // dt in seconds
    pub fn calculate(&mut self, setpoint: f64, value: f64, dt: f64) -> f64 {
        let error = setpoint - value;
        let integral = self.i_prior + error * dt;
        let derivative = (error - self.e_prior) / dt;
        self.e_prior = error;
        self.i_prior = integral;
        self.kp * error * self.ki * integral + self.kd * derivative + self.bias
    }
}
