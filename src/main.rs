use std::{
    sync::{mpsc, Arc, Mutex},
    thread::sleep,
    time::Duration,
};

mod client;
mod data;
mod srvr;

fn main() {
    // let (s: Sender<data::Packet>, r: Receiver<data::Packet>) = async_channel::unbounded();
    let (s, r) = mpsc::channel();

    let cancel: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    let cancelctrlc = cancel.clone();
    ctrlc::set_handler(move || {
        *cancelctrlc.lock().unwrap() = true;
    })
    .expect("Error setting Ctrl-C handler");

    let srvr = srvr::Server::new(cancel.clone(), r);
    let client = client::Client::new(cancel.clone(), s.clone());

    let t1 = std::thread::spawn(move || {
        srvr.run();
    });

    let t2 = std::thread::spawn(move || {
        client.run();
    });

    let cancel_t3 = cancel.clone();
    let t3 = std::thread::spawn(move || {
        println!("stopping in 2 seconds, or press ctrl-c");
        sleep(Duration::from_secs(2));
        *cancel_t3.lock().unwrap() = true;
    });

    _ = t1.join();
    _ = t2.join();
    _ = t3.join();

    println!("join finished");
}
