use super::MessageParameters;
use std::collections::HashMap;

/// Update messages for the receiver
pub(crate) fn receiver_messages(
    current_index_string: &str,
    messages: &mut HashMap<String, HashMap<String, Vec<String>>>,
    last_message: &mut HashMap<String, HashMap<String, String>>,
    elts: &MessageParameters,
) -> Result<(), Box<dyn std::error::Error>> {
    let channels_receiver = messages.get_mut(&elts.receiver).unwrap();
    let messages_receiver = channels_receiver.get_mut(&elts.sender).unwrap();
    let size_messages_receiver = messages_receiver.len();

    messages_receiver.push(
        format!(
            "type Message_{}_v_{}_From{}To{} = RecvTimed<{}, '{}', {}, {}, {}, {}, '{}', Message_{}_v_{}_From{}To{}>;",
            current_index_string,
            size_messages_receiver,
            elts.receiver,
            elts.sender,
            elts.message,
            elts.clock,
            elts.left_bound,
            elts.left_bracket == "[",
            elts.right_bound,
            elts.right_bracket == "]",
            elts.reset,
            current_index_string,
            size_messages_receiver + 1,
            elts.receiver,
            elts.sender,
        )
    );

    let last_channel_receiver = last_message.get_mut(&elts.receiver).unwrap();
    let last_messages_receiver = last_channel_receiver.get_mut(&elts.sender).unwrap();

    *last_messages_receiver = format!(
        "Message_{}_v_{}_From{}To{}",
        current_index_string,
        size_messages_receiver + 1,
        elts.receiver,
        elts.sender,
    );

    Ok(())
}
