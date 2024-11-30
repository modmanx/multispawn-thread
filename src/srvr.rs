use std::sync::{mpsc::Receiver, Arc, Mutex};

use crate::data::Packet;

pub struct Server {
    token: Arc<Mutex<bool>>,
    r: Receiver<Packet>,
}

impl Server {
    pub fn new(token: Arc<Mutex<bool>>, r: Receiver<Packet>) -> Self {
        Server { token, r }
    }

    pub fn run(&self) {
        println!("running srvr");
        let pcount: Arc<Mutex<u64>> = Arc::default();
        let token = self.token.clone();
        let pcount1 = pcount.clone();
        let cancelsignal = self.token.clone();
        loop {
            {
                let _data = self.r.recv().unwrap();
            }
            {
                let mut lll = pcount1.lock().unwrap();
                *lll += 1;

                if *lll > 20_000_000 {
                    *token.lock().unwrap() = true;
                    break;
                }

                if *cancelsignal.lock().unwrap() {
                    break;
                }
            }
        }

        let pcount2 = pcount.clone();
        println!("Total recv: {}", pcount2.lock().unwrap());
    }
}
