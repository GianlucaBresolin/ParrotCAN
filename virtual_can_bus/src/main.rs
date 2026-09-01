use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use std::sync::{Arc};
use tokio::sync::{Mutex, Notify};
use tokio::net::{TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registered_controllers = Arc::new(Mutex::new(HashMap::new()));
    let controllers_for_task = Arc::clone(&registered_controllers);

    let received_bits = Arc::new(Mutex::new(HashMap::new()));
    let received_bits_for_task = Arc::clone(&received_bits);

    let notify = Arc::new(Notify::new());
    let notify_for_task = Arc::clone(&notify);

    // 1. TASK FOR REGISTERING ECUS
    tokio::spawn(async move {
        let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
        let mut next_id: usize = 0;

        loop {
            if let Ok((socket, _)) = listener.accept().await {
                let id = next_id;
                next_id += 1;

                let (mut read_half, mut write_half) = tokio::io::split(socket);
                let (tx, mut rx) = mpsc::unbounded_channel::<u8>();

                controllers_for_task.lock().await.insert(id, tx);

                let controllers_for_reader = Arc::clone(&controllers_for_task);
                let notify_for_reader = Arc::clone(&notify_for_task);
                let bits = Arc::clone(&received_bits_for_task);
                // TASK HANDLER FOR READING ECU BITs
                tokio::spawn(async move {
                    let mut buf = [0u8; 1];
                    loop {
                        match read_half.read_exact(&mut buf).await {
                            Ok(_) if buf[0] == 0 || buf[0] == 1 => {
                                let mut bits_lock = bits.lock().await;
                                bits_lock.insert(id, buf[0]);

                                // checks if it was the last bit waited: only a signal, no calculation here
                                let registered_count = controllers_for_reader.lock().await.len();
                                if bits_lock.len() == registered_count {
                                    notify_for_reader.notify_one();
                                }
                            }
                            _ => {
                                // connection lost, remove the controller
                                controllers_for_reader.lock().await.remove(&id);
                                bits.lock().await.remove(&id);

                                // now received bits might be complete: wake up to check
                                notify_for_reader.notify_one();
                                break;
                            }
                        }
                    }
                });

                // TASK HANDLER FOR WRITING BUS STATE TO ECU
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

    // 2. CALCULATE BUS STATE AFTER DELTA TIME
    let mut time = 0;
    loop {
        let registered_count = registered_controllers.lock().await.len();

        // No ECUs registered: default
        if registered_count == 0 {
            sleep(Duration::from_millis(1000)).await;
            time += 1;
            println!("Current CAN bus state: 1 (tick: {})", time);
            continue;
        }

        // wakes up on set-complete signal OR after 2s (unlock deadlock for ECUs waiting on tx)
        tokio::select! {
            _ = notify.notified() => {},
            _ = sleep(Duration::from_millis(2000)) => {},
        }

        let bus_state: u8 = {
            let bits = received_bits.lock().await;
            if bits.is_empty() {
                // unlock ecu deadlock for first transmission
                1
            } else if bits.values().any(|&b| b == 0) {
                0 
            } else {
                1 
            }
        };

        time += 1;

        // Go back to rx again
        received_bits.lock().await.clear();

        println!("Current CAN bus state: {} (tick: {})", bus_state, time);
        sleep(Duration::from_millis(20)).await;

        let mut regs = registered_controllers.lock().await;
        regs.retain(|_, tx| tx.send(bus_state).is_ok());
    }
}