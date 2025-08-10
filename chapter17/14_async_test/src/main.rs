use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
pub async fn fetch_data(url: &str) -> Result<String, String> {
    // In a real application, this would make an HTTP request
    // For this example, we'll just simulate it
    if url == "https://example.com" {
        Ok(String::from("Example data"))
    } else {
        Err(String::from("Failed to fetch data"))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::future::Future;
    #[test]
    fn test_fetch_data_success() {
        let future = fetch_data("https://example.com");
        let result = futures::executor::block_on(future);
        assert_eq!(result, Ok(String::from("Example data")));
    }
    #[test]
    fn test_fetch_data_failure() {
        let future = fetch_data("https://nonexistent.com");
        let result = futures::executor::block_on(future);
        assert_eq!(result, Err(String::from("Failed to fetch data")));
    }
}
