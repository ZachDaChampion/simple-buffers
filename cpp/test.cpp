#include <cstdint>
#include <iostream>

#include "simplebuffers.hpp"

using namespace std;
using namespace simplebuffers;

/*

Sequence X {
    a: u8;
    b: string;
    c: [Y];
    f: Y;
}

Sequence Y {
    d: u8;
    e: string;
}

*/

class XWriter;
class YWriter;

class XWriter : public SimpleBufferWriter {
   public:
    uint16_t size() const override { return 7; }
    XWriter(uint8_t a, char* b, ArrayWriter<YWriter> c) : a(a), b(b), c(c) {}

    uint8_t a;
    char* b;
    ArrayWriter<YWriter> c;

    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                             uint8_t* dyn_cursor = nullptr) const override {
        if (dest_end - dest < 7) return nullptr;
        if (dyn_cursor == nullptr) dyn_cursor = dest + 7;
        dyn_cursor = write_field(dest, dest_end, dyn_cursor, a);
        dest += get_static_size(a);
        dyn_cursor = write_field(dest, dest_end, dyn_cursor, b);
        dest += get_static_size(b);
        dyn_cursor = write_field(dest, dest_end, dyn_cursor, c);
        return dyn_cursor;
    }
};

class YWriter : public SimpleBufferWriter {
   public:
    uint16_t size() const override { return 3; }
    YWriter(uint8_t d, char* e) : d(d), e(e) {}

    uint8_t d;
    char* e;

    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                             uint8_t* dyn_cursor = nullptr) const override {
        if (dest_end - dest < 2) return nullptr;
        if (dyn_cursor == nullptr) dyn_cursor = dest + 2;
        dyn_cursor = write_field(dest, dest_end, dyn_cursor, d);
        dest += get_static_size(d);
        dyn_cursor = write_field(dest, dest_end, dyn_cursor, e);
        return dyn_cursor;
    }
};

uint8_t buffer[100];

int main() {
    YWriter c_writers[] = {YWriter(1, "c1"), YWriter(2, "c2")};
    XWriter x_writer(5, "b str", ArrayWriter(c_writers, 2));

    int32_t bytes_written = x_writer.write(buffer, 100);

    cout << "bytes written: " << bytes_written << endl;
    for (int i = 0; i < bytes_written; i++) {
        if (buffer[i] >= 32 && buffer[i] <= 126)
            cout << "'" << (char)buffer[i] << "' | ";
        else
            cout << (int)buffer[i] << " | ";
    }

    return 0;
}

/*

5 | 0 4 | 0 8 | 'b' ' ' 's' 't' 'r' 0 | 0 2 | 0 2 | 1 | 0 5 | 2 | 0 5 | 'b' '1' 0 | 'b' '2' 0

*/