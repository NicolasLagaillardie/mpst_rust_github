//! The module for handling
//! protobuf code generation

#[cfg(feature = "protobuf")]
#[allow(missing_docs)]
pub mod generated;

/// Test function for extracting global protocol fields
pub fn extract_global_protocol(global_protocol: generated::globaltype::GlobalType) {
    println!("Test global protocol");
    println!("start: {:?}", global_protocol.start);
}

/// Test function for extracting action fields
pub fn extract_action(action: generated::globaltype::Action) {
    println!("Test action");
    println!("from_role: {:?}", action.from_role);
}

/// Test function for extracting index fields
pub fn extract_index(index: generated::globaltype::action::Index) {
    println!("Test label");
    println!("label: {:?}", index.label);
}

/// Test
pub fn read_global_protocol_file() {}
