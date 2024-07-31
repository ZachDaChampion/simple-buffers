# Optimized Binary Data Serialization

SimpleBuffers provides special optimizations for handling lists of uint8_t, which is particularly
useful for sending raw binary data. This optimization uses memcpy to efficiently copy the entire
list, resulting in improved performance for large binary payloads.

## Writing Raw Binary Data

Let's extend our example schema to include a sequence for sending raw binary data:

```
sequence BinaryPayload {
    data: [u8];
    description: str;
}
```

The generated C++ code for this sequence would include:

```cpp
class BinaryPayloadWriter : public simplebuffers::SimpleBufferWriter {
public:
    BinaryPayloadWriter(simplebuffers::ListWriter<uint8_t> data, const char* description);

    simplebuffers::ListWriter<uint8_t> data;
    const char* description;

    uint16_t static_size() const override;
    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const override;
};
```

To write a `BinaryPayload` with raw binary data:

```cpp
// Prepare your raw binary data
std::vector<uint8_t> raw_data = {0x01, 0x02, 0x03, 0x04, 0x05};  // Example data

// Create a ListWriter for the raw data
simplebuffers::ListWriter<uint8_t> data_list(raw_data.data(), raw_data.size());

// Create the BinaryPayloadWriter
const char* description = "Example binary payload";
BinaryPayloadWriter payload(data_list, description);

// Serialize the payload
uint8_t buffer[1024];
int32_t bytes_written = payload.write(buffer, sizeof(buffer));

if (bytes_written > 0) {
    std::cout << "Binary payload serialized successfully. Bytes written: " << bytes_written << std::endl;
} else {
    std::cerr << "Failed to serialize binary payload." << std::endl;
}
```

## Reading Raw Binary Data

The corresponding reader for `BinaryPayload` would look like this:

```cpp
class BinaryPayloadReader : public simplebuffers::SimpleBufferReader {
public:
    BinaryPayloadReader(const uint8_t* data_ptr, size_t idx = 0);

    simplebuffers::ListReader<uint8_t> data() const;
    const char* description() const;

    uint16_t static_size() const override;
};
```

To read the serialized `BinaryPayload`:

```cpp
BinaryPayloadReader reader(buffer);

// Access the raw binary data
auto data = reader.data();
std::cout << "Raw data size: " << data.len() << " bytes" << std::endl;

// You can access individual bytes if needed
for (uint16_t i = 0; i < data.len(); ++i) {
    std::cout << "Byte " << i << ": 0x" << std::hex << static_cast<int>(data[i]) << std::dec << std::endl;
}

// Or you can work with the entire data buffer directly
const uint8_t* raw_data_ptr = data.data();
size_t raw_data_size = data.len();

// Access the description
std::cout << "Description: " << reader.description() << std::endl;
```

The `ListReader<uint8_t>` provides a `data()` method that returns a pointer to the raw data buffer,
allowing for efficient access to the entire binary payload without copying.

This optimized handling of uint8_t lists allows SimpleBuffers to efficiently serialize and
deserialize raw binary data, making it suitable for applications that need to transmit or store
binary blobs alongside structured data.
