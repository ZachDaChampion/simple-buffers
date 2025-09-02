import { MoveTo, MoveToEntry, Request, RobotJoint } from "./simplebuffers_test.js";

function print_array_buffer(buffer: ArrayBuffer) {
    const u8_array = new Uint8Array(buffer);
    let msg = "";
    for (let i = 0; i < u8_array.length; ++i) {
        msg += `${u8_array[i].toString(16)} `;
    }
    console.log(msg);
}

const to_serialize = new Request(
    2,
    [RobotJoint.j0, RobotJoint.j2, RobotJoint.j1],
    new Request.Payload__OneOf__MoveTo(
        new MoveTo([
            new MoveToEntry(RobotJoint.j5, 12.3, 98.7),
            new MoveToEntry(RobotJoint.j4, 32.1, 78.9),
        ])
    )
);
const serialized = to_serialize.serialize();
print_array_buffer(serialized);

const deserialized = Request.deserialize(serialized);
console.log(deserialized);
for (const j of deserialized.payload.value.joints) {
    console.log(j);
}
