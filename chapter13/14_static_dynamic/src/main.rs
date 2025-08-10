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
// Static dispatch with generics
fn draw_static<T: Draw>(item: &T) {
    item.draw();
}
// Dynamic dispatch with trait objects
fn draw_dynamic(item: &dyn Draw) {
    item.draw();
}
fn main() {
    let button = Button { label: String::from("Click me") };
    
    // Both calls work the same from the caller's perspective
    draw_static(&button);
    draw_dynamic(&button);
}
