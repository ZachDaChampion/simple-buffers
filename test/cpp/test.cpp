#include "test.hpp"

namespace simplebuffers_test {

/*
 * RequestWriter
 */

RequestWriter::RequestWriter(uint32_t id, simplebuffers::ListWriter<RobotJoint> enm_array, PayloadWriter payload):
    id(id), enm_array(enm_array), payload(payload) {}

uint16_t RequestWriter::static_size() const { return 11; }

uint8_t* RequestWriter::write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const {
    if (dest_end - dest < 11) return nullptr;
    dyn_cursor = simplebuffers::write_field(dest, dest_end, dyn_cursor, id);
    if (dyn_cursor == nullptr) return nullptr;
    dest += simplebuffers::get_static_size(id);
    dyn_cursor = simplebuffers::write_field(dest, dest_end, dyn_cursor, simplebuffers::priv::ListWriterImpl<uint8_t>(reinterpret_cast<uint8_t* const>(enm_array.val), enm_array.len));
    if (dyn_cursor == nullptr) return nullptr;
    dest += simplebuffers::get_static_size(simplebuffers::priv::ListWriterImpl<uint8_t>(reinterpret_cast<uint8_t* const>(enm_array.val), enm_array.len));
    dyn_cursor = simplebuffers::write_field(dest, dest_end, dyn_cursor, payload);
    if (dyn_cursor == nullptr) return nullptr;
    return dyn_cursor;
}

/*
 * RequestWriter::PayloadWriter
 */

RequestWriter::PayloadWriter RequestWriter::PayloadWriter::init(InitWriter* val) {
    Value v;
    v.init = val;
    return PayloadWriter(Tag::INIT, v);
}

RequestWriter::PayloadWriter RequestWriter::PayloadWriter::move_to(MoveToWriter* val) {
    Value v;
    v.move_to = val;
    return PayloadWriter(Tag::MOVE_TO, v);
}

RequestWriter::PayloadWriter RequestWriter::PayloadWriter::test_one_of(TestOneOfWriter* val) {
    Value v;
    v.test_one_of = val;
    return PayloadWriter(Tag::TEST_ONE_OF, v);
}

uint8_t* RequestWriter::PayloadWriter::write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const {
    switch (tag_) {
        case Tag::INIT:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 0, *value_.init);
        case Tag::MOVE_TO:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 1, *value_.move_to);
        case Tag::TEST_ONE_OF:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 2, *value_.test_one_of);
        default:
            return nullptr;
    }
}

RequestWriter::PayloadWriter::PayloadWriter(Tag tag, Value value) : tag_(tag), value_(value) {}

/*
 * RequestWriter::PayloadWriter::TestOneOfWriter
 */

RequestWriter::PayloadWriter::TestOneOfWriter RequestWriter::PayloadWriter::TestOneOfWriter::move_to_entry(MoveToEntryWriter* val) {
    Value v;
    v.move_to_entry = val;
    return TestOneOfWriter(Tag::MOVE_TO_ENTRY, v);
}

RequestWriter::PayloadWriter::TestOneOfWriter RequestWriter::PayloadWriter::TestOneOfWriter::big_boy(BigBoy* val) {
    Value v;
    v.big_boy = val;
    return TestOneOfWriter(Tag::BIG_BOY, v);
}

RequestWriter::PayloadWriter::TestOneOfWriter RequestWriter::PayloadWriter::TestOneOfWriter::string_test(StringTestWriter* val) {
    Value v;
    v.string_test = val;
    return TestOneOfWriter(Tag::STRING_TEST, v);
}

uint8_t* RequestWriter::PayloadWriter::TestOneOfWriter::write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const {
    switch (tag_) {
        case Tag::MOVE_TO_ENTRY:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 0, *value_.move_to_entry);
        case Tag::BIG_BOY:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 1, static_cast<uint32_t>(*value_.big_boy));
        case Tag::STRING_TEST:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 6, *value_.string_test);
        default:
            return nullptr;
    }
}

RequestWriter::PayloadWriter::TestOneOfWriter::TestOneOfWriter(Tag tag, Value value) : tag_(tag), value_(value) {}

/*
 * InitWriter
 */

InitWriter::InitWriter(uint32_t expected_firmware):
    expected_firmware(expected_firmware) {}

uint16_t InitWriter::static_size() const { return 4; }

uint8_t* InitWriter::write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const {
    if (dest_end - dest < 4) return nullptr;
    dyn_cursor = simplebuffers::write_field(dest, dest_end, dyn_cursor, expected_firmware);
    if (dyn_cursor == nullptr) return nullptr;
    return dyn_cursor;
}

/*
 * MoveToWriter
 */

MoveToWriter::MoveToWriter(simplebuffers::ListWriter<MoveToEntryWriter> joints):
    joints(joints) {}

uint16_t MoveToWriter::static_size() const { return 4; }

uint8_t* MoveToWriter::write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const {
    if (dest_end - dest < 4) return nullptr;
    dyn_cursor = simplebuffers::write_field(dest, dest_end, dyn_cursor, simplebuffers::priv::ListWriterImpl<MoveToEntryWriter>(joints.val, joints.len));
    if (dyn_cursor == nullptr) return nullptr;
    return dyn_cursor;
}

/*
 * MoveToEntryWriter
 */

MoveToEntryWriter::MoveToEntryWriter(RobotJoint joint, float angle, float speed):
    joint(joint), angle(angle), speed(speed) {}

uint16_t MoveToEntryWriter::static_size() const { return 9; }

uint8_t* MoveToEntryWriter::write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const {
    if (dest_end - dest < 9) return nullptr;
    dyn_cursor = simplebuffers::write_field(dest, dest_end, dyn_cursor, static_cast<uint8_t>(joint));
    if (dyn_cursor == nullptr) return nullptr;
    dest += simplebuffers::get_static_size(static_cast<uint8_t>(joint));
    dyn_cursor = simplebuffers::write_field(dest, dest_end, dyn_cursor, angle);
    if (dyn_cursor == nullptr) return nullptr;
    dest += simplebuffers::get_static_size(angle);
    dyn_cursor = simplebuffers::write_field(dest, dest_end, dyn_cursor, speed);
    if (dyn_cursor == nullptr) return nullptr;
    return dyn_cursor;
}

/*
 * StringTestWriter
 */

StringTestWriter::StringTestWriter(FieldsWriter fields):
    fields(fields) {}

uint16_t StringTestWriter::static_size() const { return 3; }

uint8_t* StringTestWriter::write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const {
    if (dest_end - dest < 3) return nullptr;
    dyn_cursor = simplebuffers::write_field(dest, dest_end, dyn_cursor, fields);
    if (dyn_cursor == nullptr) return nullptr;
    return dyn_cursor;
}

/*
 * StringTestWriter::FieldsWriter
 */

StringTestWriter::FieldsWriter StringTestWriter::FieldsWriter::test(const char** val) {
    Value v;
    v.test = val;
    return FieldsWriter(Tag::TEST, v);
}

StringTestWriter::FieldsWriter StringTestWriter::FieldsWriter::string(int64_t* val) {
    Value v;
    v.string = val;
    return FieldsWriter(Tag::STRING, v);
}

uint8_t* StringTestWriter::FieldsWriter::write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const {
    switch (tag_) {
        case Tag::TEST:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 0, *value_.test);
        case Tag::STRING:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 1, *value_.string);
        default:
            return nullptr;
    }
}

StringTestWriter::FieldsWriter::FieldsWriter(Tag tag, Value value) : tag_(tag), value_(value) {}


/*
 * RequestReader
 */

RequestReader::RequestReader(const uint8_t* data_ptr, size_t idx) : simplebuffers::SimpleBufferReader(data_ptr + 11 * idx) {}

uint16_t RequestReader::static_size() const { return 11; }

uint32_t RequestReader::id() const {
    return simplebuffers::read_field<uint32_t>(data_ptr_ + 0);
}
simplebuffers::ListReader<RobotJoint, uint8_t> RequestReader::enm_array() const {
    return simplebuffers::ListReader<RobotJoint, uint8_t>(static_cast<const uint8_t*>(data_ptr_ + 4), 0);
}
RequestReader::PayloadReader RequestReader::payload() const {
    return PayloadReader(static_cast<const uint8_t*>(data_ptr_ + 8), 0);
}

/*
 * RequestReader::PayloadReader
 */

RequestReader::PayloadReader::PayloadReader(const uint8_t* data_ptr, size_t idx) : OneOfReader(data_ptr, idx) {
    const uint16_t offset = simplebuffers::read_field<uint16_t>(data_ptr + 1);
    tag_ = static_cast<Tag>(simplebuffers::read_field<uint8_t>(data_ptr));
    val_ptr_ = data_ptr + offset;
}

RequestReader::PayloadReader::Tag RequestReader::PayloadReader::tag() const {
    return tag_;
}

InitReader RequestReader::PayloadReader::init() const {
    return InitReader(val_ptr_, 0);
}

MoveToReader RequestReader::PayloadReader::move_to() const {
    return MoveToReader(val_ptr_, 0);
}

RequestReader::PayloadReader::TestOneOfReader RequestReader::PayloadReader::test_one_of() const {
    if (tag_ != Tag::TEST_ONE_OF) return TestOneOfReader(nullptr, 0);
    return TestOneOfReader(val_ptr_, 0);
}

/*
 * RequestReader::PayloadReader::TestOneOfReader
 */

RequestReader::PayloadReader::TestOneOfReader::TestOneOfReader(const uint8_t* data_ptr, size_t idx) : OneOfReader(data_ptr, idx) {
    const uint16_t offset = simplebuffers::read_field<uint16_t>(data_ptr + 1);
    tag_ = static_cast<Tag>(simplebuffers::read_field<uint8_t>(data_ptr));
    val_ptr_ = data_ptr + offset;
}

RequestReader::PayloadReader::TestOneOfReader::Tag RequestReader::PayloadReader::TestOneOfReader::tag() const {
    return tag_;
}

MoveToEntryReader RequestReader::PayloadReader::TestOneOfReader::move_to_entry() const {
    return MoveToEntryReader(val_ptr_, 0);
}

BigBoy RequestReader::PayloadReader::TestOneOfReader::big_boy() const {
    if (tag_ != Tag::BIG_BOY) return static_cast<BigBoy>(0);
    return static_cast<BigBoy>(simplebuffers::read_field<uint32_t>(val_ptr_));
}

StringTestReader RequestReader::PayloadReader::TestOneOfReader::string_test() const {
    return StringTestReader(val_ptr_, 0);
}

/*
 * InitReader
 */

InitReader::InitReader(const uint8_t* data_ptr, size_t idx) : simplebuffers::SimpleBufferReader(data_ptr + 4 * idx) {}

uint16_t InitReader::static_size() const { return 4; }

uint32_t InitReader::expected_firmware() const {
    return simplebuffers::read_field<uint32_t>(data_ptr_ + 0);
}

/*
 * MoveToReader
 */

MoveToReader::MoveToReader(const uint8_t* data_ptr, size_t idx) : simplebuffers::SimpleBufferReader(data_ptr + 4 * idx) {}

uint16_t MoveToReader::static_size() const { return 4; }

simplebuffers::ListReader<MoveToEntryReader> MoveToReader::joints() const {
    return simplebuffers::ListReader<MoveToEntryReader>(static_cast<const uint8_t*>(data_ptr_ + 0), 0);
}

/*
 * MoveToEntryReader
 */

MoveToEntryReader::MoveToEntryReader(const uint8_t* data_ptr, size_t idx) : simplebuffers::SimpleBufferReader(data_ptr + 9 * idx) {}

uint16_t MoveToEntryReader::static_size() const { return 9; }

RobotJoint MoveToEntryReader::joint() const {
    return static_cast<RobotJoint>(simplebuffers::read_field<uint8_t>(data_ptr_ + 0));
}
float MoveToEntryReader::angle() const {
    return simplebuffers::read_field<float>(data_ptr_ + 1);
}
float MoveToEntryReader::speed() const {
    return simplebuffers::read_field<float>(data_ptr_ + 5);
}

/*
 * StringTestReader
 */

StringTestReader::StringTestReader(const uint8_t* data_ptr, size_t idx) : simplebuffers::SimpleBufferReader(data_ptr + 3 * idx) {}

uint16_t StringTestReader::static_size() const { return 3; }

StringTestReader::FieldsReader StringTestReader::fields() const {
    return FieldsReader(static_cast<const uint8_t*>(data_ptr_ + 0), 0);
}

/*
 * StringTestReader::FieldsReader
 */

StringTestReader::FieldsReader::FieldsReader(const uint8_t* data_ptr, size_t idx) : OneOfReader(data_ptr, idx) {
    const uint16_t offset = simplebuffers::read_field<uint16_t>(data_ptr + 1);
    tag_ = static_cast<Tag>(simplebuffers::read_field<uint8_t>(data_ptr));
    val_ptr_ = data_ptr + offset;
}

StringTestReader::FieldsReader::Tag StringTestReader::FieldsReader::tag() const {
    return tag_;
}

const char* StringTestReader::FieldsReader::test() const {
    if (tag_ != Tag::TEST) return "\0";
    return simplebuffers::read_field<const char*>(val_ptr_);
}

int64_t StringTestReader::FieldsReader::string() const {
    if (tag_ != Tag::STRING) return 0;
    return simplebuffers::read_field<int64_t>(val_ptr_);
}


} // namespace simplebuffers_test