use colored::Colorize;
use convert_case::{Case, Casing};
use itertools::Itertools;
use simplebuffers_core::{Field, SBSchema, Type};
use std::fmt::{self};

enum ReserveCheckErrorTarget {
    Sequence,
    Enum,
    EnumVar,
    Field,
}

impl fmt::Display for ReserveCheckErrorTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReserveCheckErrorTarget::Sequence => "Sequence",
                ReserveCheckErrorTarget::Enum => "Enum",
                ReserveCheckErrorTarget::EnumVar => "Enum variant",
                ReserveCheckErrorTarget::Field => "Field",
            }
        )
    }
}

/// A struct to hold errors that occur during the check. The error contains a context stack, so it
/// can provide the full path to an erroneous field.
pub(super) struct ReserveCheckError<'n, 'm> {
    target: ReserveCheckErrorTarget,
    name_stack: Vec<String>,
    name: &'n str,
    matched: &'m str,
}

impl<'n, 'm> ReserveCheckError<'n, 'm> {
    /// Creates a new ReserveCheckError with an empty name stack.
    fn new(target: ReserveCheckErrorTarget, name: &'n str, matched: &'m str) -> Self {
        Self {
            target,
            name_stack: vec![],
            name,
            matched,
        }
    }

    /// Push a new name to the stack and return self.
    fn bubble(mut self, new_name: String) -> Self {
        self.name_stack.push(new_name);
        self
    }
}

impl<'n, 'm> fmt::Display for ReserveCheckError<'n, 'm> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.name_stack.is_empty() {
            write!(
                f,
                "{} {} `{}` matches reserved keyword `{}`",
                "ERROR:".red().bold(),
                self.target,
                self.name.cyan(),
                self.matched.blue().italic()
            )
        } else {
            write!(
                f,
                "{} {} `{}{}{}` matches reserved keyword `{}`",
                "ERROR:".red().bold(),
                self.target,
                self.name_stack.iter().rev().join("::").cyan(),
                "::".cyan(),
                self.name.cyan(),
                self.matched.blue().italic()
            )
        }
    }
}

/// Check if any reserved identifiers appear in a schema.
pub(super) fn check_reserved<'n, 'm>(
    schema: &'n SBSchema,
    reserved: &'m [String],
) -> Result<(), ReserveCheckError<'n, 'm>> {
    // Check if two identifiers match after adjusting case.
    fn find_match<'a>(name: &str, reserved: &'a [String]) -> Option<&'a String> {
        reserved
            .iter()
            .find(|r| name.to_case(Case::Snake) == r.to_case(Case::Snake))
    }

    // Recursive function to check if a field contains a reserved identifier.
    fn check_field<'n, 'm>(
        field: &'n Field,
        reserved: &'m [String],
    ) -> Result<(), ReserveCheckError<'n, 'm>> {
        if let Some(matched) = find_match(&field.name, reserved) {
            return Err(ReserveCheckError::new(
                ReserveCheckErrorTarget::Field,
                &field.name,
                matched,
            ));
        }

        if let Type::OneOf(subfields) = &field.ty {
            for f in subfields {
                check_field(f, reserved).map_err(|e| e.bubble(field.name.clone()))?;
            }
        }

        Ok(())
    }

    // Check all enums.
    for enm in &schema.enums {
        if let Some(matched) = find_match(&enm.name, reserved) {
            return Err(ReserveCheckError::new(
                ReserveCheckErrorTarget::Enum,
                &enm.name,
                matched,
            ));
        }
        for variant in &enm.variants {
            if let Some(matched) = find_match(&variant.name, reserved) {
                return Err(ReserveCheckError::new(
                    ReserveCheckErrorTarget::EnumVar,
                    &variant.name,
                    matched,
                )
                .bubble(enm.name.clone()));
            }
        }
    }

    // Check all sequences.
    for seq in &schema.sequences {
        if let Some(matched) = find_match(&seq.name, reserved) {
            return Err(ReserveCheckError::new(
                ReserveCheckErrorTarget::Sequence,
                &seq.name,
                matched,
            ));
        }
        for field in &seq.fields {
            check_field(field, reserved).map_err(|e| e.bubble(seq.name.clone()))?;
        }
    }

    Ok(())
}
