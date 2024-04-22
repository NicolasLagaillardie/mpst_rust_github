use super::MessageParameters;
use std::collections::HashMap;

/// Update stacks for the receiver
pub(crate) fn receiver_stacks(
    current_index_string: &str,
    stacks: &mut HashMap<String, Vec<String>>,
    last_stack: &mut HashMap<String, String>,
    elts: &MessageParameters,
) -> Result<(), Box<dyn std::error::Error>> {
    let stack_receiver = stacks.get_mut(&elts.receiver).unwrap();
    let size_stack_receiver = stack_receiver.len();

    stack_receiver.push(format!(
        "type Ordering_{}_v_{}_For{} = Role{}<Ordering_{}_v_{}_For{}>;",
        current_index_string,
        size_stack_receiver,
        elts.receiver,
        elts.sender,
        current_index_string,
        size_stack_receiver + 1,
        elts.receiver,
    ));

    let last_stacks_receiver = last_stack.get_mut(&elts.receiver).unwrap();

    *last_stacks_receiver = format!(
        "Ordering_{}_v_{}_For{}",
        current_index_string,
        size_stack_receiver + 1,
        elts.receiver
    );

    Ok(())
}
