use super::super::{GlobalElements, Tree};
use std::io::Write;

/// Generate all imports in the global_elements.output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_imports(
    global_elements: &mut GlobalElements,
    main_tree: &Tree,
) -> Result<(), Box<dyn std::error::Error>> {
    match global_elements.output.as_mut() {
        Some(generated_file) => {
            // Generate the imports of necessary crates
            writeln!(
                generated_file,
                "#![allow(dead_code, non_camel_case_types, unused_variables)]"
            )?;
            writeln!(generated_file)?;
            writeln!(
                generated_file,
                "use mpstthree::binary::struct_trait::end::End;"
            )?;
            writeln!(
                generated_file,
                "use mpstthree::binary_atmp::struct_trait::{{recv::RecvTimed, send::SendTimed}};"
            )?;
            writeln!(generated_file, "use mpstthree::generate_atmp;")?;
            if !main_tree.sub_trees.is_empty() {
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
                global_elements.roles.join(", ")
            )?;

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}
