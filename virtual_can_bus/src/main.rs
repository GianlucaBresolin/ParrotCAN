use std::time::Duration;
use tokio::time::{interval};
use std::sync::{Arc};
use tokio::sync::Mutex;
use tokio::net::{TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registered_controllers = Arc::new(Mutex::new(Vec::new()));
    let controllers_for_task = Arc::clone(&registered_controllers);

    let received_bits = Arc::new(Mutex::new(Vec::new()));
    let received_bits_for_task = Arc::clone(&received_bits);

    // --- 1. TASK FOR RECEIVING ECU MESSAGES ---
    tokio::spawn(async move {
        let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
        loop {
            if let Ok((mut socket, _)) = listener.accept().await {

                let (mut read_half, mut write_half) = tokio::io::split(socket);
                let (tx, mut rx) = mpsc::unbounded_channel::<u8>();

                controllers_for_task.lock().await.push(tx);

                // read bits from ECU
                let bits = Arc::clone(&received_bits_for_task);
                tokio::spawn(async move {
                    let mut buf = [0u8; 1];
                    loop {
                        match read_half.read_exact(&mut buf).await {
                            Ok(_) if buf[0] == 0 || buf[0] == 1 => {
                                bits.lock().await.push(buf[0]);
                            }
                            _ => break,
                        }
                    }
                });

                // send bits to ECU
                tokio::spawn(async move {
                    while let Some(bit) = rx.recv().await {
                        if write_half.write_all(&[bit]).await.is_err() {
                            break;
                        }
                    }
                });
            }
        }
    });

    // --- 2. CALCULATE BUS STATE AFTER DELTA TIME ---
    let mut ticker = interval(Duration::from_millis(100));
    let mut time = 0;

    loop {
        ticker.tick().await;
        time += 1;

        // Collect all bits sent by ECUs in the last interval
        let bits: Vec<u8> = {
            let mut lock = received_bits.lock().await;
            let collected = lock.clone();
            lock.clear();
            collected
        };

        // CAN bus state: dominant (0) if any ECU sends 0, recessive (1) if all
        // send 1 or no bits
        let bus_state: u8 = if bits.iter().any(|&b| b == 0) { 0 } else { 1 };

        let mut regs = registered_controllers.lock().await;
        regs.retain(|tx| tx.send(bus_state).is_ok());

        println!("Current CAN bus state: {} (tick: {})", bus_state, time);
    }
}
