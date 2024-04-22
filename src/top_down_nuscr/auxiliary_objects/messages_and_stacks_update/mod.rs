use super::MessageParameters;
use std::collections::HashMap;

pub(crate) mod previous_message_wrt_clocks;
pub(crate) mod receiver_messages;
pub(crate) mod receiver_stacks;
pub(crate) mod sender_messages;
pub(crate) mod sender_stacks;

/// Use both functions #[messages]
/// and #[stacks] at once
pub(crate) fn messages_and_stacks(
    index: &[i32],
    messages: &mut HashMap<String, HashMap<String, Vec<String>>>,
    previous_message_wrt_clocks: &mut HashMap<
        String,
        HashMap<String, (String, String, String, String)>,
    >,
    last_message: &mut HashMap<String, HashMap<String, String>>,
    stacks: &mut HashMap<String, Vec<String>>,
    last_stack: &mut HashMap<String, String>,
    elts: &MessageParameters,
) -> Result<(), Box<dyn std::error::Error>> {
    let current_index_string = index
        .iter()
        .map(|&id| id.to_string())
        .collect::<Vec<_>>()
        .join("_");

    sender_messages::sender_messages(
        &current_index_string,
        messages,
        previous_message_wrt_clocks,
        last_message,
        elts,
    )?;
    receiver_messages::receiver_messages(
        &current_index_string,
        messages,
        previous_message_wrt_clocks,
        last_message,
        elts,
    )?;
    sender_stacks::sender_stacks(&current_index_string, stacks, last_stack, elts)?;
    receiver_stacks::receiver_stacks(&current_index_string, stacks, last_stack, elts)?;

    Ok(())
}
