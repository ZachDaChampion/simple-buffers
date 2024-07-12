use crate::annotate::CppEnum;
use crate::annotate::CppOneOf;
use crate::annotate::CppSchema;
use crate::annotate::CppSequence;
use crate::annotate::ToReaderWriterString;
use crate::argparse::CppGeneratorParams;
use indent::indent_by;
use indoc::formatdoc;
use itertools::Itertools;

//                                                                                                //
// ======================================= Main Function ======================================== //
//                                                                                                //

/// Generates a header file from a given schema.
///
/// # Arguments
///
/// * `params` - Generator params.
/// * `schema` - The schema to generate from.
///
/// # Returns
///
/// The code for a header file, as a String.
pub(crate) fn generate_header(params: &CppGeneratorParams, schema: &CppSchema) -> String {
    // Generate the name of the include guards.
    let include_guards = format!(
        "SIMPLEBUFFERS_GENERATED__{}_HPP",
        params.global.file_name.to_uppercase()
    );

    // Generate namespace name.
    let namespace = format!("simplebuffers_{}", params.global.file_name);

    // Generate enum definitions.
    let enum_definitions = schema.enums.iter().map(define_enum).join("\n\n");

    // Generate forward declarations for sequence writers.
    let writer_forward_declarations = schema
        .sequences
        .iter()
        .map(forward_declare_sequence_writer)
        .join("\n");

    // Generate full descriptions for sequence writers.
    let sequence_writer_definitions = schema
        .sequences
        .iter()
        .map(define_sequence_writer)
        .join("\n\n");

    // Generate forward declarations for sequence readers.
    let reader_forward_declarations = schema
        .sequences
        .iter()
        .map(forward_declare_sequence_reader)
        .join("\n");

    // Generate full descriptions for sequence readers.
    let sequence_reader_definitions = schema
        .sequences
        .iter()
        .map(define_sequence_reader)
        .join("\n\n");

    // Generate the full header file.
    formatdoc! {
        r#"
        #ifndef {include_guards}
        #define {include_guards}

        #include "simplebuffers.hpp"

        namespace {namespace} {{

        {enum_definitions}

        {writer_forward_declarations}

        {sequence_writer_definitions}
        
        {reader_forward_declarations}

        {sequence_reader_definitions}

        }} // namespace {namespace}

        #endif"#
    }
    .replace("\n\n\n", "\n")
}

//                                                                                                //
// ================================== Generate File Components ================================== //
//                                                                                                //

/// Generates the C++ code for defining an enum.
fn define_enum(data: &CppEnum) -> String {
    // Name of the enum.
    let name = &data.name;

    // Data type to base the enum class off of.
    let dtype = data.size_to_type();

    // Generate the code for defining individual enum variants.
    let variants = data
        .variants
        .iter()
        .map(|v| format!("{} = {}", v.0, v.1))
        .join(",\n");

    // Generate the full enum code.
    formatdoc! {
        r"
        enum class {name} : {dtype} {{
            {variants}
        }};",
        variants = indent_by(4, variants)
    }
}

/// Generates the C++ code for forward declaring sequence writers.
fn forward_declare_sequence_writer(seq: &CppSequence) -> String {
    let case_corrected_name = seq.to_writer_string();
    format!("class {case_corrected_name};")
}

/// Generates the C++ code for defining a sequence writer.
fn define_sequence_writer(seq: &CppSequence) -> String {
    // The full name of the sequence writer class, in the form "SequenceWriter".
    let class_name = seq.to_writer_string();

    // Generate class body. We do this separately from the final class generation so that we can
    // trim the body and remove the extra whitespace present when there are no oneof fields.
    let body = {
        // Generate the parameter list for the constructor.
        let param_list = seq
            .fields
            .iter()
            .map(|f| format!("{} {}", f.ty.to_writer_string(), f.name))
            .join(", ");

        // Generate class definitions of any oneof fields contained in the sequence. These are
        // subclasses of this sequence class.
        let oneofs = seq.oneofs().map(define_oneof_writer).join("\n\n");

        // Generate member declarations for all fields.
        let members = seq
            .fields
            .iter()
            .map(|f| format!("{} {};", f.ty.to_writer_string(), f.name))
            .join("\n");

        // Generate class body.
        formatdoc! {
            r"
            {oneofs}

            {class_name}({param_list});

            {members}

            uint16_t static_size() const override;
            uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                                     uint8_t* dyn_cursor) const override;"
        }
    };

    // Generate full class code.
    formatdoc! {
        r"
        class {class_name} : public simplebuffers::SimpleBufferWriter {{
           public:
            {body}
        }};",
        body = indent_by(4, body.trim())
    }
}

/// Generates the C++ code for defining a oneof writer. This should be written as a subclass of a
/// sequence writer. Because oneofs can contain other oneofs as fields, we must recursively define
/// any oneof writers we find.
fn define_oneof_writer(oneof: &CppOneOf) -> String {
    // The full name of the oneof writer class, in the form "OneOfWriter".
    let class_name = oneof.to_writer_string();

    // Generate the public portion of the body. This is done separately so that we can trim it and
    // remove the extra white space generated when there are no oneof fields.
    let public_body = {
        // Generate class definitions of any oneof fields. These are subclasses of this oneof class
        // and are generated recursively.
        let oneofs = oneof.oneofs().map(define_oneof_writer).join("\n\n");

        // Generate a list of tags for the fields. These are members of the `Tag` enum class.
        let tags = oneof
            .fields
            .iter()
            .map(|f| format!("{} = {}", f.tag, f.index))
            .join(",\n");

        // Generate a list of possible values for each oneof field. These are held in a union and
        // are stored as pointers.
        let values = oneof
            .fields
            .iter()
            .map(|f| format!("{}* {};", f.ty.to_writer_string(), f.name))
            .join("\n");

        // Generate static "constructors" for this oneof class. There is one constructor for each
        // field. The true class constructor is private, so one of these must be called to
        // instantiate a class instance.
        let constructors = oneof
            .fields
            .iter()
            .map(|f| {
                format!(
                    "static {} {}({}* val);",
                    class_name,
                    f.constructor,
                    f.ty.to_writer_string()
                )
            })
            .join("\n");

        // Generate public body.
        formatdoc! {
            r"
            {oneofs}

            enum class Tag : uint8_t {{
                {tags}
            }};
            
            union Value {{
                {values}
            }};
            
            {constructors}

            uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                                     uint8_t* dyn_cursor) const override;",
            tags = indent_by(4, tags),
            values = indent_by(4, values)
        }
    };

    // Generate full class code.
    formatdoc! {
        r"
        class {class_name} : public simplebuffers::OneOfWriter {{
           public:
            {public_body}

           private:
            {class_name}(Tag tag, Value value);
        
            Tag tag;
            Value value;
        }};",
        public_body = indent_by(4, public_body.trim()),
    }
}

/// Generates the C++ code for forward declaring sequence readers.
fn forward_declare_sequence_reader(seq: &CppSequence) -> String {
    let case_corrected_name = seq.to_reader_string();
    format!("class {case_corrected_name};")
}

/// Generates the C++ code for defining sequence readers.
fn define_sequence_reader(seq: &CppSequence) -> String {
    // The full name of the sequence writer class, in the form "SequenceReader".
    let class_name = seq.to_reader_string();

    // Generate code to declare accessor functions for each field.
    let fields = seq
        .fields
        .iter()
        .map(|f| {
            format!(
                "{ty} {name}() const;",
                ty = f.ty.to_reader_string(),
                name = f.name
            )
        })
        .join("\n");

    // Generate full class code.
    formatdoc! {
        r"
        class {class_name} : public simplebuffers::SimpleBufferReader {{
           public:
            {fields}
            
            uint16_t static_size() const override;
        }};",
        fields = indent_by(4, fields.trim())
    }
}
