#include "simplebuffers.hpp"

#include <cstring>

namespace simplebuffers {

bool read_bool(uint8_t const* addr) { return *addr != 0; }

uint8_t read_uint8(uint8_t const* addr) { return *addr; }

uint16_t read_uint16(uint8_t const* addr) {
    return
        (static_cast<uint16_t>(*addr << 8)) |
        *(addr + 1);
}

uint32_t read_uint32(uint8_t const* addr) {
    return
        (static_cast<uint32_t>(*addr)       << 24) |
        (static_cast<uint32_t>(*(addr + 1)) << 16) |
        (static_cast<uint16_t>(*(addr + 2)) << 8 ) |
        *(addr + 3);
}

uint64_t read_uint64(uint8_t const* addr) {
    return 
        (static_cast<uint64_t>(*addr)       << 56) |
        (static_cast<uint64_t>(*(addr + 1)) << 48) |
        (static_cast<uint64_t>(*(addr + 2)) << 40) |
        (static_cast<uint64_t>(*(addr + 3)) << 32) |
        (static_cast<uint32_t>(*(addr + 4)) << 24) |
        (static_cast<uint32_t>(*(addr + 5)) << 16) |
        (static_cast<uint16_t>(*(addr + 6)) << 8 ) |
        *(addr + 7);
}

int8_t read_int8(uint8_t const* addr) { return *addr; }

int16_t read_int16(uint8_t const* addr) { return static_cast<int16_t>(read_uint16(addr)); }

int32_t read_int32(uint8_t const* addr) { return static_cast<int32_t>(read_uint32(addr)); }

int64_t read_int64(uint8_t const* addr) { return static_cast<int64_t>(read_uint64(addr)); }

float read_float(uint8_t const* addr) {
    float ret;
    uint32_t tmp = read_uint32(addr);
    memcpy(&ret, &tmp, sizeof(float));
    return ret;
}

double read_double(uint8_t const* addr) {
    double ret;
    uint64_t tmp = read_uint64(addr);
    memcpy(&ret, &tmp, sizeof(double));
    return ret;
}

char const* read_string(uint8_t const* addr) {
    uint16_t offset = read_uint16(addr);
    return reinterpret_cast<char const*>(addr + offset);
}

}  // namespace simplebuffers
