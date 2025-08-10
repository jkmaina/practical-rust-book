trait Component {
    fn render(&self);
    fn update(&mut self);
}
struct Button {
    label: String,
    clicked: bool,
}
impl Component for Button {
    fn render(&self) {
        println!("Rendering button: {}", self.label);
    }
    
    fn update(&mut self) {
        if self.clicked {
            println!("Button {} was clicked!", self.label);
            self.clicked = false;
        }
    }
}
struct TextBox {
    content: String,
    cursor_position: usize,
}
impl Component for TextBox {
    fn render(&self) {
        println!("Rendering textbox: {}", self.content);
    }
    
    fn update(&mut self) {
        println!("Textbox cursor at position: {}", self.cursor_position);
    }
}
struct Application {
    components: Vec<Box<dyn Component>>,
}
impl Application {
    fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    
    fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }
    
    fn render(&self) {
        for component in &self.components {
            component.render();
        }
    }
    
    fn update(&mut self) {
        for component in &mut self.components {
            component.update();
        }
    }
}
fn main() {
    let mut app = Application::new();
    
    app.add_component(Box::new(Button {
        label: String::from("Submit"),
        clicked: true,
    }));
    
    app.add_component(Box::new(TextBox {
        content: String::from("Hello, world!"),
        cursor_position: 5,
    }));
    
    app.render();
    app.update();
}
