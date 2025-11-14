//! State machine testing demo
//!
//! Shows how to test statig state machines on host (no hardware needed)

use statig::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    ButtonPressed,
}

#[derive(Default)]
pub struct SimpleMachine;

#[state_machine(
    initial = "State::off()",
    state(derive(Debug, Clone, PartialEq))
)]
impl SimpleMachine {
    #[state]
    fn off(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::on()),
        }
    }

    #[state]
    fn on(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::off()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let sm = SimpleMachine::default().state_machine();
        assert_eq!(sm.state(), &State::off());
    }

    #[test]
    fn test_toggle_on() {
        let mut sm = SimpleMachine::default().state_machine();
        sm.handle(&Event::ButtonPressed);
        assert_eq!(sm.state(), &State::on());
    }

    #[test]
    fn test_toggle_off() {
        let mut sm = SimpleMachine::default().state_machine();
        sm.handle(&Event::ButtonPressed); // Off → On
        sm.handle(&Event::ButtonPressed); // On → Off
        assert_eq!(sm.state(), &State::off());
    }

    #[test]
    fn test_multiple_toggles() {
        let mut sm = SimpleMachine::default().state_machine();
        for _ in 0..10 {
            sm.handle(&Event::ButtonPressed);
            assert_eq!(sm.state(), &State::on());
            sm.handle(&Event::ButtonPressed);
            assert_eq!(sm.state(), &State::off());
        }
    }
}
