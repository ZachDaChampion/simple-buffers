/* eslint-disable @typescript-eslint/no-namespace */

import {
    Sequence
} from "./simplebuffers.ts";

export enum RobotJoint {
    j0 = 0,
    j1 = 1,
    j2 = 2,
    j3 = 3,
    j4 = 4,
    j5 = 5
}

export enum BigBoy {
    only_option = 999999
}

export class Request extends Sequence {
    static static_size = 11;
    static_size = Request.static_size;
    id: number;
    enmArray: RobotJoint[];
    payload: ONEOF;
}

export class Init extends Sequence {
    static static_size = 4;
    static_size = Init.static_size;
    expected_firmware: number;
}

export class MoveTo extends Sequence {
    static static_size = 4;
    static_size = MoveTo.static_size;
    joints: MoveToEntry[];
}

export class MoveToEntry extends Sequence {
    static static_size = 9;
    static_size = MoveToEntry.static_size;
    joint: RobotJoint;
    angle: number;
    speed: number;
}

export class StringTest extends Sequence {
    static static_size = 3;
    static_size = StringTest.static_size;
    fields: ONEOF;
}
