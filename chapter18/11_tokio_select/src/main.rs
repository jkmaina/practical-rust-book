use tokio::time::{sleep, Duration};
#[tokio::main]
async fn main() {
    let mut count = 0;
    
    loop {
        tokio::select! {
            _ = sleep(Duration::from_secs(1)) => {
                println!("Tick");
                count += 1;
                if count >= 5 {
                    break;
                }
            }
            _ = tokio::signal::ctrl_c() => {
                println!("Received Ctrl+C, exiting");
                break;
            }
        }
    }
}
