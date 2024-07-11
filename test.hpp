#ifndef SIMPLEBUFFERS_GENERATED__TEST_HPP
#define SIMPLEBUFFERS_GENERATED__TEST_HPP

#include "simplebuffers.hpp"

namespace simplebuffers_test {

enum class RobotJoint : uint_fast8_t {
    J_0 = 0,
    J_1 = 1,
    J_2 = 2,
    J_3 = 3,
    J_4 = 4,
    J_5 = 5
};

enum class BigBoy : uint_fast32_t {
    ONLY_OPTION = 999999
};

class RequestWriter;
class InitWriter;
class MoveToWriter;
class MoveToEntryWriter;
class StringTestWriter;

class RequestWriter : public simplebuffers::SimpleBufferWriter {
   public:
    class PayloadWriter : public simplebuffers::OneOfWriter {
       public:
        class TestOneOfWriter : public simplebuffers::OneOfWriter {
           public:
            enum class Tag : uint8_t {
                MOVE_TO_ENTRY = 0,
                STRING_TEST = 1,
                BIG_BOY = 2
            };

            union Value {
                MoveToEntryWriter* move_to_entry;
                StringTestWriter* string_test;
                BigBoy* big_boy;
            };

            static TestOneOfWriter MoveToEntry(MoveToEntryWriter* val);
            static TestOneOfWriter StringTest(StringTestWriter* val);
            static TestOneOfWriter BigBoy(BigBoy* val);

            uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                                     uint8_t* dyn_cursor) const override;

           private:
            TestOneOfWriter(Tag tag, Value value);

            Tag tag;
            Value value;
        };

        enum class Tag : uint8_t {
            INIT = 0,
            MOVE_TO = 1,
            TEST_ONE_OF = 2
        };

        union Value {
            InitWriter* init;
            MoveToWriter* move_to;
            TestOneOfWriter* test_one_of;
        };

        static PayloadWriter Init(InitWriter* val);
        static PayloadWriter MoveTo(MoveToWriter* val);
        static PayloadWriter TestOneOf(TestOneOfWriter* val);

        uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                                 uint8_t* dyn_cursor) const override;

       private:
        PayloadWriter(Tag tag, Value value);

        Tag tag;
        Value value;
    };

    RequestWriter(uint32_t id, PayloadWriter payload);

    uint32_t id;
    PayloadWriter payload;

    uint16_t static_size() const override;
    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                             uint8_t* dyn_cursor) const override;
};

class InitWriter : public simplebuffers::SimpleBufferWriter {
   public:
    InitWriter(uint32_t expected_firmware);

    uint32_t expected_firmware;

    uint16_t static_size() const override;
    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                             uint8_t* dyn_cursor) const override;
};

class MoveToWriter : public simplebuffers::SimpleBufferWriter {
   public:
    MoveToWriter(simplebuffers::ArrayWriter<MoveToEntryWriter> joints);

    simplebuffers::ArrayWriter<MoveToEntryWriter> joints;

    uint16_t static_size() const override;
    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                             uint8_t* dyn_cursor) const override;
};

class MoveToEntryWriter : public simplebuffers::SimpleBufferWriter {
   public:
    MoveToEntryWriter(RobotJoint joint, float angle, float speed);

    RobotJoint joint;
    float angle;
    float speed;

    uint16_t static_size() const override;
    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                             uint8_t* dyn_cursor) const override;
};

class StringTestWriter : public simplebuffers::SimpleBufferWriter {
   public:
    StringTestWriter(char* test, int64_t string);

    char* test;
    int64_t string;

    uint16_t static_size() const override;
    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                             uint8_t* dyn_cursor) const override;
};

} // namespace simplebuffers_test

#endif