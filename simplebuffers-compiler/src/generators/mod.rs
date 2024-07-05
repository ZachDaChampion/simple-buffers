mod sanitycheck;

use sanitycheck::SanityCheckCodeGenerator;
use simplebuffers_codegen::CodeGenerator;

type CodeGeneratorConstructor = fn() -> Box<dyn CodeGenerator>;

pub(crate) const GENERATORS: [(&str, CodeGeneratorConstructor); 1] =
    [("sanitycheck", || Box::new(SanityCheckCodeGenerator::new()))];
