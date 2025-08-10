// Trait bounds example showing different syntax approaches

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Using impl Trait syntax
fn notify_impl(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Using trait bounds syntax
fn notify_bounds<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple parameters - impl Trait allows different types
fn notify_different(item1: &impl Summary, item2: &impl Summary) {
    println!("Item 1: {}", item1.summarize());
    println!("Item 2: {}", item2.summarize());
}

// Multiple parameters - trait bounds require same type
fn notify_same<T: Summary>(item1: &T, item2: &T) {
    println!("Item 1: {}", item1.summarize());
    println!("Item 2: {}", item2.summarize());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 1.70 Released!"),
        author: String::from("Rust Team"),
    };
    
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("New Rust version is out!"),
    };
    
    println!("=== Using impl Trait ===");
    notify_impl(&article);
    notify_impl(&tweet);
    
    println!("\n=== Using trait bounds ===");
    notify_bounds(&article);
    notify_bounds(&tweet);
    
    println!("\n=== Different types allowed ===");
    notify_different(&article, &tweet);  // Works - different types OK
    
    println!("\n=== Same type required ===");
    notify_same(&article, &article);     // Works - same type
    // notify_same(&article, &tweet);    // Would NOT compile - different types
}