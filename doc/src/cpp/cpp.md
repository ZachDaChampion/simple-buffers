# Generated C++ API

The SimpleBuffers compiler generates C++ code that provides a convenient API for serializing and
deserializing data structures defined in the schema. This section describes the main components of
the generated API and how to use them.

## Writers

For each sequence defined in the schema, the compiler generates a corresponding `Writer` class.
These classes are used to construct and serialize data.

### Sequence Writers

For example, given the `Request` sequence from our schema:

```
sequence Request {
    id: u32;
    payload: oneof {
        init: Init;
        moveTo: MoveTo;
    };
}
```

The compiler generates a `RequestWriter` class:

```cpp
class RequestWriter : public simplebuffers::SimpleBufferWriter {
public:
    RequestWriter(uint32_t id, PayloadWriter payload);

    uint32_t id;
    PayloadWriter payload;

    uint16_t static_size() const override;
    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const override;
};
```

To create and serialize a `Request`:

```cpp
InitWriter init_payload(firmware_version);
RequestWriter::PayloadWriter payload = RequestWriter::PayloadWriter::init(&init_payload);
RequestWriter request(request_id, payload);

uint8_t buffer[1024];
int32_t bytes_written = request.write(buffer, sizeof(buffer));
```

### OneOf Writers

For OneOf fields, the compiler generates nested classes. In the `Request` example, there's a
`PayloadWriter` nested class:

```cpp
class RequestWriter::PayloadWriter : public simplebuffers::OneOfWriter {
public:
    enum class Tag : uint8_t {
        INIT = 0,
        MOVE_TO = 1
    };

    static PayloadWriter init(InitWriter* val);
    static PayloadWriter move_to(MoveToWriter* val);

    // ... other methods ...
};
```

### List Writers

For list fields, the compiler generates a `ListWriter` specialization:

```cpp
class MoveToWriter : public simplebuffers::SimpleBufferWriter {
public:
    MoveToWriter(simplebuffers::ListWriter<MoveToEntryWriter> joints);

    simplebuffers::ListWriter<MoveToEntryWriter> joints;

    // ... other methods ...
};
```

To create a list:

```cpp
std::vector<MoveToEntryWriter> entries = { /* ... */ };
simplebuffers::ListWriter<MoveToEntryWriter> joints_list(entries.data(), entries.size());
MoveToWriter move_to(joints_list);
```

## Readers

For each sequence, the compiler also generates a corresponding `Reader` class for deserialization.

### Sequence Readers

Continuing with the `Request` example:

```cpp
class RequestReader : public simplebuffers::SimpleBufferReader {
public:
    RequestReader(const uint8_t* data_ptr, size_t idx = 0);

    uint32_t id() const;
    PayloadReader payload() const;

    // ... other methods ...
};
```

To read a serialized `Request`:

```cpp
RequestReader reader(buffer);
uint32_t id = reader.id();
RequestReader::PayloadReader payload = reader.payload();
```

### OneOf Readers

For OneOf fields, the compiler generates nested reader classes:

```cpp
class RequestReader::PayloadReader : public simplebuffers::OneOfReader {
public:
    enum class Tag : uint8_t {
        INIT = 0,
        MOVE_TO = 1
    };

    PayloadReader(const uint8_t* data_ptr, size_t idx = 0);
    Tag tag() const;
    InitReader init() const;
    MoveToReader move_to() const;

    // ... other methods ...
};
```

To read a OneOf field:

```cpp
RequestReader::PayloadReader payload = reader.payload();
switch (payload.tag()) {
    case RequestReader::PayloadReader::Tag::INIT:
        InitReader init = payload.init();
        // Process init...
        break;
    case RequestReader::PayloadReader::Tag::MOVE_TO:
        MoveToReader move_to = payload.move_to();
        // Process move_to...
        break;
}
```

### List Readers

For list fields, the compiler generates a `ListReader` specialization:

```cpp
class MoveToReader : public simplebuffers::SimpleBufferReader {
public:
    MoveToReader(const uint8_t* data_ptr, size_t idx = 0);
    simplebuffers::ListReader<MoveToEntryReader> joints() const;

    // ... other methods ...
};
```

To read a list:

```cpp
MoveToReader move_to_reader = payload.move_to();
auto joints = move_to_reader.joints();
for (uint16_t i = 0; i < joints.len(); ++i) {
    MoveToEntryReader entry = joints[i];
    // Process entry...
}
```

## Enums

For each enum defined in the schema, the compiler generates a corresponding C++ enum class:

```cpp
enum class RobotJoint : uint_fast8_t {
    J_0 = 0,
    J_1 = 1,
    J_2 = 2,
    J_3 = 3,
    J_4 = 4,
    J_5 = 5
};
```

These enum classes can be used directly in your C++ code and are automatically handled by the
generated Writer and Reader classes.

This API design allows for efficient serialization and deserialization of data structures defined in
the SimpleBuffers schema, with a focus on performance and ease of use in C++ applications.
