export interface SerializedComponent {
    static_buf: ArrayBuffer;
    dynamic_buf: ArrayBuffer;
    dyn_offset: number;
}

export class Writer {
    serialize_component(dyn_offset: number): SerializedComponent {
        throw new Error("serialize_component not implemented");
    }

    serialize(): ArrayBuffer {
        const buffers = this.serialize_component(0);
        const static_buf = buffers[0];
        const dynamic_buf = buffers[1];
        const static_len = static_buf.byteLength;
        const dynamic_len = dynamic_buf.byteLength;

        const out = new Uint8Array(static_len + dynamic_len);
        out.set(new Uint8Array(static_buf), 0);
        out.set(new Uint8Array(dynamic_buf), static_len);
        return out.buffer;
    }
}

export class LazySequenceReader {
    buffer: ArrayBuffer;

    constructor(buffer: ArrayBuffer) {
        this.buffer = buffer;
    }
}

export class Sequence extends Writer {
    static_size = 0;

    static deserialize(buffer: ArrayBuffer) {
        throw new Error("deserialize not implemented");
    }

    static deserialize_lazy(buffer: ArrayBuffer) {
        throw new Error("deserialize_lazy not implemented");
    }
}

export class String extends Writer {
    static_size = 2;
    value: string;

    constructor(value: string) {
        super();
        this.value = value;
    }

    serialize_component(dyn_offset: number): SerializedComponent {
        const static_buf = new ArrayBuffer(this.static_size);
        const static_view = new DataView(static_buf);

        // Offset to string
        static_view.setUint16(0, dyn_offset, true);

        // Encode string into a uint8 array
        const encoder = new TextEncoder();
        const encodedString = new Uint8Array(this.value.length + 1);
        const encodeResult = encoder.encodeInto(this.value, encodedString);

        // Append null terminator
        if (encodeResult.written < encodedString.length) {
            encodedString[encodeResult.written] = 0;
        }

        const dynamic_buf = encodedString.buffer;
        return {
            static_buf: static_buf,
            dynamic_buf: dynamic_buf,
            dyn_offset: dyn_offset + dynamic_buf.byteLength,
        };
    }
}

// export class OneOf<ValueType> extends Writer {
//     key: number;
//     value: ValueType;

//     constructor(value: ValueType) {
//         super();
//         this.value = value;
//     }

//     serialize_component(): [ArrayBuffer, ArrayBuffer] {
//         const static_buf = new ArrayBuffer(this.static_size);
//         const static_view = new DataView(static_buf);
//         const dynamic_buffs = [];

//         return [static_buf, dynamic_buf];
//     }
// }
