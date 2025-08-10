// Media library example demonstrating traits with default implementations
// Shows polymorphism across different media types (Song, Movie, Podcast)

trait MediaItem {
    fn title(&self) -> &str;
    fn creator(&self) -> &str;
    fn duration(&self) -> u32;  // Duration in seconds
    fn description(&self) -> String {
        format!("{} by {} ({} seconds)", self.title(), self.creator(), self.duration())
    }
}
struct Song {
    name: String,
    artist: String,
    album: String,
    length: u32,
}
impl MediaItem for Song {
    fn title(&self) -> &str {
        &self.name
    }
    
    fn creator(&self) -> &str {
        &self.artist
    }
    
    fn duration(&self) -> u32 {
        self.length
    }
    
    fn description(&self) -> String {
        format!("Song: {} by {} from the album {} ({} seconds)", 
                self.name, self.artist, self.album, self.length)
    }
}
struct Movie {
    title_name: String,
    director: String,
    release_year: u32,
    runtime: u32,
}
impl MediaItem for Movie {
    fn title(&self) -> &str {
        &self.title_name
    }
    
    fn creator(&self) -> &str {
        &self.director
    }
    
    fn duration(&self) -> u32 {
        self.runtime
    }
    
    fn description(&self) -> String {
        format!("Movie: {} directed by {} ({}) - {} seconds", 
                self.title_name, self.director, self.release_year, self.runtime)
    }
}
struct Podcast {
    episode_title: String,
    host: String,
    episode_number: u32,
    duration_secs: u32,
}
impl MediaItem for Podcast {
    fn title(&self) -> &str {
        &self.episode_title
    }
    
    fn creator(&self) -> &str {
        &self.host
    }
    
    fn duration(&self) -> u32 {
        self.duration_secs
    }
}
// A function that works with any media item
fn play_media(item: &impl MediaItem) {
    println!("Now playing: {}", item.description());
    println!("Duration: {} seconds", item.duration());
}
fn main() {
    let song = Song {
        name: String::from("Bohemian Rhapsody"),
        artist: String::from("Queen"),
        album: String::from("A Night at the Opera"),
        length: 354,
    };
    
    let movie = Movie {
        title_name: String::from("The Matrix"),
        director: String::from("The Wachowskis"),
        release_year: 1999,
        runtime: 8160,  // 2 hours 16 minutes in seconds
    };
    
    let podcast = Podcast {
        episode_title: String::from("Rust for Beginners"),
        host: String::from("Rusty Developer"),
        episode_number: 42,
        duration_secs: 1800,  // 30 minutes in seconds
    };
    
    play_media(&song);
    play_media(&movie);
    play_media(&podcast);  // Uses the default description implementation
    
    println!("\n=== Media Library Summary ===");
    let media_items: Vec<&dyn MediaItem> = vec![&song, &movie, &podcast];
    for item in media_items {
        println!("{}", item.description());
    }
}
 
