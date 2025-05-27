use std::collections::HashMap;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let server = TcpListener::bind(LOCAL).expect("Failed to bind server");
    server
        .set_nonblocking(true)
        .expect("Failed to set non-blocking");

    let clients = Arc::new(Mutex::new(HashMap::new()));
    let (tx, rx) = mpsc::channel::<(String, String)>(); // (name, msg)

    loop {
        // Accept new clients
        if let Ok((mut socket, addr)) = server.accept() {
            let tx = tx.clone();
            let clients = Arc::clone(&clients);

            thread::spawn(move || {
                // Read client name
                let mut name_buff = vec![0; MSG_SIZE];
                let name = match socket.read_exact(&mut name_buff) {
                    Ok(_) => {
                        let name = name_buff
                            .into_iter()
                            .take_while(|&x| x != 0)
                            .collect::<Vec<_>>();
                        String::from_utf8(name).unwrap_or("unknown".to_string())
                    }
                    Err(_) => "unknown".to_string(),
                };
                println!("Client {} set name: {}", addr, name);

                // Add client to the list
                clients.lock().unwrap().insert(
                    name.clone(),
                    socket.try_clone().expect("Failed to clone client"),
                );

                tx.send((name.clone(), "joined the chat".to_string())).ok();

                loop {
                    let mut buff = vec![0; MSG_SIZE];
                    match socket.read_exact(&mut buff) {
                        Ok(_) => {
                            let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                            let msg = String::from_utf8(msg).unwrap_or_default();
                            println!("{} ({}): {}", addr, name, msg);
                            tx.send((name.clone(), msg)).ok();
                        }
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(_) => {
                            println!("Client {} ({}) disconnected", addr, name);
                            clients.lock().unwrap().remove(&name);
                            tx.send((name.clone(), "left the chat".to_string())).ok();
                            break;
                        }
                    }
                    sleep();
                }
            });
        }

        // Broadcast messages to all clients
        if let Ok((name, msg)) = rx.try_recv() {
            let full_msg = format!("{}: {}", name, msg);
            let mut clients_guard = clients.lock().unwrap();
            let mut disconnected = vec![];

            for (client_name, client) in clients_guard.iter_mut() {
                let mut buff = full_msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                if let Err(_) = client.write_all(&buff) {
                    disconnected.push(client_name.clone());
                }
            }
            // Remove disconnected clients
            for name in disconnected {
                clients_guard.remove(&name);
            }
        }
        sleep();
    }
}

fn sleep() {
    thread::sleep(std::time::Duration::from_millis(100));
}
