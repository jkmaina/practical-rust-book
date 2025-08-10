// Observer pattern implementation using Rc<T> and RefCell<T>
// Demonstrates shared ownership (Rc) and interior mutability (RefCell)
// Shows how smart pointers enable flexible design patterns in Rust

use std::rc::Rc;
use std::cell::RefCell;

// Define a trait for observers
trait Observer {
    fn update(&self, message: &str);
    fn name(&self) -> &str;
}

// A basic observer that just prints messages
struct ConsoleObserver {
    id: u32,
}

impl Observer for ConsoleObserver {
    fn update(&self, message: &str) {
        println!("[Console Observer {}] {}", self.id, message);
    }
    
    fn name(&self) -> &str {
        "ConsoleObserver"
    }
}

// A logger observer that formats messages differently
struct LoggerObserver {
    name: String,
    log_level: String,
}

impl Observer for LoggerObserver {
    fn update(&self, message: &str) {
        println!("[{}] {}: {}", self.log_level, self.name, message);
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

// A counting observer that tracks how many messages it received
struct CountingObserver {
    name: String,
    count: RefCell<u32>,  // Interior mutability for counting
}

impl CountingObserver {
    fn new(name: &str) -> Self {
        CountingObserver {
            name: name.to_string(),
            count: RefCell::new(0),
        }
    }
    
    fn get_count(&self) -> u32 {
        *self.count.borrow()
    }
}

impl Observer for CountingObserver {
    fn update(&self, message: &str) {
        *self.count.borrow_mut() += 1;
        println!("[{}] Message #{}: {}", self.name, self.get_count(), message);
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

// The subject that observers will watch
struct Subject {
    observers: RefCell<Vec<Rc<dyn Observer>>>,
    name: String,
}

impl Subject {
    fn new(name: &str) -> Self {
        println!("Creating subject: {}", name);
        Subject {
            observers: RefCell::new(Vec::new()),
            name: name.to_string(),
        }
    }
    
    fn add_observer(&self, observer: Rc<dyn Observer>) {
        println!("Adding observer '{}' to subject '{}'", observer.name(), self.name);
        self.observers.borrow_mut().push(observer);
    }
    
    fn remove_observer(&self, observer_name: &str) -> bool {
        let mut observers = self.observers.borrow_mut();
        if let Some(pos) = observers.iter().position(|obs| obs.name() == observer_name) {
            observers.remove(pos);
            println!("Removed observer '{}' from subject '{}'", observer_name, self.name);
            true
        } else {
            false
        }
    }
    
    fn notify(&self, message: &str) {
        let observer_count = self.observers.borrow().len();
        println!("\n[{}] Notifying {} observers: {}", self.name, observer_count, message);
        
        for observer in self.observers.borrow().iter() {
            observer.update(message);
        }
    }
    
    fn observer_count(&self) -> usize {
        self.observers.borrow().len()
    }
}

fn main() {
    println!("=== Observer Pattern Demo ===");
    
    // Create a subject
    let subject = Subject::new("NewsPublisher");
    
    // Create different types of observers as trait objects
    let console_obs1: Rc<dyn Observer> = Rc::new(ConsoleObserver { id: 1 });
    let console_obs2: Rc<dyn Observer> = Rc::new(ConsoleObserver { id: 2 });
    
    let logger_obs: Rc<dyn Observer> = Rc::new(LoggerObserver {
        name: "SystemLogger".to_string(),
        log_level: "INFO".to_string(),
    });
    
    let counter_obs_concrete = CountingObserver::new("MessageCounter");
    let counter_obs: Rc<dyn Observer> = Rc::new(counter_obs_concrete);
    
    // Add observers to the subject
    subject.add_observer(Rc::clone(&console_obs1));
    subject.add_observer(Rc::clone(&console_obs2));
    subject.add_observer(Rc::clone(&logger_obs));
    subject.add_observer(Rc::clone(&counter_obs));
    
    println!("\nTotal observers: {}", subject.observer_count());
    
    // Send notifications
    subject.notify("Breaking: Rust 1.75 released!");
    subject.notify("Weather: Sunny with a chance of code");
    subject.notify("Sports: Local team wins championship");
    
    // Note: Can't access concrete methods through trait object
    println!("\nMessages sent to all observers");
    
    // Remove an observer
    println!("\n=== Removing Observer ===");
    subject.remove_observer("SystemLogger");
    
    // Send another notification
    subject.notify("Final message after removal");
    
    println!("\nFinal observer count: {}", subject.observer_count());
    
    println!("\n=== Smart Pointer Benefits ===");
    println!("- Rc<T> enables multiple owners of the same observer");
    println!("- RefCell<T> allows mutation of observer list through immutable reference");
    println!("- Interior mutability in CountingObserver for state changes");
}
 
