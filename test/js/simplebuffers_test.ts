/* eslint-disable @typescript-eslint/no-namespace */

import { Writer, LazySequenceReader, Sequence, OneOf, SerializedComponent } from "./simplebuffers";

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

    serialize_component(dyn_offset: number): SerializedComponent {
        const static_buf = new ArrayBuffer(this.static_size);
        const static_view = new DataView(static_buf);
        const dynamic_buf = new ArrayBuffer(0);
        static_view.setUint32(0, this.expected_firmware, true);
        return { static_buf: static_buf, dynamic_buf: dynamic_buf, dyn_offset: dyn_offset };
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
    fields: StringTest.FieldsTypeTest;

    constructor(fields: StringTest.FieldsTypeTest) {
        super();
        this.fields = fields;
    }
}

export namespace StringTest {
    export class FieldsTypeTest extends OneOf<string> {
        constructor(value: string) {
            super(value);
            this.key = 0;
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
