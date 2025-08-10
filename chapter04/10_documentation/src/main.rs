/// Calculates the area of a rectangle.
///
/// # Arguments
///
/// * `width` - The width of the rectangle
/// * `height` - The height of the rectangle
///
/// # Returns
///
/// The area of the rectangle (width * height)
///
/// # Examples
///
/// 
/// let area = calculate_area(5.0, 3.0);
/// assert_eq!(area, 15.0);
/// 
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

fn main() {
    // Example usage of the calculate_area function
    let width = 5.0;
    let height = 3.0;
    let area = calculate_area(width, height);
    
    println!("The area of the rectangle is: {}", area);
}
