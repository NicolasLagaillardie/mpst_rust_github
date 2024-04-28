use super::MessageParameters;
use std::collections::HashMap;

/// Update stacks for the sender
pub(crate) fn sender_stacks(
    current_index_string: &str,
    stacks: &mut HashMap<String, Vec<String>>,
    last_stack: &mut HashMap<String, String>,
    elts: &MessageParameters,
) -> Result<(), Box<dyn std::error::Error>> {
    let stack_sender = stacks.get_mut(&elts.sender).unwrap();
    let size_stack_sender = stack_sender.len();

    stack_sender.push(format!(
        "type Ordering_{}_v_{}_For{} = Role{}<Ordering_{}_v_{}_For{}>;",
        current_index_string,
        size_stack_sender,
        elts.sender,
        elts.receiver,
        current_index_string,
        size_stack_sender + 1,
        elts.sender,
    ));

    let last_stacks_sender = last_stack.get_mut(&elts.sender).unwrap();

    *last_stacks_sender = format!(
        "Ordering_{}_v_{}_For{}",
        current_index_string,
        size_stack_sender + 1,
        elts.sender
    );

    Ok(())
}
