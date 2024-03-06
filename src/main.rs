use smlang::statemachine;
use std::fmt::{self, Display, Formatter};

mod linear_actuator;
use linear_actuator::LinearActuator;

use getch::Getch;

pub enum Events {
    EvDefault,
    EvRemoveStabilization,
    EvStabilizev,
}

statemachine! {
    transitions: {
        *StateInitial + EvDefault = StateDetermineStabilization,
        StateDetermineStabilization + EvDefault [ctx.is_stabilized()] = StateStabilized,
        StateDetermineStabilization + EvDefault [!ctx.is_stabilized()] = StateOnWheels,
        StateStabilized + EvRemoveStabilization / ctx.action_retract_stabilizers(); = StateOnWheels,
        StateOnWheels + EvStabilizev / ctx.action_extend_stabilizers(); = StateStabilized,
    }
}

impl Display for States {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            States::StateInitial => write!(f, "StateInitial"),
            States::StateDetermineStabilization => write!(f, "StateDetermineStabilization"),
            States::StateStabilized => write!(f, "StateStabilized"),
            States::StateOnWheels => write!(f, "StateOnWheels"),
        }
    }
}

/// Context
pub struct Context {
    linear_actuator: LinearActuator,
}

impl Context {
    fn is_stabilized(&self) -> bool {
        self.linear_actuator == LinearActuator::Extended
    }

    fn action_retract_stabilizers(&mut self) {
        self.linear_actuator.retract();
    }

    fn action_extend_stabilizers(&mut self) {
        self.linear_actuator.extend();
    }
}

fn main() {
    let ctx = Context {
        linear_actuator: LinearActuator::new(),
    };
    let mut sm = StateMachine::new(ctx);

    let console = Getch::new();

    loop {
        println!(">>> Current state: {}", sm.state());

        if sm.process_event(Events::EvDefault).is_some() {
            continue;
        }

        println!("Enter (r)emove stabilization, (s)tabilize, (q)uit: ");
        let input = console.getch().unwrap();
        let event = match input {
            b'r' => Events::EvRemoveStabilization,
            b's' => Events::EvStabilizev,
            b'q' => break,
            _ => continue,
        };

        sm.process_event(event);
    }
}
