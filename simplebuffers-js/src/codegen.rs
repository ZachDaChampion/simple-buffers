use std::collections::HashSet;

use indent::indent_by;
use indoc::formatdoc;
use itertools::Itertools;
use simplebuffers_core::{Enum, Primitive, SBSchema, Sequence, Type};

/// Capacity used to initialize the import set with.
const DEFAULT_IMPORT_CAPACITY: usize = 16;

type ImportSet = HashSet<&'static str>;

/// A class that was generated.
struct GeneratedClass {
    /// The main code that was generated.
    pub main: String,
    /// An optional namespace that complements the main code. This is used for
    /// namespace merging to facilitate scoped OneOf types.
    pub namespace: Option<String>,
}

//                                                                                                //
// ======================================= Main Function ======================================== //
//                                                                                                //

/// Generates a typescript file from a given schema.
///
/// # Arguments
///
/// * `schema` - The schema to generate from.
///
/// # Returns
///
/// The code for the file, as a String.
pub(crate) fn generate_code(schema: &SBSchema) -> String {
    // Set of things to import from the corelib.
    let mut import_set = ImportSet::with_capacity(DEFAULT_IMPORT_CAPACITY);

    // Generate enum definitions.
    let enum_definitions = schema.enums.iter().map(define_enum).join("\n\n");

    // Generate sequence definitions.
    let sequence_definitions = schema
        .sequences
        .iter()
        .map(|s| define_sequence(s, &mut import_set).main)
        .join("\n\n");

    // Generate code for imports. This has to be last so we know we have all of them.
    let imports = generate_imports(&import_set);

    // Generate the full file.
    formatdoc! {
        r#"
        /* eslint-disable @typescript-eslint/no-namespace */

        {imports}

        {enum_definitions}

        {sequence_definitions}
        "#
    }
    .replace("\n\n\n", "\n")
}

//                                                                                                //
// =========================================== Utils ============================================ //
//                                                                                                //

fn to_ts_type(ty: &Type) -> String {
    match ty {
        Type::Primitive(primitive) => match primitive {
            Primitive::Bool => "boolean".to_string(),
            Primitive::U8
            | Primitive::I8
            | Primitive::U16
            | Primitive::I16
            | Primitive::U32
            | Primitive::I32
            | Primitive::F32
            | Primitive::F64 => "number".to_string(),
            Primitive::U64 | Primitive::I64 => "bigint".to_string(),
        },
        Type::Sequence(name) => name.clone(),
        Type::Enum(name, _) => name.clone(),
        Type::Array(vals_type) => format!("{}[]", to_ts_type(vals_type)),
        Type::String => "string".to_string(),
        Type::OneOf(fields) => "ONEOF".to_string(),
    }
}

//                                                                                                //
// ===================================== Generate Sections ====================================== //
//                                                                                                //

/// Generates TS code for importing everything needed from corelib.
fn generate_imports(import_set: &ImportSet) -> String {
    if import_set.is_empty() {
        return "".to_string();
    }

    let imports = import_set.iter().join(",\n");

    formatdoc! {
        r#"
        import {{
            {imports}
        }} from "./simplebuffers.ts";"#,
        imports = indent_by(4, imports)
    }
}

/// Generates TS code for defining an enum.
fn define_enum(data: &Enum) -> String {
    let variants = data
        .variants
        .iter()
        .map(|v| format!("{} = {}", v.name, v.value))
        .join(",\n");

    formatdoc! {
        r"
        export enum {name} {{
            {variants}
        }}",
        name = data.name,
        variants = indent_by(4, variants)
    }
}

/// Generates the TS code for defining a sequence class.
fn define_sequence(seq: &Sequence, import_set: &mut ImportSet) -> GeneratedClass {
    import_set.insert("Sequence");

    let class_name = seq.name.clone();
    let static_size = seq.fields.iter().fold(0, |acc, f| acc + f.ty.size());

    // Generate field declarations
    let field_declarations = seq
        .fields
        .iter()
        .map(|f| format!("{}: {};", f.name, to_ts_type(&f.ty)))
        .join("\n");

    // Generate full class code.
    let main_code = formatdoc! {
        r"
        export class {class_name} extends Sequence {{
            static static_size = {static_size};
            static_size = {class_name}.static_size;
            {field_declarations}
        }}",
        field_declarations = indent_by(4, field_declarations)
    };

    GeneratedClass {
        main: main_code,
        namespace: None,
    }
}
