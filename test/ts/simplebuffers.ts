/* eslint-disable @typescript-eslint/no-unused-vars */

function buffer_reserve(buffer: ArrayBuffer, additional_space: number): ArrayBuffer {
    if (additional_space > 0) {
        const new_buffer = new ArrayBuffer(buffer.byteLength + additional_space);
        new Uint8Array(new_buffer).set(new Uint8Array(buffer), 0);
        return new_buffer;
    } else {
        return buffer;
    }
}

export function identity(x: Component): Component {
    return x;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type ComponentConstructor = (_: any) => Component;
type Deserializer = (buffer: ArrayBuffer) => unknown;

export interface SerializedComponent {
    buffer: ArrayBuffer;
    dyn_offset: number;
}

export function serialize_list<RawType>(
    raw_values: RawType[],
    component_constructor: ComponentConstructor,
    buffer: ArrayBuffer,
    static_offset: number,
    dyn_offset: number
): SerializedComponent {
    const values = raw_values.map(component_constructor);

    // Resize buffer to fit list, if necessary
    const serialized_list_size = values.map((x) => x.static_size).reduce((a, b) => a + b);
    const remaining_dyn_buffer_space = buffer.byteLength - dyn_offset;
    buffer = buffer_reserve(buffer, serialized_list_size - remaining_dyn_buffer_space);
    const static_view = new DataView(buffer);

    // Write length
    static_view.setUint16(static_offset, values.length, true);
    static_offset += 2;

    // Write offset to list
    static_view.setUint16(static_offset, dyn_offset - static_offset, true);

    // Serialize list into dynamic section
    let list_offset = dyn_offset;
    dyn_offset += serialized_list_size;
    for (const value of values) {
        const result = value.serialize_component(buffer, list_offset, dyn_offset);
        list_offset += value.static_size;
        dyn_offset = result.dyn_offset;
        buffer = result.buffer;
    }

    return {
        buffer: buffer,
        dyn_offset: dyn_offset,
    };
}

export function deserialize_list<RawType>(
    buffer: ArrayBuffer,
    element_size: number,
    deserializer: (buffer: ArrayBuffer) => RawType
): RawType[] {
    const static_view = new DataView(buffer);
    const length = static_view.getUint16(0, true);
    const offset = static_view.getUint16(2, true);
    const list_buffer = buffer.slice(2 + offset);

    const result: RawType[] = [];
    for (let i = 0; i < length; ++i) {
        result.push(deserializer(list_buffer.slice(i * element_size)));
    }

    return result;
}

export class Writer {
    static_size = 0;

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        throw new Error("serialize_component not implemented");
    }

    serialize(initial_dyn_allocation = 0): ArrayBuffer {
        const initial_buffer = new ArrayBuffer(this.static_size + initial_dyn_allocation);
        const result = this.serialize_component(initial_buffer, 0, this.static_size);
        return result.buffer;
    }
}

export class Component extends Writer {
    static deserialize(buffer: ArrayBuffer): unknown {
        throw new Error("deserialize not implemented");
    }

    static deserialize_lazy(buffer: ArrayBuffer): LazySequenceReader {
        throw new Error("deserialize_lazy not implemented");
    }
}

export class LazySequenceReader {
    buffer: ArrayBuffer;

    constructor(buffer: ArrayBuffer) {
        this.buffer = buffer;
    }
}

export class Sequence extends Component {}

export class U8 extends Component {
    static static_size = 1;
    static_size = U8.static_size;
    value: number;

    constructor(value: number) {
        super();
        this.value = value;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const static_view = new DataView(buffer);
        static_view.setUint8(static_offset, this.value);
        return {
            buffer: buffer,
            dyn_offset: dyn_offset,
        };
    }

    static deserialize(buffer: ArrayBuffer): number {
        const static_view = new DataView(buffer);
        return static_view.getUint8(0);
    }
}

export class I8 extends Component {
    static static_size = 1;
    static_size = I8.static_size;
    value: number;

    constructor(value: number) {
        super();
        this.value = value;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const static_view = new DataView(buffer);
        static_view.setInt8(static_offset, this.value);
        return {
            buffer: buffer,
            dyn_offset: dyn_offset,
        };
    }

    static deserialize(buffer: ArrayBuffer): number {
        const static_view = new DataView(buffer);
        return static_view.getInt8(0);
    }
}

export class U16 extends Component {
    static static_size = 2;
    static_size = U16.static_size;
    value: number;

    constructor(value: number) {
        super();
        this.value = value;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const static_view = new DataView(buffer);
        static_view.setUint16(static_offset, this.value, true);
        return {
            buffer: buffer,
            dyn_offset: dyn_offset,
        };
    }

    static deserialize(buffer: ArrayBuffer): number {
        const static_view = new DataView(buffer);
        return static_view.getUint16(0, true);
    }
}

export class I16 extends Component {
    static static_size = 2;
    static_size = I16.static_size;
    value: number;

    constructor(value: number) {
        super();
        this.value = value;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const static_view = new DataView(buffer);
        static_view.setInt16(static_offset, this.value, true);
        return {
            buffer: buffer,
            dyn_offset: dyn_offset,
        };
    }

    static deserialize(buffer: ArrayBuffer): number {
        const static_view = new DataView(buffer);
        return static_view.getInt16(0, true);
    }
}

export class U32 extends Component {
    static static_size = 4;
    static_size = U32.static_size;
    value: number;

    constructor(value: number) {
        super();
        this.value = value;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const static_view = new DataView(buffer);
        static_view.setUint32(static_offset, this.value, true);
        return {
            buffer: buffer,
            dyn_offset: dyn_offset,
        };
    }

    static deserialize(buffer: ArrayBuffer): number {
        const static_view = new DataView(buffer);
        return static_view.getUint32(0, true);
    }
}

export class I32 extends Component {
    static static_size = 4;
    static_size = I32.static_size;
    value: number;

    constructor(value: number) {
        super();
        this.value = value;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const static_view = new DataView(buffer);
        static_view.setInt32(static_offset, this.value, true);
        return {
            buffer: buffer,
            dyn_offset: dyn_offset,
        };
    }

    static deserialize(buffer: ArrayBuffer): number {
        const static_view = new DataView(buffer);
        return static_view.getInt32(0, true);
    }
}

export class U64 extends Component {
    static static_size = 8;
    static_size = U64.static_size;
    value: bigint;

    constructor(value: bigint) {
        super();
        this.value = value;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const static_view = new DataView(buffer);
        static_view.setBigUint64(static_offset, this.value, true);
        return {
            buffer: buffer,
            dyn_offset: dyn_offset,
        };
    }

    static deserialize(buffer: ArrayBuffer): bigint {
        const static_view = new DataView(buffer);
        return static_view.getBigUint64(0);
    }
}

export class I64 extends Component {
    static static_size = 8;
    static_size = I64.static_size;
    value: bigint;

    constructor(value: bigint) {
        super();
        this.value = value;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const static_view = new DataView(buffer);
        static_view.setBigInt64(static_offset, this.value, true);
        return {
            buffer: buffer,
            dyn_offset: dyn_offset,
        };
    }

    static deserialize(buffer: ArrayBuffer): bigint {
        const static_view = new DataView(buffer);
        return static_view.getBigInt64(0);
    }
}

export class F32 extends Component {
    static static_size = 4;
    static_size = F32.static_size;
    value: number;

    constructor(value: number) {
        super();
        this.value = value;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const static_view = new DataView(buffer);
        static_view.setFloat32(static_offset, this.value, true);
        return {
            buffer: buffer,
            dyn_offset: dyn_offset,
        };
    }

    static deserialize(buffer: ArrayBuffer): number {
        const static_view = new DataView(buffer);
        return static_view.getFloat32(0);
    }
}

export class F64 extends Component {
    static static_size = 8;
    static_size = F64.static_size;
    value: number;

    constructor(value: number) {
        super();
        this.value = value;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const static_view = new DataView(buffer);
        static_view.setFloat64(static_offset, this.value, true);
        return {
            buffer: buffer,
            dyn_offset: dyn_offset,
        };
    }

    static deserialize(buffer: ArrayBuffer): number {
        const static_view = new DataView(buffer);
        return static_view.getFloat64(0);
    }
}

export class Bool extends Component {
    static static_size = 1;
    static_size = Bool.static_size;
    value: number;

    constructor(value: number) {
        super();
        this.value = value;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const static_view = new DataView(buffer);
        static_view.setUint8(static_offset, this.value ? 1 : 0);
        return {
            buffer: buffer,
            dyn_offset: dyn_offset,
        };
    }

    static deserialize(buffer: ArrayBuffer): boolean {
        const static_view = new DataView(buffer);
        return static_view.getUint8(0) != 1 ? true : false;
    }
}

export class String extends Component {
    static static_size = 2;
    static_size = String.static_size;
    value: string;

    constructor(value: string) {
        super();
        this.value = value;
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        // Resize buffer to fit string, if necessary
        const string_len = this.value.length + 1;
        const remaining_dyn_buffer_space = buffer.byteLength - dyn_offset;
        buffer = buffer_reserve(buffer, string_len - remaining_dyn_buffer_space);
        const static_view = new DataView(buffer);

        // Write offset to string
        static_view.setUint16(static_offset, dyn_offset - static_offset, true);

        // Encode string into a dynamic portion of the buffer
        const encoder = new TextEncoder();
        const dyn_buffer = new Uint8Array(buffer, dyn_offset);
        const encode_result = encoder.encodeInto(this.value, dyn_buffer);

        // Append null terminator
        if (encode_result.written < dyn_buffer.length) {
            dyn_buffer[encode_result.written] = 0x00;
        } else {
            throw new Error("Dynamic buffer not large enough");
        }

        return {
            buffer: buffer,
            dyn_offset: dyn_offset + string_len,
        };
    }

    static deserialize(buffer: ArrayBuffer): string {
        const static_view = new DataView(buffer);
        const offset = static_view.getUint16(0, true);
        const string_buffer = new Uint8Array(buffer, offset);
        const null_terminator_pos = string_buffer.indexOf(0x00);

        if (null_terminator_pos == -1) {
            throw new Error("Couldn't find end to string");
        }

        const isolated_string = string_buffer.slice(0, null_terminator_pos);
        return new TextDecoder().decode(isolated_string);
    }
}

export interface OneOfOption {
    key: string;
    constructor: ComponentConstructor;
    deserializer: Deserializer;
}

export class OneOf extends Component {
    static static_size = 3;
    static_size = OneOf.static_size;
    key: string;
    key_id: number;
    value: unknown;
    component_constructor: ComponentConstructor;

    constructor(key_id: number, value: unknown, options: OneOfOption[]) {
        super();
        this.key_id = key_id;
        this.value = value;
        this.key = options[key_id].key;

        if (value instanceof Component) {
            this.component_constructor = identity;
        } else if (key_id < options.length) {
            this.component_constructor = options[key_id].constructor;
        } else {
            throw new Error("Key out of oneof range");
        }
    }

    serialize_component(
        buffer: ArrayBuffer,
        static_offset: number,
        dyn_offset: number
    ): SerializedComponent {
        const value = this.component_constructor(this.value);

        // Resize buffer to fit string, if necessary
        const component_size = value.static_size;
        const remaining_dyn_buffer_space = buffer.byteLength - dyn_offset;
        buffer = buffer_reserve(buffer, component_size - remaining_dyn_buffer_space);
        const static_view = new DataView(buffer);

        // Write key
        static_view.setUint8(static_offset, this.key_id);
        static_offset += 1;

        // Write offset to component
        static_view.setUint16(static_offset, dyn_offset - static_offset, true);

        // Serialize value into dynamic section
        return value.serialize_component(buffer, dyn_offset, dyn_offset + component_size);
    }

    static deserialize_impl(buffer: ArrayBuffer, options: OneOfOption[]): OneOf {
        const static_view = new DataView(buffer);
        const key = static_view.getUint8(0);
        const offset = static_view.getUint16(1, true);

        if (key >= options.length) {
            throw new Error(`Deserialized oneof key ${key} is invalid`);
        }

        const deserializer = options[key].deserializer;
        const value = deserializer(buffer.slice(1 + offset));

        return new OneOf(key, value, options);
    }
}
