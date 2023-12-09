use std::{thread, sync::mpsc};

use super::BChannel;

#[test]
fn test_correctly_receive_serial(){
  let mut bchannels = BChannel::new(2);

  let bchannel0: BChannel<i32> = 
    std::mem::replace(&mut bchannels[0], BChannel::empty()); 

  let bchannel1: BChannel<i32> = 
    std::mem::replace(&mut bchannels[1], BChannel::empty()); 

  bchannel0.send(0);
  assert_eq!(bchannel0.recv().unwrap(), 0);
  assert_eq!(bchannel1.recv().unwrap(), 0);
}

#[test]
fn test_correctly_receive_parallel(){
  const NUM_CHANNELS: usize = 3;

  let mut bchannels = BChannel::new(NUM_CHANNELS);
  let mut handles = Vec::with_capacity(NUM_CHANNELS);

  for i in 0..NUM_CHANNELS {
    let bchannel: BChannel<i32> = 
      std::mem::replace(&mut bchannels[i], BChannel::empty()); 

    
    let handle = thread::spawn(move || {
      if i == 0 {
        bchannel.send(0);
        println!("Sent");
      }
      let result = bchannel.recv(); 
      match result { 
        Ok(received) => {
          assert_eq!(received, 0);
        }
        Err(_) => {
          panic!("Error: The channel was closed before a message was sent");
        }
      }
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }
}

#[test]
fn test_inorder_serial(){
  let mut bchannels = BChannel::new(2);

  let bchannel0: BChannel<i32> = 
    std::mem::replace(&mut bchannels[0], BChannel::empty()); 

  let bchannel1: BChannel<i32> = 
    std::mem::replace(&mut bchannels[1], BChannel::empty()); 

  bchannel0.send(0);
  bchannel1.send(1);
  assert_eq!(bchannel0.recv().unwrap(), bchannel1.recv().unwrap());
  assert_eq!(bchannel0.recv().unwrap(), bchannel1.recv().unwrap());
}

#[test]
fn test_inorder_parallel(){
  const NUM_CHANNELS: usize = 3;

  let mut receivers = Vec::with_capacity(NUM_CHANNELS);
  let mut bchannels = BChannel::new(NUM_CHANNELS);
  let mut handles = Vec::with_capacity(NUM_CHANNELS);

  for i in 0..NUM_CHANNELS {
    let (tx, rx) = mpsc::channel();
    receivers.push(rx);

    let bchannel: BChannel<String> = 
      std::mem::replace(&mut bchannels[i], BChannel::empty()); 

    
    let handle = thread::spawn(move || {
      if i < 2 {
        let val = format!("{}", i);
        bchannel.send(String::from(val));
      }
      for _ in 0..2 {
        match bchannel.recv() {
          Ok(received) => {
            println!("{i} received: {}", received);
            let _ = tx.send(received);
          }
          Err(_) => {
            println!("Channel closed");
          }
        }
      }
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  for _ in 0..2 {
    let value = receivers[0].recv().unwrap(); 
    assert_eq!(value, receivers[1].recv().unwrap());
    assert_eq!(value, receivers[2].recv().unwrap());
  }
}
