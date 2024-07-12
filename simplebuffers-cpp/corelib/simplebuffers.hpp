#ifndef SIMPLEBUFFERS__SIMPLEBUFFERS__ZACHDACHAMPION__HPP
#define SIMPLEBUFFERS__SIMPLEBUFFERS__ZACHDACHAMPION__HPP

#include <cstdint>
#include <cstring>

namespace simplebuffers {

//                                                                                                //
// ===================================== SimpleBufferWriter ===================================== //
//                                                                                                //

/**
 * @brief An abstract base class for any class that can be written to a SimpleBuffer.
 *
 * All Sequences and OneOfs that are generated by the SimpleBuffers compiler inherit from this
 * class. A special ArrayWriter class is also provided for writing arrays.
 */
class SimpleBufferWriter {
   public:
    /**
     * @brief Returns the static size of the object.
     *
     * The number returned by this function represents the size of all statically-sized fields in
     * a component. Dynamically-sized fields are not included in this number, but relative offsets
     * are. These offsets are used to determine the location of dynamically-sized fields in the
     * buffer.
     *
     * @return The static size of the object.
     */
    virtual uint16_t static_size() const = 0;

    /**
     * @brief Writes a component to the destination buffer.
     *
     * Generally, this method should not be called directly. Instead, call the write() method. When
     * implementing this method in a derived class, keep the following in mind:
     *
     * - All static data should be written at `dest`.
     *
     * - All dynamic data should be written at `dyn_cursor`.
     *
     * - Fields should be written using the `write_field()` function, which will automatically
     *   handle dynamic data. Make sure to update `dyn_cursor` after writing each field. If a
     *   nullptr is returned, exit immediately and return nullptr.
     *
     * - This function assumes that space has already been reserved for the static data in the
     *   destination buffer. Therefore, there is no need to perform any checks when writing static
     *   data.
     *
     * - You MUST check for space in the destination buffer before writing dynamic data.
     *   `dyn_cursor` will always be after the static data, so the remaining buffer space is
     *   `dest_end - dyn_cursor`. If there is not enough space, return `nullptr`. You must also do
     *   this after calling `write_field()`, since it may write dynamic data.
     *
     * - After writing the dynamic data, you MUST return a pointer to the first free byte in the
     *   dynamic section of the buffer. This is the byte immediately after the last dynamic byte you
     *   wrote. If you did not write any dynamic data, return `dyn_cursor`.
     *
     * @param dest The destination to write static data to.
     * @param dest_end The end of the destination buffer.
     * @param dyn_cursor A dynamic cursor used for writing variable-length data.
     * @return A pointer to the end of the dynamic data written to the buffer, or `nullptr` if the
     *         buffer was too small.
     */
    virtual uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                                     uint8_t* dyn_cursor) const = 0;

    /**
     * @brief Serializes the component to the destination buffer.
     *
     * This method should not be overridden by derived classes. Instead, derived classes should
     * implement the write_component() method.
     *
     * @param dest The destination buffer to write the data to.
     * @param dest_size The size of the destination buffer.
     * @return The number of bytes written to the destination buffer, or -1 if the buffer was too
     *         small.
     */
    int32_t write(uint8_t* dest, uint16_t dest_size) const {
        uint8_t* res = write_component(dest, dest + dest_size, dest + static_size());
        if (res == nullptr) return -1;
        return res - dest;
    }
};

//                                                                                                //
// ======================================== ArrayWriter ========================================= //
//                                                                                                //

/**
 * @brief A class for writing arrays to a simple buffer.
 *
 * @tparam T The type of the array elements.
 */
template <typename T>
class ArrayWriter : public SimpleBufferWriter {
   public:
    /**
     * @brief Constructs an ArrayWriter object.
     *
     * @param val Pointer to the array.
     * @param len Length of the array.
     */
    ArrayWriter(T* const val, uint16_t len) : val_(val), len_(len) {}

    uint16_t static_size() const override { return 4; }

    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                             uint8_t* dyn_cursor = nullptr) const {
        uint16_t offset = dyn_cursor - dest;
        dest[0] = len_ >> 8;
        dest[1] = len_ & 0xFF;
        dest[2] = offset >> 8;
        dest[3] = offset & 0xFF;

        return write_data_(dyn_cursor, dest_end, val_, len_);
    }

   protected:
    /**
     * @brief Writes the data of an array to a destination buffer.
     *
     * This function is specialized for uint8_t and int8_t because they can be written efficiently
     * with memcpy without worrying about endianness.
     *
     * @tparam T The type of the array elements.
     * @param dest The destination buffer to write the data to.
     * @param dest_end Pointer to the end of the destination buffer.
     * @param val Pointer to the data to be written.
     * @param len The length of the data to be written.
     * @return Pointer to the next position in the destination buffer after writing the data.
     */
    static uint8_t* write_data_(uint8_t* dest, const uint8_t* dest_end, T* val, uint16_t len) {
        uint16_t element_static_size = (len > 0) ? get_static_size(val[0]) : 0;
        uint16_t total_static_size = 2 + element_static_size * len;  // 2 bytes for the array length
        if (dest + total_static_size > dest_end) return nullptr;

        // Create a pointer to the end of the static array data. This is where the dynamic data of
        // the elements will be written.
        uint8_t* dyn_cursor = dest + total_static_size;

        // Write each element to the buffer.
        for (uint16_t i = 0; i < len; ++i) {
            uint8_t* res = write_field(dest, dest_end, dyn_cursor, val[i]);
            if (res == nullptr) return nullptr;
            dest += element_static_size;
            dyn_cursor = res;
        }

        return dyn_cursor;
    }

   private:
    T* const val_;  ///< Pointer to the array.
    uint16_t len_;  ///< Length of the array.
};

/**
 * Efficiently writes u8 data from the source array to the destination array.
 *
 * @param dest Pointer to the destination array.
 * @param dest_end Pointer to the end of the destination array.
 * @param val Pointer to the source array.
 * @param len Length of the source array.
 * @return Pointer to the next position in the destination array after writing the data.
 */
template <>
uint8_t* ArrayWriter<uint8_t>::write_data_(uint8_t* dest, const uint8_t* dest_end, uint8_t* val,
                                           uint16_t len) {
    uint16_t total_static_size = 2 + len;  // 2 bytes for the array length
    if (dest + total_static_size > dest_end) return nullptr;

    memcpy(dest, val, len);
    return dest + total_static_size;
}

/**
 * Efficiently writes i8 data from the source array to the destination array.
 *
 * @param dest Pointer to the destination array.
 * @param dest_end Pointer to the end of the destination array.
 * @param val Pointer to the source array.
 * @param len Length of the source array.
 * @return Pointer to the next position in the destination array after writing the data.
 */
template <>
uint8_t* ArrayWriter<int8_t>::write_data_(uint8_t* dest, const uint8_t* dest_end, int8_t* val,
                                          uint16_t len) {
    uint16_t total_static_size = 2 + len;  // 2 bytes for the array length
    if (dest + total_static_size > dest_end) return nullptr;

    memcpy(dest, val, len);
    return dest + total_static_size;
}

//                                                                                                //
// ======================================== OneOfWriter ========================================= //
//                                                                                                //

/**
 * @brief An abstract class for writing OneOf fields to a simple buffer. Specific OneOf writers
 *        should inherit from this class.
 */
class OneOfWriter : public SimpleBufferWriter {
   public:
    /**
     * @brief The static size of all OneOf structures is 3 bytes: 1 byte for the tag and 2 bytes for
     *        the offset.
     *
     * @return The static size of the OneOf structure (3);
     */
    uint16_t static_size() const override { return 3; }
};

//                                                                                                //
// ====================================== Get static size ======================================= //
//                                                                                                //

/**
 * @brief Returns the static size of a given value.
 *
 * This does not include dynamic data, such as strings or arrays.
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
uint16_t get_static_size(const SimpleBufferWriter& val) { return val.static_size(); }
uint16_t get_static_size(char* const& val) { return 2; }

//                                                                                                //
// ======================================== Write field ========================================= //
//                                                                                                //

/**
 * @brief Writes a field value to the destination buffer.
 *
 * @param dest The destination to write static data to.
 * @param dest_end The end of the destination buffer.
 * @param dyn_cursor The dynamic cursor for writing variable-length fields.
 * @param val The value to write.
 * @return A pointer to the end of the dynamic data written to the buffer, or `nullptr` if the
 *         buffer was too small.
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
    dest[0] = val ? 1 : 0;
    return dyn_cursor;
}

uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                     char* const& val) {
    uint16_t str_len = strlen(val);
    if (dyn_cursor + str_len + 1 > dest_end) return nullptr;  // +1 for null terminator

    // Write the data offset to the static section of the buffer.
    uint16_t offset = dyn_cursor - dest;
    dest[0] = offset >> 8;
    dest[1] = offset & 0xFF;

    // Write the string to the dynamic section of the buffer.
    memcpy(dyn_cursor, val, str_len + 1);
    dyn_cursor += str_len + 1;

    return dyn_cursor;
}

uint8_t* write_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor,
                     const SimpleBufferWriter& val) {
    return val.write_component(dest, dest_end, dyn_cursor);
}

/**
 * @brief Writes a OneOf field to the destination buffer.
 *
 * @param[out] dest The destination to write static data to.
 * @param[in] dest_end The end of the destination buffer.
 * @param[out] dyn_cursor The dynamic cursor for writing variable-length fields.
 * @param[in] tag The tag of the OneOf field.
 * @param[in] val The value to write.
 * @return A pointer to the end of the dynamic data written to the buffer, or `nullptr` if the
 *         buffer was too small.
 */
template <typename T>
uint8_t* write_oneof_field(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor, uint8_t tag,
                           const T& val) {
    uint16_t static_size = get_static_size(val);
    if (dyn_cursor + static_size > dest_end) return nullptr;

    // Write the tag and offset to the static section of the buffer.
    uint16_t offset = dyn_cursor - (dest + 1);  // +1 because the tag comes before the offset
    dest[0] = tag;
    dest[1] = offset >> 8;
    dest[2] = offset & 0xFF;

    return write_field(dyn_cursor, dest_end, dyn_cursor + static_size, val);
};

//                                                                                                //
// ===================================== SimpleBufferReader ===================================== //
//                                                                                                //

class SimpleBufferReader {
   public:
    /**
     * Construct a new Reader object.
     *
     * @param[in] data_ptr A pointer to this component's location in a data buffer.
     * @param[in] data_len The length of the data buffer (measured from `data_ptr` to end).
     */
    SimpleBufferReader(uint8_t* data_ptr, size_t data_len = 0)
        : data_ptr_(data_ptr), data_len_(data_len) {}

    /**
     * @brief Returns a pointer to this component in the data buffer.
     *
     * @return Pointer to the component in the data buffer.
     */
    const uint8_t* data() const { return data_ptr_; }

    /**
     * @brief Returns the static size of the object.
     *
     * The number returned by this function represents the size of all statically-sized fields in
     * a component. Dynamically-sized fields are not included in this number, but relative offsets
     * are. These offsets are used to determine the location of dynamically-sized fields in the
     * buffer.
     *
     * @return The static size of the object.
     */
    virtual uint16_t static_size() const = 0;

   protected:
    const uint8_t* data_ptr_;
    size_t data_len_;
};

//                                                                                                //
// ======================================== ArrayReader ========================================= //
//                                                                                                //
/**
 * @brief A class for reading arrays from a simple buffer.
 *
 * @tparam T The type of the array elements.
 */
template <typename T>
class ArrayReader : public SimpleBufferReader {
   public:
    /**
     * Construct a new Reader object.
     *
     * @param[in] data_ptr A pointer to this component's location in a data buffer.
     * @param[in] data_len The length of the data buffer (measured from `data_ptr` to end).
     */
    ArrayReader(uint8_t* data_ptr, size_t data_len = 0) : SimpleBufferReader(data_ptr, data_len) {
        data_end_ = data_ptr + data_len;
        if (data_ptr + static_size() >= data_len) {
            array_len_ = read_u16(data_ptr);
            array_content_ = data_ptr + read_u16(data_ptr + 2);
        } else {
            array_len_ = 0;
            array_content_ = nullptr;
        }
    }

    /**
     * Get the number of elements in the array.
     *
     * @return The length of the array.
     */
    uint16_t len() const { return array_len_; }

    /**
     * Read the value at index `idx` from the array.
     *
     * This function does not check bounds and has undefined behavior if an out-of-bounds index is
     * given.
     *
     * @param[in] idx The index to read.
     * @return The value read from the array.
     */
    T read(uint16_t idx) const {
        return T(array_content_, static_cast<uint16_t>(data_end_ - array_content_));
    }

    /**
     * Read the value at index `idx` from the array.
     *
     * This function does not check bounds and has undefined behavior if an out-of-bounds index is
     * given.
     *
     * @param[in] idx The index to read.
     * @return The value read from the array.
     */
    T operator[](uint16_t idx) const { return read(idx); }

    /**
     * @brief Returns the static size of the object.
     *
     * The number returned by this function represents the size of all statically-sized fields in
     * a component. Dynamically-sized fields are not included in this number, but relative offsets
     * are. These offsets are used to determine the location of dynamically-sized fields in the
     * buffer.
     *
     * @return The static size of the object.
     */
    uint16_t static_size() const override { return 4; }

   protected:
    uint16_t array_len_;
    const uint8_t* array_content_;
    const uint8_t* data_end_;
};

template <>
uint8_t ArrayReader<uint8_t>::read(uint16_t idx) const {
    return read_u8(array_content_ + idx);
}

template <>
int8_t ArrayReader<int8_t>::read(uint16_t idx) const {
    return read_i8(array_content_ + idx);
}

template <>
uint16_t ArrayReader<uint16_t>::read(uint16_t idx) const {
    return read_u16(array_content_ + idx);
}

template <>
int16_t ArrayReader<int16_t>::read(uint16_t idx) const {
    return read_i16(array_content_ + idx);
}

template <>
uint32_t ArrayReader<uint32_t>::read(uint16_t idx) const {
    return read_u32(array_content_ + idx);
}

template <>
int32_t ArrayReader<int32_t>::read(uint16_t idx) const {
    return read_i32(array_content_ + idx);
}

template <>
uint64_t ArrayReader<uint64_t>::read(uint16_t idx) const {
    return read_u64(array_content_ + idx);
}

template <>
int64_t ArrayReader<int64_t>::read(uint16_t idx) const {
    return read_i64(array_content_ + idx);
}

template <>
float ArrayReader<float>::read(uint16_t idx) const {
    return read_f32(array_content_ + idx);
}

template <>
double ArrayReader<double>::read(uint16_t idx) const {
    return read_f64(array_content_ + idx);
}

template <>
bool ArrayReader<bool>::read(uint16_t idx) const {
    return read_bool(array_content_ + idx);
}

template <>
const char* ArrayReader<const char*>::read(uint16_t idx) const {
    const uint8_t* const static_ptr = array_content_ + idx;
    const uint16_t offset = read_u16(static_ptr);
    return reinterpret_cast<const char*>(static_ptr + offset);
}

//                                                                                                //
// ========================================= Read field ========================================= //
//                                                                                                //

/**
 * @brief Reads a u8 field from a buffer.
 *
 * @param src The destination to read static data from.
 */
uint8_t read_u8(const uint8_t* src) { return *src; }

/**
 * @brief Reads an i8 field from a buffer.
 *
 * @param src The destination to read static data from.
 */
int8_t read_i8(const uint8_t* src) { return *src; }

/**
 * @brief Reads a u16 field from a buffer.
 *
 * @param src The destination to read static data from.
 */
uint16_t read_u16(const uint8_t* src) {
    uint16_t val = 0;
    val |= src[0] << 0;
    val |= src[1] << 8;
    return val;
}

/**
 * @brief Reads an i16 field from a buffer.
 *
 * @param src The destination to read static data from.
 */
int16_t read_i16(const uint8_t* src) {
    int16_t val = 0;
    val |= src[0] << 0;
    val |= src[1] << 8;
    return val;
}

/**
 * @brief Reads a u32 field from a buffer.
 *
 * @param src The destination to read static data from.
 */
uint32_t read_u32(const uint8_t* src) {
    uint32_t val = 0;
    val |= src[0] << 0;
    val |= src[1] << 8;
    val |= src[2] << 16;
    val |= src[3] << 24;
    return val;
}

/**
 * @brief Reads an i32 field from a buffer.
 *
 * @param src The destination to read static data from.
 */
int32_t read_i32(const uint8_t* src) {
    int32_t val = 0;
    val |= src[0] << 0;
    val |= src[1] << 8;
    val |= src[2] << 16;
    val |= src[3] << 24;
    return val;
}

/**
 * @brief Reads a u64 field from a buffer.
 *
 * @param src The destination to read static data from.
 */
uint64_t read_u64(const uint8_t* src) {
    uint64_t val = 0;
    val |= src[0] << 0;
    val |= src[1] << 8;
    val |= src[2] << 16;
    val |= src[3] << 24;
    val |= src[0] << 32;
    val |= src[1] << 40;
    val |= src[2] << 48;
    val |= src[3] << 56;
    return val;
}

/**
 * @brief Reads an i64 field from a buffer.
 *
 * @param src The destination to read static data from.
 */
int64_t read_i64(const uint8_t* src) {
    int64_t val = 0;
    val |= src[0] << 0;
    val |= src[1] << 8;
    val |= src[2] << 16;
    val |= src[3] << 24;
    val |= src[0] << 32;
    val |= src[1] << 40;
    val |= src[2] << 48;
    val |= src[3] << 56;
    return val;
}

/**
 * @brief Reads an f32 field from a buffer.
 *
 * @param src The destination to read static data from.
 */
float read_f32(const uint8_t* src) {
    uint32_t val_int;
    float val;
    val_int |= src[0] << 0;
    val_int |= src[1] << 8;
    val_int |= src[2] << 16;
    val_int |= src[3] << 24;
    memcpy(&val, &val_int, sizeof(float));
    return val;
}

/**
 * @brief Reads an f64 field from a buffer.
 *
 * @param src The destination to read static data from.
 */
double read_f64(const uint8_t* src) {
    uint64_t val_int;
    double val;
    val_int |= src[0] << 0;
    val_int |= src[1] << 8;
    val_int |= src[2] << 16;
    val_int |= src[3] << 24;
    val_int |= src[0] << 32;
    val_int |= src[1] << 40;
    val_int |= src[2] << 48;
    val_int |= src[3] << 56;
    memcpy(&val, &val_int, sizeof(double));
    return val;
}

/**
 * @brief Reads a bool field from a buffer.
 *
 * @param src The destination to read static data from.
 */
bool read_bool(const uint8_t* src) { return *src ? true : false; }

/**
 * @brief Reads a string field from a buffer.
 *
 * @param src The destination to read data from.
 */
const char* read_string(const uint8_t* src) {
    const uint16_t offset = read_u16(src);
    return reinterpret_cast<const char*>(src + offset);
}

}  // namespace simplebuffers

#endif  // SIMPLEBUFFERS__SIMPLEBUFFERS__ZACHDACHAMPION__HPP