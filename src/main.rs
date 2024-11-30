use std::sync::{mpsc, Arc, Mutex};

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

    // std::thread::spawn(move || {
    //     let cancel_token = token.clone();
    //     async move {
    //         if let Ok(()) = signal::ctrl_c().await {
    //             println!("received Ctrl-C, shutting down");
    //             cancel_token.cancel();
    //         }
    //     }
    // });

    let srvr = srvr::Server::new(cancel.clone(), r);
    let client = client::Client::new(cancel.clone(), s.clone());

    let t1 = std::thread::spawn(move || {
        srvr.run();
    });

    let t2 = std::thread::spawn(move || {
        client.run();
    });

    _ = t1.join();
    _ = t2.join();

    println!("join finished");
}
