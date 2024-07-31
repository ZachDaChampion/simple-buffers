# Compiler Usage

The SimpleBuffers compiler is invoked from the command line to generate code from your schema files.
The basic syntax is as follows:

```
simplebuffers [options] <generator> <schema_file> [generator-specific arguments]
```

- `<generator>`: Specifies the target language for code generation (e.g., cpp for C++).
- `<schema_file>`: Path to your SimpleBuffers schema file.

## Options

- `-l, --lib <path>`: Specify a custom library to load for third-party generators.
- `-s, --srcdir <path>`: Set the directory where your SimpleBuffers schema lives.
- `-d, --dstdir <path>`: Set the directory where generated files will be written.

## Generator-Specific Arguments

Different code generators may require or accept additional arguments. These are passed after the
main options and are specific to the chosen generator. The compiler passes these arguments directly
to the selected generator.

### Example: Using the C++ Generator

For the C++ generator, you might use a command like this:

```
simplebuffers -d ./output cpp myschema.sb --header-dir include
```

In this example:

- `-d ./output` specifies the output directory for the generated files
- `cpp` is the generator name
- `myschema.sb` is the input schema file
- `--header-dir include` is a C++ specific argument that determines the destination for generated
  header files

Note that the exact arguments accepted by the C++ generator may vary. Always refer to the specific
generator's documentation for the most up-to-date information on available options.

## Output

The compiler will generate language-specific files based on your schema. For C++, this typically
includes:

1. A header file (.hpp) in the specified header directory
2. A source file (.cpp) in the main output directory
3. A core library header file (simplebuffers.hpp) in the header directory

These files will contain the necessary classes and functions to serialize and deserialize your data
structures according to the SimpleBuffers schema.

Remember to include these generated files in your project and link against them as needed.

## Help

For up-to-date information about CLI usage and options, run:

```
simplebuffers --help
```

## Version Information

You can check the version of the SimpleBuffers compiler by running:

```
simplebuffers --version
```

This will display the current version of the compiler.
