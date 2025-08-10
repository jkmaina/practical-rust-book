// Parallel image processing example demonstrating thread-based workload distribution
// Shows how to divide computational work across multiple threads for performance
// Simulates image blur processing without external dependencies
// Demonstrates Arc<Mutex<T>> for shared output and work partitioning strategies

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

// Simple RGB pixel representation
#[derive(Clone, Copy, Debug)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

// Simple image representation
#[derive(Debug)]
struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl Image {
    fn new(width: u32, height: u32) -> Self {
        let pixels = vec![Pixel { r: 0, g: 0, b: 0 }; (width * height) as usize];
        Image { width, height, pixels }
    }
    
    fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        let index = (y * self.width + x) as usize;
        self.pixels[index]
    }
    
    fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        let index = (y * self.width + x) as usize;
        self.pixels[index] = pixel;
    }
    
    // Create a sample image with gradient pattern
    fn create_sample(width: u32, height: u32) -> Self {
        let mut img = Image::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let r = ((x * 255) / width) as u8;
                let g = ((y * 255) / height) as u8;
                let b = ((x + y) * 255 / (width + height)) as u8;
                img.set_pixel(x, y, Pixel { r, g, b });
            }
        }
        img
    }
    
    // Save image as BMP format (widely supported bitmap format)
    fn save_bmp(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let mut file = File::create(filename)?;
        
        // BMP file header (14 bytes)
        let file_size = 54 + (self.width * self.height * 3); // Header + pixel data
        file.write_all(b"BM")?; // Signature
        file.write_all(&file_size.to_le_bytes())?; // File size
        file.write_all(&[0, 0, 0, 0])?; // Reserved
        file.write_all(&54u32.to_le_bytes())?; // Offset to pixel data
        
        // DIB header (40 bytes)
        file.write_all(&40u32.to_le_bytes())?; // DIB header size
        file.write_all(&self.width.to_le_bytes())?; // Width
        file.write_all(&self.height.to_le_bytes())?; // Height
        file.write_all(&1u16.to_le_bytes())?; // Color planes
        file.write_all(&24u16.to_le_bytes())?; // Bits per pixel
        file.write_all(&[0; 24])?; // Compression, image size, etc. (all zeros)
        
        // Pixel data (BGR format, bottom-to-top)
        let row_padding = (4 - (self.width * 3) % 4) % 4;
        for y in (0..self.height).rev() { // BMP stores rows bottom-to-top
            for x in 0..self.width {
                let pixel = self.get_pixel(x, y);
                file.write_all(&[pixel.b, pixel.g, pixel.r])?; // BGR order
            }
            // Add row padding to align to 4-byte boundary
            for _ in 0..row_padding {
                file.write_all(&[0])?;
            }
        }
        
        Ok(())
    }
}

fn main() {
    println!("=== Parallel Image Processing Example ===");
    
    // Create a sample image (simulating loaded image)
    let width = 800;
    let height = 600;
    let img = Image::create_sample(width, height);
    println!("Created sample image: {}x{}", width, height);
    
    // Create output image wrapped in Arc<Mutex<T>> for thread-safe access
    let output = Arc::new(Mutex::new(Image::new(width, height)));
    
    // Determine number of threads and work distribution
    let num_threads = 4;
    let rows_per_thread = height / num_threads;
    println!("Using {} threads, {} rows per thread", num_threads, rows_per_thread);
    
    let start = Instant::now();
    
    // Spawn threads to process different parts of the image
    let mut handles = vec![];
    
    for i in 0..num_threads {
        // Clone Arc for each thread
        let output = Arc::clone(&output);
        
        // Calculate this thread's work range
        let start_row = i * rows_per_thread;
        let end_row = if i == num_threads - 1 {
            height // Last thread handles remaining rows
        } else {
            (i + 1) * rows_per_thread
        };
        
        println!("Thread {} will process rows {} to {}", i, start_row, end_row - 1);
        
        // Clone image data for this thread (in real app, would share read-only data)
        let img_clone = img.pixels.clone();
        
        let handle = thread::spawn(move || {
            println!("Thread {} started processing", i);
            
            // Process each pixel in this thread's section
            for y in start_row..end_row {
                for x in 0..width {
                    // Apply blur filter to current pixel
                    let blurred_pixel = apply_blur(&img_clone, x, y, width, height);
                    
                    // Lock output image and update pixel
                    // Note: In real implementation, would batch updates to reduce lock contention
                    let mut output = output.lock().unwrap();
                    output.set_pixel(x, y, blurred_pixel);
                }
            }
            
            println!("Thread {} finished processing", i);
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for (i, handle) in handles.into_iter().enumerate() {
        handle.join().unwrap();
        println!("Thread {} joined", i);
    }
    
    let duration = start.elapsed();
    println!("\nParallel processing completed in: {:?}", duration);
    
    // Extract final result
    let output = Arc::try_unwrap(output).unwrap().into_inner().unwrap();
    println!("Processed {} pixels total", output.pixels.len());
    
    // Save the original and processed images
    println!("\nSaving images to disk...");
    if let Err(e) = img.save_bmp("original.bmp") {
        println!("Failed to save original image: {}", e);
    } else {
        println!("Original image saved as: original.bmp");
    }
    
    if let Err(e) = output.save_bmp("blurred.bmp") {
        println!("Failed to save processed image: {}", e);
    } else {
        println!("Processed image saved as: blurred.bmp");
        println!("You can open these .bmp files in any image viewer!");
    }
    
    println!("\n=== Parallel Processing Benefits ===");
    println!("1. Work distribution - each thread processes different image regions");
    println!("2. CPU utilization - multiple cores work simultaneously");
    println!("3. Scalability - can adjust thread count based on hardware");
    println!("4. Load balancing - work divided evenly among threads");
    println!("5. Performance - processing time reduced by parallelization");
}

// Apply a simple box blur filter to a pixel
fn apply_blur(pixels: &[Pixel], x: u32, y: u32, width: u32, height: u32) -> Pixel {
    let mut r_sum = 0u32;
    let mut g_sum = 0u32;
    let mut b_sum = 0u32;
    let mut count = 0u32;
    
    // Define blur kernel size (3x3)
    let kernel_size = 3;
    let half_kernel = kernel_size / 2;
    
    // Apply blur kernel around the current pixel
    for ky in 0..kernel_size {
        for kx in 0..kernel_size {
            let px = x as i32 + kx as i32 - half_kernel as i32;
            let py = y as i32 + ky as i32 - half_kernel as i32;
            
            // Check if the sample pixel is within image bounds
            if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                let index = (py as u32 * width + px as u32) as usize;
                let pixel = pixels[index];
                
                r_sum += pixel.r as u32;
                g_sum += pixel.g as u32;
                b_sum += pixel.b as u32;
                count += 1;
            }
        }
    }
    
    // Calculate average color values
    let r = (r_sum / count) as u8;
    let g = (g_sum / count) as u8;
    let b = (b_sum / count) as u8;
    
    Pixel { r, g, b }
}
