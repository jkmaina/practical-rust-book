trait Draw {
    fn draw(&self);
}
struct Button {
    label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button with label: {}", self.label);
    }
}
struct Image {
    path: String,
}
impl Draw for Image {
    fn draw(&self) {
        println!("Drawing an image from: {}", self.path);
    }
}
fn main() {
    let button = Button { label: String::from("Click me") };
    let image = Image { path: String::from("landscape.png") };
    
    // Creating trait objects
    let ui_elements: Vec<Box<dyn Draw>> = vec![
        Box::new(button),
        Box::new(image),
    ];
    
    for element in ui_elements {
        element.draw();
    }
}
