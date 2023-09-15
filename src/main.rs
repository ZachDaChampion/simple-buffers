const SOURCE: &str = r"enum Error {
  Ok = 0;
  InvalidRequest = 1;
}

// this is a comment
sequence MoveToEntry {
  id: u8; // another
  dest: f32;
  speed: f32;
}

sequence MoveTo {
  entries: [MoveToEntry];
}

sequence Init {
  expected_fw_version: u32;
}

sequence Request {
  request_id: u32;
  payload: oneof {
    init: Init;
    move_to: MoveTo;
  };
}";

mod tokenizer;

fn main() {
    let tokenizer = tokenizer::Tokenizer::new(SOURCE, "test").unwrap();

    for token in tokenizer {
        match token {
            Ok(token) => println!("{:?}", token),
            Err(err) => println!("{}", err),
        }
    }
}
