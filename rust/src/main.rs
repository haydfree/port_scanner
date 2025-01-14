use std::sync::Arc;
use std::time::{Instant};
use tokio::sync::Semaphore;
use tokio::time::{timeout, Duration};
use tokio::sync::SemaphorePermit;
use tokio::task;
use tokio::net::TcpStream;

const PORT_NUM: u16 = 1024;
const HOST: &str = "192.168.0.1";
const MAX_THREADS: usize = 12;
const CONNECTION_TIMEOUT: Duration = Duration::from_millis(50);

async fn scan_port(port: u16, semaphore: Arc<Semaphore>) {
    let _permit: SemaphorePermit = semaphore.acquire().await.unwrap();

    let addr = format!("{}:{}", HOST, port);
    let result = timeout(CONNECTION_TIMEOUT, TcpStream::connect(&addr)).await;

    match result {
        Ok(Ok(_)) => { println!("port {} is open on {}", port, HOST); }
        Ok(Err(_)) => {}
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    let start_time = Instant::now();

    let semaphore = Arc::new(Semaphore::new(MAX_THREADS));
    let mut tasks = Vec::new();
    for port in 1..=PORT_NUM {
        let semaphore = Arc::clone(&semaphore);
        tasks.push(task::spawn(scan_port(port, semaphore)));
    }
    for task in tasks { task.await.unwrap(); }

    let end_time = Instant::now();
    let elapsed = end_time.duration_since(start_time);

    println!("execution time: {:.5} s", elapsed.as_secs_f64());
}
