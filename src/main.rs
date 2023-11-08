use std::thread;
use std::sync::mpsc;
use std::time::Duration;

struct BChannel<T : Clone> {
   rx : mpsc::Receiver<T>,
   txs : Vec<mpsc::Sender<T>>,
}

impl<T : Clone> BChannel<T> {
    fn send(&self, data : T)  -> () {
        let cloned_senders = self.txs.clone();
        for tx in cloned_senders {
            tx.send(data.clone()).unwrap();
        }
    }

    fn recv(&self) -> T {
        self.rx.recv().unwrap()
    }
}


fn main() {
    const NUM_CHANNELS: usize = 4;

    let mut senders: Vec<mpsc::Sender<_>> = Vec::with_capacity(NUM_CHANNELS);
    let mut handles = Vec::with_capacity(NUM_CHANNELS);

    for i in 0..NUM_CHANNELS {
        let (tx, rx) = mpsc::channel();
        senders.push(tx);
        
        let handle = thread::spawn(move || {
            for received in rx {
                println!("{i} received: {}", received);
            }
        });
        handles.push(handle);
    }
    let (tx, rx) = mpsc::channel();

    let bchannel = BChannel {
        rx,
        txs : senders,
    };
        

    let handle = thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            thread::sleep(Duration::from_secs(1));
            bchannel.send(val);
        }
        let received = bchannel.recv();
        println!("Got: {}", received);
    });

    handles.push(handle);

    tx.send(String::from("Hi")).unwrap();
    for handle in handles {
        handle.join().unwrap();
    }
}
