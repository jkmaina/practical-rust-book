trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self) -> Result<(), String>;
}
struct Logger {
    log_level: String,
}
impl Plugin for Logger {
    fn name(&self) -> &str {
        "Logger Plugin"
    }
    
    fn execute(&self) -> Result<(), String> {
        println!("Logging with level: {}", self.log_level);
        Ok(())
    }
}
struct Analyzer {
    algorithm: String,
}
impl Plugin for Analyzer {
    fn name(&self) -> &str {
        "Analyzer Plugin"
    }
    
    fn execute(&self) -> Result<(), String> {
        println!("Analyzing data with algorithm: {}", self.algorithm);
        Ok(())
    }
}
struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}
impl PluginManager {
    fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
    
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        println!("Registering plugin: {}", plugin.name());
        self.plugins.push(plugin);
    }
    
    fn execute_all(&self) -> Result<(), String> {
        for plugin in &self.plugins {
            println!("Executing plugin: {}", plugin.name());
            plugin.execute()?;
        }
        Ok(())
    }
}
fn main() {
    let mut manager = PluginManager::new();
    
    manager.register_plugin(Box::new(Logger {
        log_level: String::from("DEBUG"),
    }));
    
    manager.register_plugin(Box::new(Analyzer {
        algorithm: String::from("Fast Fourier Transform"),
    }));
    
    if let Err(e) = manager.execute_all() {
        println!("Error executing plugins: {}", e);
    }
}
