# C++ API Example

Give the following `.sb` file:

```
Sequence X {
    a: u8;
    b: string;
    c: [Y];
}

Sequence Y {
    d: u8;
    e: string;
}
```

The C++ generator will generate the following code for reading and writing the `X` sequence:

```cpp
class XWriter : public SimpleBufferWriter {
public:

    // Static size of the sequence is 5 bytes.
    uint16_t size() const override {
        return 5;
    }

    // Simple constructor.
    XWriter(uint8_t a, char* b, BWriter c[], uint16_t c_len) {
        this->a = a;
        this->b = b;
        this->c = c;
        this->c_len = c_len;
    }

    // Member variables are public for easy access and modification.
    uint8_t a;
    char* b;
    BWriter c[];
    uint16_t c_len;

protected:

    // Write the sequence to the buffer.
    uint8_t* write_component_(uint8_t* dest, const uint8_t* dest_end,
                                      uint8_t* dyn_cursor) {
        if (dest_end - dest < size()) {
            return nullptr;
        }

        dest[0] = a;
        dest[1] = dyn_cursor - dest;
    }
};

class XReader {
public:

    // Constructor takes a pointer to the beginning of the serialized sequence, which is assumed
    // to be valid.
    AReader(uint8_t const* data) : _data(data) {}

    // Accessor functions will lazily read data from the serialized sequence.
    uint8_t a() const { /* ... */ }
    const char const* b() const { /* ... */ }
    const SimpleBufferArrayReader<BReader> c() const { /* ... */ }

protected:

    // Address of the beginning of the sequence. The size of the sequence is not needed, since the
    // positions of all data is either known statically or can be read from the buffer. We assume
    // that the buffer is valid and complete for the lifetime of the reader.
    uint8_t const* const _data;
};
```

The `B` sequence is generated similarly.

To write a sequence, you can use the following code:

```cpp
ArrayWriter b_writers(
    {
        BWriter(1, "b1"),
        BWriter(2, "b2")
    }, 2
);

XWriter x_writer = XWriter(5, "b str", b_writers);

uint8_t buffer[100];
int32_t bytes_written = x_writer.write(buffer, 100);

if (bytes_written < 0) {
    // Buffer is too small.
}
```

Or even simpler:

```cpp
uint8_t buffer[100];
int32_t bytes_written = XWriter(
    5,
    "b str",
    ArrayWriter({
        BWriter(1, "b1"),
        BWriter(2, "b2")
    }, 2),
).write(buffer, 100);
```

And to read a sequence:

```cpp
auto x_reader = XReader(buffer);

uint8_t x_a = x_reader.a();
const char* x_b = x_reader.b();

uint8_t b_0_d = x_reader.c()[0].d();
const char* b_0_e = x_reader.c()[0].e();

uint8_t b_1_d = x_reader.c()[1].d();
const char* b_1_e = x_reader.c()[1].e();
```
