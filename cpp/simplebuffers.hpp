#ifndef SIMPLEBUFFERS__SIMPLEBUFFERS__ZACHDACHAMPION__HPP
#define SIMPLEBUFFERS__SIMPLEBUFFERS__ZACHDACHAMPION__HPP

#include <cstdint>
#include <cstring>

namespace simplebuffers {

template <typename T>
class ArrayWriter;

//                                                                                                //
// ===================================== SimpleBufferWriter ===================================== //
//                                                                                                //

/**
 * A generic buffer writer interface. Child classes are used to write specific data types to a
 * buffer.
 *
 * Do not use this class directly. Instead, use one of the child classes generated by the
 * SimpleBuffers code generator.
 */
class SimpleBufferWriter {
   public:
    /**
     * Returns the number of static bytes that will be written to the buffer. This does not include
     * dynamic data, such as strings or arrays, but it does include offset values for dynamic data.
     *
     * @return The number of static bytes that will be written to the buffer.
     */
    virtual uint16_t size() const = 0;

    /**
     * Writes the message to the buffer.
     *
     * @param[out] dest The destination buffer to write to.
     * @param[in] dest_size The size of the destination buffer.
     * @return The number of bytes written to the buffer, or `-1` if there was not enough room in
     *         the buffer.
     */
    int32_t write(uint8_t* dest, uint16_t dest_size) const {
        uint8_t* res = write_component(dest, dest + dest_size, nullptr);
        if (res == nullptr) return -1;
        return res - dest;
    }

    /**
     * Writes the data to the buffer as a component of a larger message.
     *
     * Static data will be written directly to the beginning of `dest`, while dynamic data will
     * be appended at `dyn_cursor`, which should point to the first free byte in the buffer. If
     * `dyn_cursor` is `nullptr`, then dynamic data will be written immediately after the static
     * data.
     *
     * This function will return a pointer to the first free byte in the dynamic data section of
     * the buffer, or `nullptr` if the buffer cannot fit the data.
     *
     * **Note:** The returned pointer is not guaranteed to be valid. If the buffer is exactly
     * full after writing, then the returned pointer will be outside the buffer. Make sure to
     * check this before writing to the buffer at the returned pointer.
     *
     * @param[out] dest The destination buffer to write to.
     * @param[in] dest_end A pointer to the end of the destination buffer (i.e. the first byte
     * after the end of the buffer).
     * @param[out] dyn_cursor The current position of the dynamic data cursor.
     * @return A pointer to the first free byte in the dynamic section of the buffer after
     *         writing, or `nullptr` if the buffer was too small.
     */
    virtual uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                                     uint8_t* dyn_cursor = nullptr) const = 0;
};

/**
 * Writes a field value to a destination buffer.
 *
 * This function writes the value of a field to a destination buffer. It is designed to work
 * with any supported data type through template specialization. If the data type is not
 * supported, then this function will return `nullptr`.
 *
 * @param dest The destination buffer where the field value will be written.
 * @param dest_end A pointer to the end of the destination buffer.
 * @param dyn_cursor A pointer to the current position in the dynamic memory buffer.
 * @param val The value of the field to be written.
 * @return A pointer to the next position in the destination buffer after writing the field
 *         value.
 */
uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                     const uint8_t& val) {
    dest[0] = val;
    return dyn_cursor;
}
uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                     const int8_t& val) {
    dest[0] = val;
    return dyn_cursor;
}

uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                     const uint16_t& val) {
    dest[0] = val >> 8;
    dest[1] = val & 0xFF;
    return dyn_cursor;
}

uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                     const int16_t& val) {
    dest[0] = val >> 8;
    dest[1] = val & 0xFF;
    return dyn_cursor;
}

uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                     const uint32_t& val) {
    dest[0] = val >> 24;
    dest[1] = val >> 16;
    dest[2] = val >> 8;
    dest[3] = val & 0xFF;
    return dyn_cursor;
}

uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                     const int32_t& val) {
    dest[0] = val >> 24;
    dest[1] = val >> 16;
    dest[2] = val >> 8;
    dest[3] = val & 0xFF;
    return dyn_cursor;
}

uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                     const uint64_t& val) {
    dest[0] = val >> 56;
    dest[1] = val >> 48;
    dest[2] = val >> 40;
    dest[3] = val >> 32;
    dest[4] = val >> 24;
    dest[5] = val >> 16;
    dest[6] = val >> 8;
    dest[7] = val & 0xFF;
    return dyn_cursor;
}

uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                     const int64_t& val) {
    dest[0] = val >> 56;
    dest[1] = val >> 48;
    dest[2] = val >> 40;
    dest[3] = val >> 32;
    dest[4] = val >> 24;
    dest[5] = val >> 16;
    dest[6] = val >> 8;
    dest[7] = val & 0xFF;
    return dyn_cursor;
}

uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                     const float& val) {
    uint32_t val_int;
    memcpy(&val_int, &val, sizeof(uint32_t));  // Only legal way to do this pre C++ 20
    dest[0] = val_int >> 24;
    dest[1] = val_int >> 16;
    dest[2] = val_int >> 8;
    dest[3] = val_int & 0xFF;
    return dyn_cursor;
}

uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                     const double& val) {
    uint64_t val_int;
    memcpy(&val_int, &val, sizeof(uint64_t));  // Only legal way to do this pre C++ 20
    dest[0] = val_int >> 56;
    dest[1] = val_int >> 48;
    dest[2] = val_int >> 40;
    dest[3] = val_int >> 32;
    dest[4] = val_int >> 24;
    dest[5] = val_int >> 16;
    dest[6] = val_int >> 8;
    dest[7] = val_int & 0xFF;
    return dyn_cursor;
}

uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor, const bool& val) {
    dest[0] = val;
    return dyn_cursor;
}

uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                     const SimpleBufferWriter& val) {
    return val.write_component(dest, dest_end, dyn_cursor);
}

uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                     const char* const& val) {
    // Make sure the dynamic section of the buffer is large enough to hold the string
    // (including the null terminator).
    uint16_t str_len = strlen(val);
    if (dyn_cursor + str_len + 1 > dest_end) return nullptr;

    // Write the data offset to the static section of the buffer.
    uint16_t offset = dyn_cursor - dest;
    dest[0] = offset >> 8;
    dest[1] = offset & 0xFF;

    // Write the string to the dynamic section of the buffer.
    memcpy(dyn_cursor, val, str_len + 1);
    dyn_cursor += str_len + 1;

    return dyn_cursor;
}

/**
 * Writes a field value to the dynamic portion of the destination buffer. This is used for sequences
 * and unions that are directly nested in other sequences or unions. Without this function, the
 * static size of sequences could not be known due to potential cycles in the data structure.
 *
 * @param dest The destination buffer where the field value will be written.
 * @param dest_end A pointer to the end of the destination buffer.
 * @param dyn_cursor A pointer to the current position in the dynamic memory buffer.
 * @param val The value of the field to be written.
 * @return A pointer to the next position in the destination buffer after writing the field
 *         value.
 */
uint8_t* write_field_in_dynamic(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                                const SimpleBufferWriter& val) {
    // Make sure the dynamic section of the buffer is large enough to hold the static data of
    // the component.
    uint16_t static_size = val.size();
    if (dyn_cursor + static_size > dest_end) return nullptr;

    // Write the data offset to the static section of the buffer.
    uint16_t offset = dyn_cursor - dest;
    dest[0] = offset >> 8;
    dest[1] = offset & 0xFF;

    // Write the component to the dynamic section of the buffer.
    return val.write_component(dyn_cursor, dest_end, dyn_cursor + static_size);
}

/**
 * Calculates the static size of a given value. This does not include dynamic data, such as
 * strings or arrays.
 *
 * @param val The value to calculate the static size for.
 * @return The static size of the value.
 */
uint16_t get_static_size(const uint8_t val) { return 1; }
uint16_t get_static_size(const int8_t val) { return 1; }
uint16_t get_static_size(const uint16_t val) { return 2; }
uint16_t get_static_size(const int16_t val) { return 2; }
uint16_t get_static_size(const uint32_t val) { return 4; }
uint16_t get_static_size(const int32_t val) { return 4; }
uint16_t get_static_size(const uint64_t val) { return 8; }
uint16_t get_static_size(const int64_t val) { return 8; }
uint16_t get_static_size(const float val) { return 4; }
uint16_t get_static_size(const double val) { return 8; }
uint16_t get_static_size(const bool val) { return 1; }
uint16_t get_static_size(const SimpleBufferWriter& val) { return val.size(); }
uint16_t get_static_size(const char* const& val) { return 2; }

//                                                                                                //
// ======================================== ArrayWriter ========================================= //
//                                                                                                //

/**
 * A special SimpleBufferWriter that writes arrays to the buffer.
 */
template <typename T>
class ArrayWriter : public SimpleBufferWriter {
   public:
    // Static size of an array is always 2 bytes (for the offset).
    uint16_t size() const override { return 2; }

    // Stores a pointer to the array and its length.
    ArrayWriter(const T* val, uint16_t len) : val_(val), len_(len) {}

    // Writes the array to the buffer.
    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                             uint8_t* dyn_cursor = nullptr) const override {
        // Write the offset to the static section of the buffer.
        uint16_t offset = dyn_cursor - dest;
        dest[0] = offset >> 8;
        dest[1] = offset & 0xFF;

        // Write the array to the dynamic section of the buffer.
        return write_data_(dyn_cursor, dest_end, val_, len_);
    }

   private:
    const T* val_;
    uint16_t len_;
};

/**
 * Writes the data of an array.
 *
 * @tparam T The type of the array elements.
 * @param[in] dest The destination buffer to write to.
 * @param[in] dest_end A pointer to the end of the destination buffer (i.e. the first byte after
 *                     the final element of the buffer).
 * @param[in] val The array of values to write.
 * @param[in] len The length of the array.
 * @return A pointer to the first free byte in the dynamic section of the buffer after writing,
 *         or `nullptr` if the buffer was too small.
 */
template <typename T>
uint8_t* write_data_(uint8_t* dest, const uint8_t* dest_end, T* val, uint16_t len) {
    // Make sure there is enough space in the buffer for the array.
    uint16_t element_static_size = (len > 0) ? get_static_size(val[0]) : 0;
    uint16_t total_static_size = 2 + element_static_size * len;  // 2 bytes for the array length
    if (dest + total_static_size > dest_end) return nullptr;

    // Create a pointer to the end of the static array data. This is where the dynamic data of
    // the elements will be written.
    uint8_t* dyn_cursor = dest + total_static_size;

    // Write the array length to the buffer.
    dest[0] = len >> 8;
    dest[1] = len & 0xFF;
    dest += 2;

    // Write each element to the buffer.
    for (uint16_t i = 0; i < len; ++i) {
        uint8_t* res = write_field(dest, dest_end, dyn_cursor, val[i]);
        if (res == nullptr) return nullptr;
        dest += element_static_size;
        dyn_cursor = res;
    }

    return dyn_cursor;
}

template <>
uint8_t* write_data_<uint8_t>(uint8_t* dest, const uint8_t* dest_end, uint8_t* val, uint16_t len) {
    // Make sure there is enough space in the buffer for the array.
    uint16_t total_static_size = 2 + len;  // 2 bytes for the array length
    if (dest + total_static_size > dest_end) return nullptr;

    // Write the array length to the buffer.
    dest[0] = len >> 8;
    dest[1] = len & 0xFF;
    dest += 2;

    // Write each element to the buffer.
    memcpy(dest, val, len);

    return dest + total_static_size;
}

template <>
uint8_t* write_data_<int8_t>(uint8_t* dest, const uint8_t* dest_end, int8_t* val, uint16_t len) {
    // Make sure there is enough space in the buffer for the array.
    uint16_t total_static_size = 2 + len;  // 2 bytes for the array length
    if (dest + total_static_size > dest_end) return nullptr;

    // Write the array length to the buffer.
    dest[0] = len >> 8;
    dest[1] = len & 0xFF;
    dest += 2;

    // Write each element to the buffer.
    memcpy(dest, val, len);

    return dest + total_static_size;
}

}  // namespace simplebuffers

#endif  // SIMPLEBUFFERS__SIMPLEBUFFERS__ZACHDACHAMPION__HPP