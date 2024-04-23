use super::{previous_message_wrt_clocks::fn_previous_message_wrt_clocks, MessageParameters};
use std::collections::HashMap;

/// Update messages for the sender
pub(crate) fn sender_messages(
    current_index_string: &str,
    messages: &mut HashMap<String, HashMap<String, Vec<String>>>,
    previous_message_wrt_clocks: &mut HashMap<String, HashMap<String, (String, String)>>,
    last_message: &mut HashMap<String, HashMap<String, String>>,
    elts: &MessageParameters,
) -> Result<(), Box<dyn std::error::Error>> {
    let channels_sender = messages.get_mut(&elts.sender).unwrap();
    let messages_sender = channels_sender.get_mut(&elts.receiver).unwrap();
    let size_messages_sender = messages_sender.len();

    messages_sender.push(
        format!(
            "type Message_{}_v_{}_From{}To{} = SendTimed<{}, '{}', {}, {}, {}, {}, '{}', Message_{}_v_{}_From{}To{}>;",
            current_index_string,
            size_messages_sender,
            elts.sender,
            elts.receiver,
            elts.message,
            elts.clock,
            elts.left_bound,
            elts.left_bracket == "[",
            elts.right_bound,
            elts.right_bracket == "]",
            elts.reset,
            current_index_string,
            size_messages_sender + 1,
            elts.sender,
            elts.receiver
        )
    );

    let last_channel_sender = last_message.get_mut(&elts.sender).unwrap();
    let last_messages_sender = last_channel_sender.get_mut(&elts.receiver).unwrap();

    *last_messages_sender = format!(
        "Message_{}_v_{}_From{}To{}",
        current_index_string,
        size_messages_sender + 1,
        elts.sender,
        elts.receiver
    );

    fn_previous_message_wrt_clocks(previous_message_wrt_clocks, elts, &elts.sender)?;

    Ok(())
}
