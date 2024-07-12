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
                BIG_BOY = 1,
                STRING_TEST = 6
            };

            union Value {
                MoveToEntryWriter* move_to_entry;
                BigBoy* big_boy;
                StringTestWriter* string_test;
            };

            static TestOneOfWriter move_to_entry(MoveToEntryWriter* val);
            static TestOneOfWriter big_boy(BigBoy* val);
            static TestOneOfWriter string_test(StringTestWriter* val);

            uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const override;

           protected:
            TestOneOfWriter(Tag tag, Value value);

            Tag tag_;
            Value value_;
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

        uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const override;

       protected:
        PayloadWriter(Tag tag, Value value);

        Tag tag_;
        Value value_;
    };

    RequestWriter(uint32_t id, simplebuffers::ListWriter<RobotJoint> enm_array, PayloadWriter payload);

    uint32_t id;
    simplebuffers::ListWriter<RobotJoint> enm_array;
    PayloadWriter payload;

    uint16_t static_size() const override;
    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const override;
};

class InitWriter : public simplebuffers::SimpleBufferWriter {
   public:
    InitWriter(uint32_t expected_firmware);

    uint32_t expected_firmware;

    uint16_t static_size() const override;
    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const override;
};

class MoveToWriter : public simplebuffers::SimpleBufferWriter {
   public:
    MoveToWriter(simplebuffers::ListWriter<MoveToEntryWriter> joints);

    simplebuffers::ListWriter<MoveToEntryWriter> joints;

    uint16_t static_size() const override;
    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const override;
};

class MoveToEntryWriter : public simplebuffers::SimpleBufferWriter {
   public:
    MoveToEntryWriter(RobotJoint joint, float angle, float speed);

    RobotJoint joint;
    float angle;
    float speed;

    uint16_t static_size() const override;
    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const override;
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

        uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const override;

       protected:
        FieldsWriter(Tag tag, Value value);

        Tag tag_;
        Value value_;
    };

    StringTestWriter(FieldsWriter fields);

    FieldsWriter fields;

    uint16_t static_size() const override;
    uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const override;
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
                BIG_BOY = 1,
                STRING_TEST = 6
            };

            TestOneOfReader(const uint8_t* data_ptr, size_t idx = 0);
            Tag tag() const;
            MoveToEntryReader move_to_entry() const;
            BigBoy big_boy() const;
            StringTestReader string_test() const;

           protected:
            Tag tag_;
        };

        enum class Tag : uint8_t {
            INIT = 0,
            MOVE_TO = 1,
            TEST_ONE_OF = 2
        };

        PayloadReader(const uint8_t* data_ptr, size_t idx = 0);
        Tag tag() const;
        InitReader init() const;
        MoveToReader move_to() const;
        TestOneOfReader test_one_of() const;

       protected:
        Tag tag_;
    };

    RequestReader(const uint8_t* data_ptr, size_t idx = 0);
    uint16_t static_size() const override;
    uint32_t id() const;
    simplebuffers::ListReader<RobotJoint, uint8_t> enm_array() const;
    PayloadReader payload() const;
};

class InitReader : public simplebuffers::SimpleBufferReader {
    public:
    InitReader(const uint8_t* data_ptr, size_t idx = 0);
    uint16_t static_size() const override;
    uint32_t expected_firmware() const;
};

class MoveToReader : public simplebuffers::SimpleBufferReader {
    public:
    MoveToReader(const uint8_t* data_ptr, size_t idx = 0);
    uint16_t static_size() const override;
    simplebuffers::ListReader<MoveToEntryReader> joints() const;
};

class MoveToEntryReader : public simplebuffers::SimpleBufferReader {
    public:
    MoveToEntryReader(const uint8_t* data_ptr, size_t idx = 0);
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

        FieldsReader(const uint8_t* data_ptr, size_t idx = 0);
        Tag tag() const;
        const char* test() const;
        int64_t string() const;

       protected:
        Tag tag_;
    };

    StringTestReader(const uint8_t* data_ptr, size_t idx = 0);
    uint16_t static_size() const override;
    FieldsReader fields() const;
};

} // namespace simplebuffers_test

#endif