use super::super::GlobalElements;
use std::io::Write;

/// Generate the main function in the global_elements.output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_fn_main(
    global_elements: &mut GlobalElements,
) -> Result<(), Box<dyn std::error::Error>> {
    match global_elements.output.as_mut() {
        Some(generated_file) => {
            // Add the main function and invite dev to complete code
            writeln!(generated_file, "fn main() {{")?;

            write!(generated_file, "\tlet (")?;

            for role in global_elements.roles.iter() {
                write!(generated_file, "thread_{}, ", role.to_lowercase())?;
            }

            write!(generated_file, ") = fork_mpst(")?;

            for role in global_elements.roles.iter() {
                write!(generated_file, "endpoint_{}_0_v_0, ", role.to_lowercase())?;
            }

            writeln!(generated_file, ");")?;
            writeln!(generated_file)?;

            for role in global_elements.roles.iter() {
                writeln!(
                    generated_file,
                    "\tprintln!(\"Thread {}: {{:?}}\", thread_{}.join());",
                    role,
                    role.to_lowercase()
                )?;
            }

            writeln!(generated_file, "}}")?;

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}
