use super::MessageParameters;
use std::collections::HashMap;

/// Use both functions #[messages_update]
/// and #[stacks_update] at once
pub(crate) fn messages_and_stacks_update(
    messages: &mut HashMap<String, HashMap<String, Vec<String>>>,
    last_messages: &mut HashMap<String, HashMap<String, String>>,
    stacks: &mut HashMap<String, Vec<String>>,
    last_stacks: &mut HashMap<String, String>,
    elts: &MessageParameters,
) {
    messages_update(messages, last_messages, elts);
    stacks_update(stacks, last_stacks, elts);
}

/// Update messages_sender and last_messages_sender
fn messages_update(
    messages: &mut HashMap<String, HashMap<String, Vec<String>>>,
    last_messages: &mut HashMap<String, HashMap<String, String>>,
    elts: &MessageParameters,
) {
    sender_messages_update(messages, last_messages, elts);
    receiver_messages_update(messages, last_messages, elts);
}

/// Update messages for the sender
fn sender_messages_update(
    messages: &mut HashMap<String, HashMap<String, Vec<String>>>,
    last_messages: &mut HashMap<String, HashMap<String, String>>,
    elts: &MessageParameters,
) {
    let channels_sender = messages.get_mut(&elts.sender).unwrap();
    let messages_sender = channels_sender.get_mut(&elts.receiver).unwrap();
    let size_messages_sender = messages_sender.len();

    messages_sender.push(
        format!(
            "type Message{}From{}To{} = SendTimed<{}, '{}', {}, {}, {}, {}, '{}', Message{}From{}To{}>;",
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
            size_messages_sender + 1,
            elts.sender,
            elts.receiver
        )
    );

    let last_channel_sender = last_messages.get_mut(&elts.sender).unwrap();
    let last_messages_sender = last_channel_sender.get_mut(&elts.receiver).unwrap();

    *last_messages_sender = format!(
        "Message{}From{}To{}",
        size_messages_sender + 1,
        elts.sender,
        elts.receiver
    );
}

/// Update messages for the receiver
fn receiver_messages_update(
    messages: &mut HashMap<String, HashMap<String, Vec<String>>>,
    last_messages: &mut HashMap<String, HashMap<String, String>>,
    elts: &MessageParameters,
) {
    let channels_receiver = messages.get_mut(&elts.receiver).unwrap();
    let messages_receiver = channels_receiver.get_mut(&elts.sender).unwrap();
    let size_messages_receiver = messages_receiver.len();

    messages_receiver.push(
        format!(
            "type Message{}From{}To{} = RecvTimed<{}, '{}', {}, {}, {}, {}, '{}', Message{}From{}To{}>;",
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
            size_messages_receiver + 1,
            elts.receiver,
            elts.sender,
        )
    );

    let last_channel_receiver = last_messages.get_mut(&elts.receiver).unwrap();
    let last_messages_receiver = last_channel_receiver.get_mut(&elts.sender).unwrap();

    *last_messages_receiver = format!(
        "Message{}From{}To{}",
        size_messages_receiver + 1,
        elts.receiver,
        elts.sender,
    );
}

/// Update messages_sender and last_messages_sender
fn stacks_update(
    stacks: &mut HashMap<String, Vec<String>>,
    last_stacks: &mut HashMap<String, String>,
    elts: &MessageParameters,
) {
    sender_stacks_update(stacks, last_stacks, elts);
    receiver_stacks_update(stacks, last_stacks, elts);
}

/// Update stacks for the sender
fn sender_stacks_update(
    stacks: &mut HashMap<String, Vec<String>>,
    last_stacks: &mut HashMap<String, String>,
    elts: &MessageParameters,
) {
    let stack_sender = stacks.get_mut(&elts.sender).unwrap();
    let size_stack_sender = stack_sender.len();

    stack_sender.push(format!(
        "type Ordering{}For{} = Role{}<Ordering{}For{}>;",
        size_stack_sender,
        elts.sender,
        elts.receiver,
        size_stack_sender + 1,
        elts.sender,
    ));

    let last_stacks_sender = last_stacks.get_mut(&elts.sender).unwrap();

    *last_stacks_sender = format!("Ordering{}For{}", size_stack_sender + 1, elts.sender,);
}

/// Update stacks for the receiver
fn receiver_stacks_update(
    stacks: &mut HashMap<String, Vec<String>>,
    last_stacks: &mut HashMap<String, String>,
    elts: &MessageParameters,
) {
    let stack_receiver = stacks.get_mut(&elts.receiver).unwrap();
    let size_stack_receiver = stack_receiver.len();

    stack_receiver.push(format!(
        "type Ordering{}For{} = Role{}<Ordering{}For{}>;",
        size_stack_receiver,
        elts.receiver,
        elts.sender,
        size_stack_receiver + 1,
        elts.receiver,
    ));

    let last_stacks_receiver = last_stacks.get_mut(&elts.receiver).unwrap();

    *last_stacks_receiver = format!("Ordering{}For{}", size_stack_receiver + 1, elts.receiver,);
}
