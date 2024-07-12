#include "test.hpp"

namespace simplebuffers_test {

/*
 * RequestWriter
 */

RequestWriter::RequestWriter(uint32_t id, PayloadWriter payload):
    id(id), payload(payload) {}

uint16_t RequestWriter::static_size() const { return 7; }

uint8_t* RequestWriter::write_component(uint8_t* dest, const uint8_t* dest_end,
                         uint8_t* dyn_cursor) const {
    if (dest_end - dest < 7) return nullptr;
    dyn_cursor = simplebuffers::write_field(dest, dest_end, dyn_cursor, id);
    if (dyn_cursor == nullptr) return nullptr;
    dest += simplebuffers::get_static_size(id);
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

uint8_t* RequestWriter::PayloadWriter::write_component(uint8_t* dest, const uint8_t* dest_end,
                         uint8_t* dyn_cursor) const {
    switch (tag) {
        case Tag::INIT:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 0, *value.init);
        case Tag::MOVE_TO:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 1, *value.move_to);
        case Tag::TEST_ONE_OF:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 2, *value.test_one_of);
        default:
            return nullptr;
    }
}

RequestWriter::PayloadWriter::PayloadWriter(Tag tag, Value value) : tag(tag), value(value) {}

/*
 * RequestWriter::PayloadWriter::TestOneOfWriter
 */

RequestWriter::PayloadWriter::TestOneOfWriter RequestWriter::PayloadWriter::TestOneOfWriter::move_to_entry(MoveToEntryWriter* val) {
    Value v;
    v.move_to_entry = val;
    return TestOneOfWriter(Tag::MOVE_TO_ENTRY, v);
}

RequestWriter::PayloadWriter::TestOneOfWriter RequestWriter::PayloadWriter::TestOneOfWriter::string_test(StringTestWriter* val) {
    Value v;
    v.string_test = val;
    return TestOneOfWriter(Tag::STRING_TEST, v);
}

RequestWriter::PayloadWriter::TestOneOfWriter RequestWriter::PayloadWriter::TestOneOfWriter::big_boy(BigBoy* val) {
    Value v;
    v.big_boy = val;
    return TestOneOfWriter(Tag::BIG_BOY, v);
}

uint8_t* RequestWriter::PayloadWriter::TestOneOfWriter::write_component(uint8_t* dest, const uint8_t* dest_end,
                         uint8_t* dyn_cursor) const {
    switch (tag) {
        case Tag::MOVE_TO_ENTRY:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 0, *value.move_to_entry);
        case Tag::STRING_TEST:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 1, *value.string_test);
        case Tag::BIG_BOY:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 2, *value.big_boy);
        default:
            return nullptr;
    }
}

RequestWriter::PayloadWriter::TestOneOfWriter::TestOneOfWriter(Tag tag, Value value) : tag(tag), value(value) {}

/*
 * InitWriter
 */

InitWriter::InitWriter(uint32_t expected_firmware):
    expected_firmware(expected_firmware) {}

uint16_t InitWriter::static_size() const { return 4; }

uint8_t* InitWriter::write_component(uint8_t* dest, const uint8_t* dest_end,
                         uint8_t* dyn_cursor) const {
    if (dest_end - dest < 4) return nullptr;
    dyn_cursor = simplebuffers::write_field(dest, dest_end, dyn_cursor, expected_firmware);
    if (dyn_cursor == nullptr) return nullptr;
    return dyn_cursor;
}

/*
 * MoveToWriter
 */

MoveToWriter::MoveToWriter(simplebuffers::ArrayWriter<MoveToEntryWriter> joints):
    joints(joints) {}

uint16_t MoveToWriter::static_size() const { return 4; }

uint8_t* MoveToWriter::write_component(uint8_t* dest, const uint8_t* dest_end,
                         uint8_t* dyn_cursor) const {
    if (dest_end - dest < 4) return nullptr;
    dyn_cursor = simplebuffers::write_field(dest, dest_end, dyn_cursor, joints);
    if (dyn_cursor == nullptr) return nullptr;
    return dyn_cursor;
}

/*
 * MoveToEntryWriter
 */

MoveToEntryWriter::MoveToEntryWriter(RobotJoint joint, float angle, float speed):
    joint(joint), angle(angle), speed(speed) {}

uint16_t MoveToEntryWriter::static_size() const { return 9; }

uint8_t* MoveToEntryWriter::write_component(uint8_t* dest, const uint8_t* dest_end,
                         uint8_t* dyn_cursor) const {
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

uint8_t* StringTestWriter::write_component(uint8_t* dest, const uint8_t* dest_end,
                         uint8_t* dyn_cursor) const {
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

uint8_t* StringTestWriter::FieldsWriter::write_component(uint8_t* dest, const uint8_t* dest_end,
                         uint8_t* dyn_cursor) const {
    switch (tag) {
        case Tag::TEST:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 0, *value.test);
        case Tag::STRING:
            return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, 1, *value.string);
        default:
            return nullptr;
    }
}

StringTestWriter::FieldsWriter::FieldsWriter(Tag tag, Value value) : tag(tag), value(value) {}


} // namespace simplebuffers_test