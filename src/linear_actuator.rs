#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinearActuator {
    Retracted,
    Extended,
}

impl LinearActuator {
    pub fn new() -> Self {
        // randomly initialize the actuator
        if rand::random() {
            LinearActuator::Retracted
        } else {
            LinearActuator::Extended
        }
    }


    pub fn retract(&mut self) {
        *self = LinearActuator::Retracted;
    }

    pub fn extend(&mut self) {
        *self = LinearActuator::Extended;
    }
}
