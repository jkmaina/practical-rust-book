// Data Science Example with Rust
// Demonstrates CSV processing, statistical analysis, and data visualization
// Uses ndarray for numerical computing and plotters for chart generation
// Shows practical data science workflows in Rust

use csv::Reader;
use ndarray::Array1;
use plotters::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Record {
    id: u32,
    value: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create sample data if CSV doesn't exist
    create_sample_data()?;
    
    // Read the CSV file
    let file = File::open("data.csv")?;
    let mut rdr = Reader::from_reader(file);
    
    let mut ids = Vec::new();
    let mut values = Vec::new();
    
    for result in rdr.deserialize() {
        let record: Record = result?;
        ids.push(record.id);
        values.push(record.value);
    }
    
    // Convert to ndarray for calculations
    let values_array = Array1::from(values.clone());
    
    // Calculate statistics
    let mean = values_array.mean().unwrap();
    let std_dev = values_array.std(1.0);
    
    println!("📊 Data Science Analysis Results:");
    println!("Data points: {}", values.len());
    println!("Mean: {:.2}", mean);
    println!("Standard Deviation: {:.2}", std_dev);
    println!("Min: {:.2}", values_array.fold(f64::INFINITY, |a, &b| a.min(b)));
    println!("Max: {:.2}", values_array.fold(f64::NEG_INFINITY, |a, &b| a.max(b)));
    
    // Create a simple bar chart
    let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let max_value = values_array.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Data Values by ID", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0f32..(ids.len() as f32), 0f64..(max_value * 1.1))?;
    
    chart.configure_mesh()
        .x_desc("ID")
        .y_desc("Value")
        .draw()?;
    
    chart.draw_series(
        ids.iter().enumerate().map(|(i, _)| {
            let value = values[i];
            Rectangle::new(
                [(i as f32, 0.0), (i as f32 + 0.8, value)],
                BLUE.filled(),
            )
        }),
    )?
    .label("Values")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));
    
    chart.configure_series_labels().draw()?;
    
    root.present()?;
    println!("📈 Chart saved as output.png");
    
    Ok(())
}

fn create_sample_data() -> Result<(), Box<dyn Error>> {
    use std::io::Write;
    
    // Check if data.csv already exists
    if std::path::Path::new("data.csv").exists() {
        return Ok(());
    }
    
    println!("📝 Creating sample data file...");
    
    let mut file = File::create("data.csv")?;
    writeln!(file, "id,value")?;
    
    // Generate sample data
    for i in 1..=20 {
        let value = (i as f64 * 1.5) + (i as f64 * 0.3).sin() * 5.0 + 10.0;
        writeln!(file, "{},{:.2}", i, value)?;
    }
    
    println!("✅ Sample data created in data.csv");
    Ok(())
}