//Data Processing Pipeline Example in Rust
// This program demonstrates a simple data processing pipeline in Rust.
// This Rust program processes temperature data, filtering invalid readings,
// calculating statistics, categorizing readings, and converting to Fahrenheit. 

fn main() {
    println!("Temperature Data Processing");
    println!("==========================");
    
    // Raw temperature data (in Celsius)
    let temperatures = [
        22.5, 19.8, 23.1, -25.0, 37.5, 25.2, 
        -30.0, 28.9, 22.3, 31.8, 24.5, -10.2
    ];
    
    println!("Processing {} temperature readings...", temperatures.len());
    
    // 1. Filter out invalid readings (using a for loop)
    let mut valid_temperatures = Vec::new();
    let mut invalid_count = 0;
    
    for &temp in temperatures.iter() {
        if temp < -20.0 || temp > 35.0 {
            println!("Discarding invalid reading: {}°C", temp);
            invalid_count += 1;
            continue;
        }
        
        valid_temperatures.push(temp);
    }
    
    println!("\nFound {} valid readings and {} invalid readings.", 
             valid_temperatures.len(), invalid_count);
    
    // 2. Calculate statistics (using a while loop)
    let mut i = 0;
    let mut sum = 0.0;
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    
    while i < valid_temperatures.len() {
        let temp = valid_temperatures[i];
        sum += temp;
        
        if temp < min {
            min = temp;
        }
        
        if temp > max {
            max = temp;
        }
        
        i += 1;
    }
    
    let avg = sum / valid_temperatures.len() as f64;
    
    println!("\nTemperature Statistics:");
    println!("Minimum: {:.1}°C", min);
    println!("Maximum: {:.1}°C", max);
    println!("Average: {:.1}°C", avg);
    
    // 3. Categorize readings (using a loop with break)
    println!("\nTemperature Categories:");
    
    let categories = ["Cold", "Cool", "Moderate", "Warm", "Hot"];
    let thresholds = [0.0, 15.0, 22.0, 28.0, 100.0];
    
    for &temp in valid_temperatures.iter() {
        let mut category = "Unknown";

        'category_loop: loop {
            for i in 0..categories.len() {
                if temp < thresholds[i] {
                    // If i == 0, assign the first category
                    category = if i == 0 {
                        categories[0]
                    } else {
                        categories[i - 1]
                    };
                    break 'category_loop;
                }
            }
            category = categories[categories.len() - 1];
            break;
        }

        println!("{:.1}°C: {}", temp, category);
    }
    
    // 4. Convert to Fahrenheit (using a for loop with enumerate)
    println!("\nConverting to Fahrenheit:");
    
    for (i, &celsius) in valid_temperatures.iter().enumerate() {
        let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
        println!("Reading #{}: {:.1}°C = {:.1}°F", i + 1, celsius, fahrenheit);
    }
}
