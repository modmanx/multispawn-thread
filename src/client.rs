use std::sync::{mpsc::Sender, Arc, Mutex};

use crate::data::Packet;

pub struct Client {
    token: Arc<Mutex<bool>>,
    s: Sender<Packet>,
}

impl Client {
    pub fn new(token: Arc<Mutex<bool>>, s: Sender<Packet>) -> Self {
        Client { token, s }
    }
    pub fn run(&self) {
        println!("running client");

        let token1 = self.token.clone();
        let s1 = self.s.clone();
        let t1 = std::thread::spawn(move || loop {
            let _ = s1.send(Packet {
                data_str: String::from("hello from client 1"),
            });
            if *token1.lock().unwrap() {
                break;
            }
        });

        let token2 = self.token.clone();
        let s2 = self.s.clone();
        let t2 = std::thread::spawn(move || loop {
            let _ = s2.send(Packet {
                data_str: String::from("hello from client 2"),
            });
            if *token2.lock().unwrap() {
                break;
            }
        });

        _ = t1.join();
        _ = t2.join();
    }
}
