# simple buffers

A simple cross-platform, cross-language serialization library.

# Syntax

```
enum Error {
  Ok = 0;
  InvalidRequest = 1;
}

struct MoveToEntry {
  id: u8;
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
}
```