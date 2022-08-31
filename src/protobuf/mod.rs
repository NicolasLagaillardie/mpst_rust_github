//! The module for handling
//! protobuf code generation

#[cfg(feature = "protobuf")]
#[allow(missing_docs)]
pub mod generated;

/*
#[cfg(test)]
mod tests {
    use super::*;

    use protobuf::Message;

    use std::fs::File;
    use std::path::Path;

    use generated::globaltype;

    /// Test function for extracting global protocol fields
    pub fn extract_global_protocol(global_protocol: globaltype::GlobalType) {
        println!("Test global protocol");
        println!("start: {:?}", global_protocol.start);
    }

    /// Test function for extracting action fields
    pub fn extract_action(action: globaltype::Action) {
        println!("Test action");
        println!("from_role: {:?}", action.from_role);
    }

    /// Test function for extracting index fields
    pub fn extract_index(index: globaltype::action::Index) {
        println!("Test label");
        println!("label: {:?}", index.label);
    }

    #[test]
    /// Test
    pub fn read_global_protocol_file(file_name: &str) {
        let mut file_bytes = File::open(&Path::new(file_name)).unwrap();

        // Test Action
        let test_action = globaltype::Action::parse_from_reader(&mut file_bytes).unwrap();
        println!("Test action: {:?}", &test_action);
        println!("Test action type_: {:?}", &test_action.type_);
        println!("Test action from_role: {:?}", &test_action.from_role);
        println!("Test action to_role: {:?}", &test_action.to_role);
        println!(
            "Test action continuations: {:?}",
            &test_action.continuations
        );
        println!("Test action lower_bound: {:?}", &test_action.lower_bound);
        println!("Test action include_lb: {:?}", &test_action.include_lb);
        println!("Test action upper_bound: {:?}", &test_action.upper_bound);
        println!("Test action include_ub: {:?}", &test_action.include_ub);
        println!("Test action reset_clock: {:?}", &test_action.reset_clock);

        // Test GlobalType
        let test_global_protocol =
            globaltype::GlobalType::parse_from_reader(&mut file_bytes).unwrap();
        println!(
            "Test global protocol start: {:?}",
            &test_global_protocol.start
        );
        println!(
            "Test global protocol actions: {:?}",
            &test_global_protocol.actions
        );
    }
}
*/
