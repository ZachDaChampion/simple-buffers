#ifndef SIMPLEBUFFERS__SIMPLEBUFFERS__ZACHDACHAMPION__HPP
#define SIMPLEBUFFERS__SIMPLEBUFFERS__ZACHDACHAMPION__HPP

#include <cstdint>

namespace simplebuffers {

/**
 * @brief Reads a boolean value from the given address.
 *
 * @param addr The address to read from.
 * @return true If the value at the address is non-zero.
 * @return false If the value at the address is zero.
 */
bool read_bool(uint8_t const* addr);

/**
 * @brief Reads an unsigned 8-bit integer value from the given address.
 *
 * @param addr The address to read from.
 * @return uint8_t The value at the address.
 */
uint8_t read_uint8(uint8_t const* addr);

/**
 * @brief Reads an unsigned 16-bit integer value from the given address.
 *
 * @param addr The address to read from.
 * @return uint16_t The value at the address.
 */
uint16_t read_uint16(uint8_t const* addr);

/**
 * @brief Reads an unsigned 32-bit integer value from the given address.
 *
 * @param addr The address to read from.
 * @return uint32_t The value at the address.
 */
uint32_t read_uint32(uint8_t const* addr);

/**
 * @brief Reads an unsigned 64-bit integer value from the given address.
 *
 * @param addr The address to read from.
 * @return uint64_t The value at the address.
 */
uint64_t read_uint64(uint8_t const* addr);

/**
 * @brief Reads a signed 8-bit integer value from the given address.
 *
 * @param addr The address to read from.
 * @return int8_t The value at the address.
 */
int8_t read_int8(uint8_t const* addr);

/**
 * @brief Reads a signed 16-bit integer value from the given address.
 *
 * @param addr The address to read from.
 * @return int16_t The value at the address.
 */
int16_t read_int16(uint8_t const* addr);

/**
 * @brief Reads a signed 32-bit integer value from the given address.
 *
 * @param addr The address to read from.
 * @return int32_t The value at the address.
 */
int32_t read_int32(uint8_t const* addr);

/**
 * @brief Reads a signed 64-bit integer value from the given address.
 *
 * @param addr The address to read from.
 * @return int64_t The value at the address.
 */
int64_t read_int64(uint8_t const* addr);

/**
 * @brief Reads a floating point value from the given address.
 *
 * @param addr The address to read from.
 * @return float The value at the address.
 */
float read_float(uint8_t const* addr);

/**
 * @brief Reads a double precision floating point value from the given address.
 *
 * @param addr The address to read from.
 * @return double The value at the address.
 */
double read_double(uint8_t const* addr);

/**
 * @brief Reads a null-terminated string pointed to by the given address.
 *
 * @param addr The address to read from.
 * @return char const* A pointer to the string.
 */
char const* read_string(uint8_t const* addr);

template <typename T>
class Array {
    
};

}  // namespace simplebuffers

#endif