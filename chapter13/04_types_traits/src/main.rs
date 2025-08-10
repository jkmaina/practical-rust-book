// Demonstrates returning types that implement traits using impl Trait syntax
// - Allows functions to return concrete types while hiding the specific type from caller
// - All return paths must return the same concrete type (compile-time constraint)
// - Provides abstraction without the runtime cost of trait objects

trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
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

// Returns a type that implements Summary
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// This function can only return ONE concrete type
fn get_tweet() -> impl Summary {
    Tweet {
        username: String::from("rustlang"),
        content: String::from("Rust is awesome!"),
        reply: false,
        retweet: false,
    }
}

// This would NOT compile - can't return different types
// fn get_summary(use_tweet: bool) -> impl Summary {
//     if use_tweet {
//         Tweet { ... }
//     } else {
//         NewsArticle { ... }  // Error: different type!
//     }
// }

fn main() {
    let item = returns_summarizable();
    println!("Summary: {}", item.summarize());
    
    let tweet = get_tweet();
    println!("Tweet: {}", tweet.summarize());
    
    // The caller doesn't know the concrete type, only that it implements Summary
    println!("Type is hidden behind impl Summary");
}