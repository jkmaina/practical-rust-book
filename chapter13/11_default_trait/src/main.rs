// Default trait provides default values for types
// Can be derived automatically or implemented manually for custom defaults

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Settings {
    font_size: u32,     // defaults to 0
    dark_mode: bool,    // defaults to false
    auto_save: bool,    // defaults to false
}

#[allow(dead_code)]
#[derive(Debug)]
struct AppConfig {
    theme: String,
    font_size: u32,
    notifications: bool,
}

// Manual Default implementation with custom values
impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            theme: String::from("light"),
            font_size: 12,
            notifications: true,
        }
    }
}

fn main() {
    // Using derived Default (all fields get their type's default)
    let default_settings = Settings::default();
    println!("Default settings: {:?}", default_settings);
    
    // Using struct update syntax with defaults
    let custom_settings = Settings {
        font_size: 14,
        ..Settings::default()
    };
    println!("Custom settings: {:?}", custom_settings);
    
    // Using manual Default implementation
    let app_config = AppConfig::default();
    println!("Default app config: {:?}", app_config);
    
    // Overriding some defaults
    let dark_config = AppConfig {
        theme: String::from("dark"),
        ..AppConfig::default()
    };
    println!("Dark config: {:?}", dark_config);
}