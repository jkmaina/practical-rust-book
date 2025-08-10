// This program simulates a weather station's data collection and display.
// It includes current weather conditions, a forecast, and basic statistics.
// The program demonstrates the use of variables, arrays, and basic control flow in Rust.
// It will not panic, as it avoids out-of-bounds access.
// This is a common way to illustrate Rust's safety features regarding array access.

fn main() {
    // Weather station data
    let station_id = "WS-123";
    let location = "Portland, OR";
    let is_active = true;
    
    // Current weather conditions
    let temperature_celsius = 23.5;
    let humidity_percent = 65;
    let pressure_hpa = 1013.25;
    let conditions = "Partly Cloudy";
    
    // Wind information (direction in degrees, speed in km/h)
    let wind = (120.0, 8.5);
    
    // Hourly temperature forecast for the next 5 hours
    let temperature_forecast = [24.0, 25.5, 26.0, 25.5, 24.0];
    
    // Display current conditions
    println!("Weather Station: {} ({})", station_id, location);
    println!("Status: {}", if is_active { "Active" } else { "Inactive" });
    println!("Current Conditions: {}", conditions);
    println!("Temperature: {:.1}°C", temperature_celsius);
    println!("Humidity: {}%", humidity_percent);
    println!("Pressure: {:.2} hPa", pressure_hpa);
    println!("Wind: {:.1}° at {:.1} km/h", wind.0, wind.1);
    
    // Display forecast
    println!("\nTemperature Forecast:");
    for (hour, temp) in temperature_forecast.iter().enumerate() {
        println!("Hour +{}: {:.1}°C", hour + 1, temp);
    }
    
    // Calculate statistics
    let forecast_length = temperature_forecast.len();
    let mut sum = 0.0;
    for temp in temperature_forecast.iter() {
        sum += temp;
    }
    let average_temp = sum / forecast_length as f64;
    
    println!("\nAverage forecasted temperature: {:.1}°C", average_temp);
    
    // Determine if temperature will rise
    let will_rise = temperature_forecast[forecast_length - 1] > temperature_celsius;
    println!("Temperature trend: {}", if will_rise { "Rising" } else { "Falling or stable" });
}
