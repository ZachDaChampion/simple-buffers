/* eslint-disable @typescript-eslint/no-namespace */

import {
    Sequence,
    OneOf,
    SerializedComponent,
    String,
    I64,
    OneOfOption,
    ComponentConstructor,
    serialize_list,
    U8,
    deserialize_list,
} from "./simplebuffers";

export enum RobotJoint {
    j0 = 0,
    j1 = 1,
    j2 = 2,
    j3 = 3,
    j4 = 4,
    j5 = 5,
}

export enum BigBoy {
    only_option = 999999,
}

export class Init extends Sequence {
    static_size = 4;
    expected_firmware: number;

    constructor(expected_firmware: number) {
        super();
        this.expected_firmware = expected_firmware;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const static_view = new DataView(buffer);
        static_view.setUint32(static_offset, this.expected_firmware, true);
        return {
            buffer: buffer,
            dyn_offset: dyn_offset,
        };
    }

    static deserialize(buffer: ArrayBuffer): Init {
        const static_view = new DataView(buffer);
        const expected_firmware = static_view.getUint32(0, true);
        return new Init(expected_firmware);
    }
}

export class MoveTo extends Sequence {
    static static_size = 4;
    static_size = MoveTo.static_size;
    joints: MoveToEntry[];

    constructor(joints: MoveToEntry[]) {
        super();
        this.joints = joints;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const write_result = serialize_list<MoveToEntry>(
            this.joints,
            MoveToEntry.constructor as ComponentConstructor,
            buffer,
            static_offset,
            dyn_offset
        );
        buffer = write_result.buffer;
        dyn_offset = write_result.dyn_offset;

        return { buffer: buffer, dyn_offset: dyn_offset };
    }

    static deserialize(buffer: ArrayBuffer): MoveTo {
        const joints = deserialize_list(buffer, MoveToEntry.static_size, MoveToEntry.deserialize);
        return new MoveTo(joints);
    }
}

export class MoveToEntry extends Sequence {
    static static_size = 9;
    static_size = MoveToEntry.static_size;
    joint: RobotJoint;
    angle: number;
    speed: number;

    constructor(joint: RobotJoint, angle: number, speed: number) {
        super();
        this.joint = joint;
        this.angle = angle;
        this.speed = speed;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const static_view = new DataView(buffer);
        static_view.setUint8(static_offset, this.joint);
        static_view.setFloat32(static_offset + 1, this.angle, true);
        static_view.setFloat32(static_offset + 5, this.speed, true);
        return {
            buffer: buffer,
            dyn_offset: dyn_offset,
        };
    }

    static deserialize(buffer: ArrayBuffer): MoveToEntry {
        const static_view = new DataView(buffer);
        const joint = static_view.getUint8(0);
        const angle = static_view.getFloat32(1, true);
        const speed = static_view.getFloat32(5, true);
        return new MoveToEntry(joint, angle, speed);
    }
}

export class StringTest extends Sequence {
    fields: StringTest.Fields__OneOf;

    constructor(fields: StringTest.Fields__OneOf) {
        super();
        this.fields = fields;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const write_result = this.fields.serialize_component(buffer, static_offset, dyn_offset);
        buffer = write_result.buffer;
        dyn_offset = write_result.dyn_offset;
        return { buffer: buffer, dyn_offset: dyn_offset };
    }

    static deserialize(buffer: ArrayBuffer): StringTest {
        const fields = StringTest.Fields__OneOf.deserialize(buffer);
        return new StringTest(fields);
    }
}

export namespace StringTest {
    export class Fields__OneOf extends OneOf {
        static get options(): OneOfOption[] {
            return [
                {
                    key: "test",
                    constructor: String.constructor as ComponentConstructor,
                    deserializer: String.deserialize,
                },
                {
                    key: "string",
                    constructor: I64.constructor as ComponentConstructor,
                    deserializer: I64.deserialize,
                },
            ];
        }

        constructor(key_id: number, value: unknown) {
            super(key_id, value, Fields__OneOf.options);
        }

        static deserialize(buffer: ArrayBuffer): Fields__OneOf {
            const raw = OneOf.deserialize_impl(buffer, Fields__OneOf.options);
            return new Fields__OneOf(raw.key_id, raw.value);
        }
    }

    export class Fields__OneOf__Test extends Fields__OneOf {
        constructor(value: string) {
            super(0, value);
        }
    }

    export class Fields__OneOf__String extends Fields__OneOf {
        constructor(value: bigint) {
            super(1, value);
        }
    }
}

export class Request extends Sequence {
    id: number;
    enmArray: RobotJoint[];
    payload: Request.Payload__OneOf;

    constructor(id: number, enmArray: RobotJoint[], payload: Request.Payload__OneOf) {
        super();
        this.id = id;
        this.enmArray = enmArray;
        this.payload = payload;
    }

    static deserialize(buffer: ArrayBuffer): Request {
        const static_view = new DataView(buffer);
        const id = static_view.getUint32(0, true);
        const enmArray = deserialize_list(buffer.slice(4), 1, U8.deserialize);
        const payload = Request.Payload__OneOf.deserialize(buffer.slice(8));
        return new Request(id, enmArray, payload);
    }
}

export namespace Request {
    export class Payload__OneOf extends OneOf {
        static get options(): OneOfOption[] {
            return [
                {
                    key: "init",
                    constructor: Init.constructor as ComponentConstructor,
                    deserializer: Init.deserialize,
                },
                {
                    key: "moveTo",
                    constructor: MoveTo.constructor as ComponentConstructor,
                    deserializer: MoveTo.deserialize,
                },
                {
                    key: "testOneOf",
                    constructor: Payload__OneOf.TestOneOf__OneOf
                        .constructor as ComponentConstructor,
                    deserializer: Payload__OneOf.TestOneOf__OneOf.deserialize,
                },
            ];
        }

        constructor(key_id: number, value: unknown) {
            super(key_id, value, Payload__OneOf.options);
        }

        static deserialize(buffer: ArrayBuffer): Payload__OneOf {
            const raw = OneOf.deserialize_impl(buffer, Payload__OneOf.options);
            return new Payload__OneOf(raw.key_id, raw.value);
        }
    }

    export class Payload__OneOf__Init extends Payload__OneOf {
        constructor(value: Init) {
            super(0, value);
        }
    }

    export class Payload__OneOf__MoveTo extends Payload__OneOf {
        constructor(value: MoveTo) {
            super(1, value);
        }
    }

    export namespace Payload__OneOf {
        export class TestOneOf__OneOf extends OneOf {
            static get options(): OneOfOption[] {
                return [
                    {
                        key: "moveToEntry",
                        constructor: MoveToEntry.constructor as ComponentConstructor,
                        deserializer: MoveToEntry.deserialize,
                    },
                    {
                        key: "bigBoy",
                        constructor: U8.constructor as ComponentConstructor,
                        deserializer: U8.deserialize,
                    },
                    {
                        key: "stringTest",
                        constructor: String.constructor as ComponentConstructor,
                        deserializer: String.deserialize,
                    },
                ];
            }

            constructor(key_id: number, value: unknown) {
                super(key_id, value, TestOneOf__OneOf.options);
            }

            static deserialize(buffer: ArrayBuffer): TestOneOf__OneOf {
                const raw = OneOf.deserialize_impl(buffer, TestOneOf__OneOf.options);
                return new TestOneOf__OneOf(raw.key_id, raw.value);
            }
        }

        export class TestOneOf__OneOf__MoveToEntry extends TestOneOf__OneOf {
            constructor(value: MoveToEntry) {
                super(0, value);
            }
        }

        export class TestOneOf__OneOf__BigBoy extends TestOneOf__OneOf {
            constructor(value: BigBoy) {
                super(1, value);
            }
        }

        export class TestOneOf__OneOf__StringTest extends TestOneOf__OneOf {
            constructor(value: string) {
                super(2, value);
            }
        }
    }
}
