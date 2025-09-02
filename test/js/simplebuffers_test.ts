/* eslint-disable @typescript-eslint/no-namespace */

import {
    Component,
    LazySequenceReader,
    Sequence,
    OneOf,
    SerializedComponent,
    String,
    I64,
    OneOfOption,
    ComponentConstructor,
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
}

export class MoveTo extends Sequence {
    joints: MoveToEntry[];

    constructor(joints: MoveToEntry[]) {
        super();
        this.joints = joints;
    }
}

export class MoveToEntry extends Sequence {
    joint: RobotJoint;
    angle: number;
    speed: number;

    constructor(joint: RobotJoint, angle: number, speed: number) {
        super();
        this.joint = joint;
        this.angle = angle;
        this.speed = speed;
    }
}

export class StringTest extends Sequence {
    fields: StringTest.Fields__OneOf;

    constructor(fields: StringTest.Fields__OneOf) {
        super();
        this.fields = fields;
    }
}

export namespace StringTest {
    export class Fields__OneOf extends OneOf {
        static options: OneOfOption[] = [
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
    id;
    enmArray;
    payload;

    constructor(id, enmArray, payload) {
        super();
        this.id = id;
        this.enmArray = enmArray;
        this.payload = payload;
    }
}
