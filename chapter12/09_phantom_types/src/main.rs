use std::marker::PhantomData;
// States for a state machine
struct Idle;
struct Running;
struct Finished;
// StateMachine with a phantom type parameter for the state
struct StateMachine<State> {
    // No actual field of type State
    data: Vec<u32>,
    _state: PhantomData<State>,
}
// Implementations for specific states
impl StateMachine<Idle> {
    fn new() -> Self {
        StateMachine {
            data: Vec::new(),
            _state: PhantomData,
        }
    }
    
    fn start(self) -> StateMachine<Running> {
        StateMachine {
            data: self.data,
            _state: PhantomData,
        }
    }
}
impl StateMachine<Running> {
    fn process(&mut self, value: u32) {
        self.data.push(value);
    }
    
    fn finish(self) -> StateMachine<Finished> {
        StateMachine {
            data: self.data,
            _state: PhantomData,
        }
    }
}
impl StateMachine<Finished> {
    fn result(&self) -> &[u32] {
        &self.data
    }
}
fn main() {
    // Type-safe state transitions
    let machine = StateMachine::new();
    let mut running = machine.start();
    
    running.process(1);
    running.process(2);
    running.process(3);
    
    let finished = running.finish();
    println!("Result: {:?}", finished.result());
    
    // This would not compile:
    // running.process(4);  // Error: running has been moved
    // machine.start();     // Error: machine has been moved
    // finished.process(5); // Error: no method 'process' on StateMachine<Finished>
}
 
