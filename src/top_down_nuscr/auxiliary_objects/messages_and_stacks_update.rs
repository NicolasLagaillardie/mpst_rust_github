use super::MessageParameters;
use std::collections::HashMap;

/// Use both functions #[messages_update]
/// and #[stacks_update] at once
pub(crate) fn messages_and_stacks_update(
    messages: &mut HashMap<String, Vec<String>>,
    last_messages: &mut HashMap<String, String>,
    stacks: &mut HashMap<String, Vec<String>>,
    last_stacks: &mut HashMap<String, String>,
    elts: &MessageParameters,
) {
    messages_update(messages, last_messages, elts);
    stacks_update(stacks, last_stacks, elts);
}

/// Update messages_sender and last_messages_sender
fn messages_update(
    messages: &mut HashMap<String, Vec<String>>,
    last_messages: &mut HashMap<String, String>,
    elts: &MessageParameters,
) {
    let messages_sender = messages.get_mut(&elts.sender).unwrap();
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

    let last_messages_sender = last_messages.get_mut(&elts.sender).unwrap();

    *last_messages_sender = format!(
        "Message{}From{}To{}",
        size_messages_sender + 1,
        elts.sender,
        elts.receiver
    );
}

/// Update messages_sender and last_messages_sender
fn stacks_update(
    stacks: &mut HashMap<String, Vec<String>>,
    last_stacks: &mut HashMap<String, String>,
    elts: &MessageParameters,
) {
    let stack_sender = stacks.get_mut(&elts.sender).unwrap();
    let size_stack_sender = stack_sender.len();

    stack_sender.push(format!(
        "type Ordering{}By{} = Role{}<Ordering{}By{}>;",
        size_stack_sender,
        elts.sender,
        elts.receiver,
        size_stack_sender + 1,
        elts.sender,
    ));

    let last_stacks_sender = last_stacks.get_mut(&elts.sender).unwrap();

    *last_stacks_sender = format!("Ordering{}By{}", size_stack_sender + 1, elts.sender,);
}
