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

            static TestOneOfWriter move_to_entry(MoveToEntryWriter* val);
            static TestOneOfWriter string_test(StringTestWriter* val);
            static TestOneOfWriter big_boy(BigBoy* val);

            uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                                     uint8_t* dyn_cursor) const override;

           protected:
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

        static PayloadWriter init(InitWriter* val);
        static PayloadWriter move_to(MoveToWriter* val);
        static PayloadWriter test_one_of(TestOneOfWriter* val);

        uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                                 uint8_t* dyn_cursor) const override;

       protected:
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
    class FieldsWriter : public simplebuffers::OneOfWriter {
       public:
        enum class Tag : uint8_t {
            TEST = 0,
            STRING = 1
        };

        union Value {
            const char** test;
            int64_t* string;
        };

        static FieldsWriter test(const char** val);
        static FieldsWriter string(int64_t* val);

        uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                                 uint8_t* dyn_cursor) const override;

       protected:
        FieldsWriter(Tag tag, Value value);

        Tag tag;
        Value value;
    };

    StringTestWriter(FieldsWriter fields);

    FieldsWriter fields;

    uint16_t static_size() const override;
    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                             uint8_t* dyn_cursor) const override;
};

class RequestReader;
class InitReader;
class MoveToReader;
class MoveToEntryReader;
class StringTestReader;

class RequestReader : public simplebuffers::SimpleBufferReader {
    public:
    class PayloadReader : public simplebuffers::OneOfReader {
       public:
        class TestOneOfReader : public simplebuffers::OneOfReader {
           public:
            enum class Tag : uint8_t {
                MOVE_TO_ENTRY = 0,
                STRING_TEST = 1,
                BIG_BOY = 2
            };

            TestOneOfReader(uint8_t* data_ptr, size_t data_len = 0);
            Tag tag() const;
            MoveToEntryReader move_to_entry() const;
            StringTestReader string_test() const;
            BigBoy big_boy() const;

           protected:
            Tag tag;
        };

        enum class Tag : uint8_t {
            INIT = 0,
            MOVE_TO = 1,
            TEST_ONE_OF = 2
        };

        PayloadReader(uint8_t* data_ptr, size_t data_len = 0);
        Tag tag() const;
        InitReader init() const;
        MoveToReader move_to() const;
        TestOneOfReader test_one_of() const;

       protected:
        Tag tag;
    };

    uint16_t static_size() const override;
    uint32_t id() const;
        PayloadReader payload() const;
};

class InitReader : public simplebuffers::SimpleBufferReader {
    public:
    uint16_t static_size() const override;
    uint32_t expected_firmware() const;
};

class MoveToReader : public simplebuffers::SimpleBufferReader {
    public:
    uint16_t static_size() const override;
    simplebuffers::ArrayReader<MoveToEntryWriter> joints() const;
};

class MoveToEntryReader : public simplebuffers::SimpleBufferReader {
    public:
    uint16_t static_size() const override;
    RobotJoint joint() const;
        float angle() const;
        float speed() const;
};

class StringTestReader : public simplebuffers::SimpleBufferReader {
    public:
    class FieldsReader : public simplebuffers::OneOfReader {
       public:
        enum class Tag : uint8_t {
            TEST = 0,
            STRING = 1
        };

        FieldsReader(uint8_t* data_ptr, size_t data_len = 0);
        Tag tag() const;
        const char* test() const;
        int64_t string() const;

       protected:
        Tag tag;
    };

    uint16_t static_size() const override;
    FieldsReader fields() const;
};

} // namespace simplebuffers_test

#endif