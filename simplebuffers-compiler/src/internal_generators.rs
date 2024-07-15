//! Generates a list of all generators being bundled with the compiler. See [get_internal_generator]
//! for more information.

use simplebuffers_codegen::CodeGenerator;

macro_rules! register_internal_generators {
    ($($name:literal : $generator:ty),*) => {
        #[doc = concat!(
            "Matches bundled generators by name.\n\n",
            "# Arguments\n\n",
            "* `name` - The name of the generator to lookup.\n\n",
            "# Returns\n\n",
            "If found, `Some(<boxed pointer to the matching generator>)`. Otherwise, `None`.\n\n",
            "# Bundled Generators\n\n",
            "| Name | Implementation |\n",
            "| ---- | -------------- |\n",
            $("| ", $name, " | [`", stringify!($generator), "`] |\n",)*
        )]

        pub(crate) fn get_internal_generator(name: &str) -> Option<Box<dyn CodeGenerator>> {
            match (name) {
                $(
                    $name => Some(Box::new(<$generator as CodeGenerator>::new())),
                )*
                _ => None
            }
        }
    };
}

register_internal_generators!(
    "sanitycheck": simplebuffers_sanitycheck::SanityCheckCodeGenerator,
    "c++": simplebuffers_cpp::CPPCodeGenerator,
    "cpp": simplebuffers_cpp::CPPCodeGenerator
);
