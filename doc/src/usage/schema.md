# Schema File Format

The core of SimpleBuffers is the schema file. This contains all of the data structures that can be
serialized. SimpleBuffers schemas are stored in files with the `.sb` extension and are passed to the
compiler for code generation.

Let's look at a simple example:

```
enum RobotJoint {
    j0 = 0;
    j1 = 1;
    j2 = 2;
    j3 = 3;
    j4 = 4;
    j5 = 5;
}

sequence Init {
    expected_firmware: u32;
}

sequence MoveToEntry {
    joint: RobotJoint;
    angle: f32;
    speed: f32;
}

sequence MoveTo {
    joints: [MoveToEntry];
    stop_smoothly: bool;
}

sequence Request {
    id: u32;
    payload: oneof {
        init: Init;
        moveTo: MoveTo;
    };
}
```

This schema characterizes some functionality for a robot arm. The main data structure is the
`Request` sequence, which contains an ID and a payload, which takes the form of some other sequence.
Before we can fully understand what this means, we have to explain some terminology.

## Enums

Enums, like in most programming languages, describe a set of finite values. In SimpleBuffers, enums
are backed by unsigned integers. Each enumeration must be explicitly assigned to a unique value.
Enumerations do not need to be assigned contiguously, as can be seen in the following example:

```
enum RobotJoint {
    j0 = 0;
    j1 = 1;
    j2 = 2;
    j3 = 3;
    j4 = 4;
    j5 = 5;
    unknown = 255;
}
```

The size of the backing integer is determined by the possible enumerations. In the above example,
`RobotJoint` will be backed by an 8-bit integer, as all enumerations can fit in it. However, if
`unknown`'s value were changed to be `300` instead of `255`, all `RobotJoint` instances would
instead be backed by a 16-bit integer as they no longer fit in 8.

## Sequences

Sequences are SimpleBuffers' equivalent to structs. Importantly, sequences are ordered; changing the
order of a sequence's fields will cause the serialization format to change. Semicolons are required
after every field.

```
sequence MoveToEntry {
    joint: RobotJoint;
    angle: f32;
    speed: f32;
}
```

Every field of a sequence (or oneof) must be annotated with a type. A type can be one of the
following:

- Primitive
- List
- Enum
- Sequence
- Oneof

## Primitive Types

SimpleBuffers contains the following primitive types:

| Type | Description                 |
| ---- | --------------------------- |
| u8   | An unsigned, 8-bit integer  |
| u16  | An unsigned, 16-bit integer |
| u32  | An unsigned, 32-bit integer |
| u64  | An unsigned, 64-bit integer |
| i8   | A signed, 8-bit integer     |
| i16  | A signed, 16-bit integer    |
| i32  | A signed, 32-bit integer    |
| i64  | A signed, 64-bit integer    |
| f32  | A 32-bit floating point     |
| f64  | A 64-bit floating point     |
| bool | A boolean value (8-bit)     |
| str  | A string                    |

Note that, unlike the rest of the primitive types, strings are variable-sized fields. This entails a
small amount of additional overhead which is explained further in
[Serialization Format](./serialization_format.md).

## Lists

Like strings, lists are variable-sized. See [Serialization Format](./serialization_format.md#lists)
for more information about the implications of this.

Lists are denoted by surrounding a type in square brackets. For example:

```
sequence MoveTo {
    joints: [MoveToEntry];
    stop_smoothly: bool;
}
```

The `joints` field is an array of `MoveToEntry` sequences.

## OneOf

Like a union in C, a oneof allows a single field to have multiple possible data types. In our
example, `Request` uses a oneof for the `payload` field. While the syntax looks similar to a
sequence, a oneof can only store a single value at a time.

```
sequence Request {
    id: u32;
    payload: oneof {
        init: Init;
        moveTo: MoveTo;
    };
}
```

Multiple oneof fields may be of the same type. This can be useful for readability and clarity, e.g.:

```
sequence LoginInfo {
    user: oneof {
        email: str;
        phone_num: str;
        username: str;
    };
}
```

## Comments

SimpleBuffers uses C-style single-line comments denoted by `//`. Multiline comments are not
supported.

```
// I am a comment
sequence MySequence {
    my_field: u8; // This is my field whom I love very much
}
```