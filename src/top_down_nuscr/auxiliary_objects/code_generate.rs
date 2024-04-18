use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

/// Generate everything in the output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_everything(
    output: &mut Option<File>,
    roles: &[String],
    payloads: &HashSet<String>,
    message_with_payloads: &HashMap<String, String>,
    messages: &HashMap<String, Vec<String>>,
    last_messages: &HashMap<String, String>,
    is_recursive: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match output.as_mut() {
        Some(generated_file) => {
            // Write the imports of necessary crates
            writeln!(
                generated_file,
                "use mpstthree::binary::struct_trait::end::End;"
            )?;
            writeln!(
                generated_file,
                "use mpstthree::binary_atmp::struct_trait::{{recv::RecvTimed, send::SendTimed}};"
            )?;
            writeln!(generated_file, "use mpstthree::generate_atmp;")?;
            if is_recursive {
                writeln!(
                    generated_file,
                    "use mpstthree::role::broadcast::RoleBroadcast;"
                )?;
            }
            writeln!(generated_file, "use mpstthree::role::end::RoleEnd;")?;
            writeln!(generated_file, "use std::collections::HashMap;")?;
            writeln!(generated_file, "use std::error::Error;")?;
            writeln!(generated_file, "use std::time::Instant;\n")?;

            writeln!(
                generated_file,
                "generate_atmp!(MeshedChannels, {});\n",
                roles.join(", ")
            )?;

            for payload in payloads.iter() {
                writeln!(*generated_file, "struct {};", payload)?;
            }
            for (name_message, payload) in message_with_payloads.iter() {
                if payload.is_empty() {
                    writeln!(*generated_file, "struct {};", name_message)?;
                } else {
                    writeln!(
                        *generated_file,
                        "struct {} {{ payload: {} }}",
                        name_message, payload
                    )?;
                }
            }
            writeln!(generated_file)?;

            for (role, role_messages) in messages.iter() {
                for message in role_messages {
                    writeln!(*generated_file, "{}", message)?;
                }
                writeln!(
                    *generated_file,
                    "type {} = End;",
                    last_messages.get(role).unwrap()
                )?;
            }
            writeln!(generated_file)?;

            writeln!(generated_file, "// Write your functions here.\n")?;
            writeln!(generated_file, "fn main(){{}}")?;

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}
