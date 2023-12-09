use super::hashtag_processor;

#[test]
fn test_correct_length(){

  let bchannels = hashtag_processor::<i32>(2, 2);
  assert_eq!(bchannels.len(), 4);
}

#[test]
fn test_correct_connection(){
  let bchannels = hashtag_processor::<i32>(2, 2);
  // Check that horizontal broadcast works
  bchannels[0][0].send(1);
  assert_eq!(bchannels[0][0].recv().unwrap(), 1);
  assert_eq!(bchannels[1][0].recv().unwrap(), 1);

  bchannels[2][0].send(2);
  assert_eq!(bchannels[2][0].recv().unwrap(), 2);
  assert_eq!(bchannels[3][0].recv().unwrap(), 2);

  // Check that vertical broadcast works
  bchannels[0][1].send(3);
  assert_eq!(bchannels[0][1].recv().unwrap(), 3);
  assert_eq!(bchannels[2][1].recv().unwrap(), 3);

  bchannels[1][1].send(4);
  assert_eq!(bchannels[1][1].recv().unwrap(), 4);
  assert_eq!(bchannels[3][1].recv().unwrap(), 4);
  
}
