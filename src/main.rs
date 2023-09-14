mod tokenizer;

fn main() {
    let source = r"enum Error {
        Ok = 0;
        InvalidRequest = 1;
      }
      
      // this is a comment
      struct MoveToEntry {
        id: u8; // another
        dest: f32;
        speed: f32;
      }
      
      struct MoveTo {
        entries: [MoveToEntry];
      }
      
      struct Init {
        expected_fw_version: u32;
      }
      
      union RequestPayload {
        init: Init;
        move_to: MoveTo;
      }
      
      struct Request {
        request_id: u32;
        payload: RequestPayload;
      }";

    let tokenizer = tokenizer::Tokenizer::new(source, "test").unwrap();

    for token in tokenizer {
        match token {
            Ok(token) => println!("{:?}", token),
            Err(err) => println!("{}", err),
        }
    }
}
