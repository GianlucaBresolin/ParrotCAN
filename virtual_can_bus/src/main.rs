use std::time::Duration;
use tokio::time::{interval, timeout};
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registered_controllers = Arc::new(Mutex::new(Vec::new()));
    let controllers_for_pull = Arc::clone(&registered_controllers);

    // --- 1. TASK FOR REGISTERING CONTROLLERS ---
    tokio::spawn(async move {
        let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

        loop {                
            if let Ok((mut socket, _)) = listener.accept().await {
                let mut buf = [0; 1024];
                if let Ok(n) = socket.read(&mut buf).await {
                    let addr = String::from_utf8_lossy(&buf[..n]).trim().to_string();
                    {
                        let mut lock = registered_controllers.lock().unwrap();
                        lock.push(addr);
                    }
                }
            }
        }
    });

    // --- 2. PULLING SIGNALS FROM CONTROLLERS ---
    let mut ticker = interval(Duration::from_millis(1));
    let mut bus_state: u8;

    loop {
        ticker.tick().await;

        let controllers = {
            let lock = controllers_for_pull.lock().unwrap();
            lock.clone()
        };

        if controllers.is_empty() {
            bus_state = 0;
            println!("Current CAN bus state: {}", bus_state);
            continue;
        }

        let mut current_bits = Vec::new();
        for addr in controllers.iter() {
            match timeout(Duration::from_micros(500), TcpStream::connect(addr)).await {
                Ok(Ok(mut stream)) => {
                    if stream.write_all(b"R").await.is_ok() {
                        let mut buf = [0; 1];
                        if stream.read_exact(&mut buf).await.is_ok() {
                            current_bits.push(buf[0]);
                        }
                    }
                }
                _ => {
                    // Timeout or connection error: skip this controller
                    continue;
                }
            }
        }

        // Process CAN bus state 
        if !current_bits.is_empty() {
            bus_state = 
                if current_bits.iter().any(|&bit| bit == 0) {
                    0
                } else {
                    1
                };
            println!("Current CAN bus state: {}", bus_state);
        }

        // drain current_bits for next iteration
        current_bits.clear();
    }
}
