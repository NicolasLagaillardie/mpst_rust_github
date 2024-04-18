use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

/// Generate all imports in the output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_imports(
    output: &mut Option<File>,
    roles: &[String],
    is_recursive: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match output.as_mut() {
        Some(generated_file) => {
            // Generate the imports of necessary crates
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

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}

/// Generate all structs in the output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_structs(
    output: &mut Option<File>,
    payloads: &HashSet<String>,
    message_with_payloads: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    match output.as_mut() {
        Some(generated_file) => {
            // Generate the structs for the payloads
            for payload in payloads.iter() {
                writeln!(generated_file, "struct {};", payload)?;
            }
            for (name_message, payload) in message_with_payloads.iter() {
                if payload.is_empty() {
                    writeln!(generated_file, "struct {};", name_message)?;
                } else {
                    writeln!(
                        generated_file,
                        "struct {} {{ payload: {} }}",
                        name_message, payload
                    )?;
                }
            }
            writeln!(generated_file)?;

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}

/// Generate all sessions in the output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_sessions(
    output: &mut Option<File>,
    messages: &HashMap<String, HashMap<String, Vec<String>>>,
    last_messages: &HashMap<String, HashMap<String, String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    match output.as_mut() {
        Some(generated_file) => {
            // Generate the sessions
            for (role, channels) in messages.iter() {
                writeln!(generated_file, "// Binary sessions for {}", role)?;
                for (other_role, role_messages) in channels.iter() {
                    for message in role_messages {
                        writeln!(generated_file, "{}", message)?;
                    }
                    writeln!(
                        generated_file,
                        "type {} = End;",
                        last_messages.get(role).unwrap().get(other_role).unwrap()
                    )?;
                }
                writeln!(generated_file)?;
            }

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}

/// Generate all sessions in the output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_stacks(
    output: &mut Option<File>,
    stacks: &HashMap<String, Vec<String>>,
    last_stacks: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    match output.as_mut() {
        Some(generated_file) => {
            // Generate the stacks
            for (role, role_stacks) in stacks.iter() {
                writeln!(generated_file, "// Stacks for {}", role)?;
                for stack in role_stacks {
                    writeln!(generated_file, "{}", stack)?;
                }
                writeln!(
                    generated_file,
                    "type {} = RoleEnd;",
                    last_stacks.get(role).unwrap()
                )?;
                writeln!(generated_file)?;
            }

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}

/// Generate all endpoints in the output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_endpoints(
    output: &mut Option<File>,
    roles: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    match output.as_mut() {
        Some(generated_file) => {
            // Generate the endpoints
            for role in roles {
                writeln!(generated_file, "// Endpoint(s) for role {}", role)?;
                write!(generated_file, "type Endpoint0By{} = MeshedChannels<", role)?;
                for other_role in roles {
                    if other_role != role {
                        write!(generated_file, "Message0From{}To{}, ", role, other_role)?;
                    }
                }
                write!(generated_file, "Ordering0By{}", role)?;
                writeln!(generated_file, ">;",)?;
                writeln!(generated_file)?;
            }

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}

/// Generate all functions in the output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_fn(output: &mut Option<File>) -> Result<(), Box<dyn std::error::Error>> {
    match output.as_mut() {
        Some(generated_file) => {
            // Add the main function and invite dev to complete code
            writeln!(generated_file, "// Write your functions here.\n")?;
            writeln!(generated_file, "fn main(){{}}")?;

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}
