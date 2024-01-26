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

class ZWriter;

class XWriter : public SimpleBufferWriter {
   public:
    XWriter(uint8_t a, char* b, ArrayWriter<YWriter> c) : a(a), b(b), c(c) {}

    uint8_t a;
    char* b;
    ArrayWriter<YWriter> c;

    uint16_t static_size() const override { return 5; }

    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                             uint8_t* dyn_cursor) const override {
        if (dest_end - dest < 7) return nullptr;
        dyn_cursor = write_field(dest, dest_end, dyn_cursor, a);
        if (dyn_cursor == nullptr) return nullptr;
        dest += get_static_size(a);
        dyn_cursor = write_field(dest, dest_end, dyn_cursor, b);
        if (dyn_cursor == nullptr) return nullptr;
        dest += get_static_size(b);
        dyn_cursor = write_field(dest, dest_end, dyn_cursor, c);
        if (dyn_cursor == nullptr) return nullptr;
        return dyn_cursor;
    }
};

class YWriter : public SimpleBufferWriter {
   public:
    YWriter(uint8_t d, char* e) : d(d), e(e) {}

    uint8_t d;
    char* e;

    uint16_t static_size() const override { return 3; }

    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                             uint8_t* dyn_cursor) const override {
        if (dest_end - dest < 2) return nullptr;
        dyn_cursor = write_field(dest, dest_end, dyn_cursor, d);
        if (dyn_cursor == nullptr) return nullptr;
        dest += get_static_size(d);
        dyn_cursor = write_field(dest, dest_end, dyn_cursor, e);
        if (dyn_cursor == nullptr) return nullptr;
        return dyn_cursor;
    }
};

class ZWriter : public SimpleBufferWriter {
   public:
    enum class Tag : uint8_t {
        X = 0,
        Y = 1,
    };

    union Type {
        XWriter x;
        YWriter y;
    };

    ZWriter(Tag tag, const Type* val) : tag(tag), val(val) {}

    Tag tag;
    const Type* val;

    uint16_t static_size() const override { return 3; }

    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                             uint8_t* dyn_cursor) const override {
        if (dest_end - dest < 2) return nullptr;
        switch (tag) {
            case Tag::X:
                dyn_cursor = write_oneof_field(dest, dest_end, dyn_cursor, (uint8_t)tag, val->x);
                break;
            case Tag::Y:
                dyn_cursor = write_oneof_field(dest, dest_end, dyn_cursor, (uint8_t)tag, val->y);
                break;
        }
        return dyn_cursor;
    }
};

uint8_t buffer[512];

int main() {
    YWriter c_writers[] = {YWriter(1, "c1"), YWriter(2, "c2")};
    XWriter x_writer(5, "b str", ArrayWriter<YWriter>(c_writers, 2));
    ZWriter z_writer(ZWriter::Tag::X, (const ZWriter::Type*)&x_writer);

    int32_t bytes_written = z_writer.write(buffer, 512);

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