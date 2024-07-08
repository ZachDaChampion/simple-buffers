use simplebuffers_codegen::CodeGenerator;

macro_rules! register_internal_generators {
    ($($name:literal : $generator:ty);*) => {
        pub(crate) fn get_internal_generator(name: &str) -> Option<Box<dyn CodeGenerator>> {
            match (name) {
            $(
                $name => Some(Box::new(<$generator as CodeGenerator>::new())),
            );*
            _ => None
        }
    }
    };
}

register_internal_generators!(
    "sanitycheck": simplebuffers_sanitycheck::SanityCheckCodeGenerator
);
