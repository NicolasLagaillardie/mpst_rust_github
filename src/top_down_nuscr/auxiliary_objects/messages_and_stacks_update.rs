use super::MessageParameters;
use std::collections::HashMap;

fn update_previous_message_wrt_clocks(
    previous_message_wrt_clocks: &mut HashMap<
        String,
        HashMap<String, (String, String, String, String)>,
    >,
    elts: &MessageParameters,
    role: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if elts.left_bound > elts.right_bound {
        return Err("Wrong time interval: a left bound is after its related right bound.".into());
    }

    if let Some(role_time_bounds) = previous_message_wrt_clocks.get_mut(role) {
        if let Some(previous_time_bound) = role_time_bounds.get_mut(&elts.clock) {
            if ((elts.left_bound.parse::<i32>().unwrap() + (elts.left_bracket == "[") as i32)
                < (previous_time_bound.2.parse::<i32>().unwrap()
                    + previous_time_bound.3.parse::<bool>().unwrap() as i32))
                && ((elts.right_bound.parse::<i32>().unwrap() + (elts.right_bracket == "]") as i32)
                    < (previous_time_bound.2.parse::<i32>().unwrap()
                        + previous_time_bound.3.parse::<bool>().unwrap() as i32))
            {
                return Err("Two of the consecutive time bounds are not feasible.".into());
            } else {
                role_time_bounds.insert(
                    elts.clock.to_string(),
                    (
                        elts.left_bound.to_string(),
                        (elts.left_bracket == "[").to_string(),
                        elts.right_bound.to_string(),
                        (elts.right_bracket == "]").to_string(),
                    ),
                );
            }
        } else {
            role_time_bounds.insert(
                elts.clock.to_string(),
                (
                    elts.left_bound.to_string(),
                    (elts.left_bracket == "[").to_string(),
                    elts.right_bound.to_string(),
                    (elts.right_bracket == "]").to_string(),
                ),
            );
        }
    } else {
        let mut temp_time_bound = HashMap::new();

        temp_time_bound.insert(
            elts.clock.to_string(),
            (
                elts.left_bound.to_string(),
                (elts.left_bracket == "[").to_string(),
                elts.right_bound.to_string(),
                (elts.right_bracket == "]").to_string(),
            ),
        );

        previous_message_wrt_clocks.insert(elts.receiver.to_string(), temp_time_bound);
    }

    Ok(())
}

/// Use both functions #[messages_update]
/// and #[stacks_update] at once
pub(crate) fn messages_and_stacks_update(
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

    messages_update(
        &current_index_string,
        messages,
        previous_message_wrt_clocks,
        last_message,
        elts,
    )?;
    stacks_update(&current_index_string, stacks, last_stack, elts)?;

    Ok(())
}

/// Update messages_sender and last_messages_sender
fn messages_update(
    current_index_string: &str,
    messages: &mut HashMap<String, HashMap<String, Vec<String>>>,
    previous_message_wrt_clocks: &mut HashMap<
        String,
        HashMap<String, (String, String, String, String)>,
    >,
    last_message: &mut HashMap<String, HashMap<String, String>>,
    elts: &MessageParameters,
) -> Result<(), Box<dyn std::error::Error>> {
    sender_messages_update(
        current_index_string,
        messages,
        previous_message_wrt_clocks,
        last_message,
        elts,
    )?;
    receiver_messages_update(
        current_index_string,
        messages,
        previous_message_wrt_clocks,
        last_message,
        elts,
    )?;

    Ok(())
}

/// Update messages for the sender
fn sender_messages_update(
    current_index_string: &str,
    messages: &mut HashMap<String, HashMap<String, Vec<String>>>,
    previous_message_wrt_clocks: &mut HashMap<
        String,
        HashMap<String, (String, String, String, String)>,
    >,
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

    update_previous_message_wrt_clocks(previous_message_wrt_clocks, elts, &elts.sender)?;

    Ok(())
}

/// Update messages for the receiver
fn receiver_messages_update(
    current_index_string: &str,
    messages: &mut HashMap<String, HashMap<String, Vec<String>>>,
    previous_message_wrt_clocks: &mut HashMap<
        String,
        HashMap<String, (String, String, String, String)>,
    >,
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

    update_previous_message_wrt_clocks(previous_message_wrt_clocks, elts, &elts.receiver)?;

    Ok(())
}

/// Update messages_sender and last_messages_sender
fn stacks_update(
    current_index_string: &str,
    stacks: &mut HashMap<String, Vec<String>>,
    last_stack: &mut HashMap<String, String>,
    elts: &MessageParameters,
) -> Result<(), Box<dyn std::error::Error>> {
    sender_stacks_update(current_index_string, stacks, last_stack, elts)?;
    receiver_stacks_update(current_index_string, stacks, last_stack, elts)?;

    Ok(())
}

/// Update stacks for the sender
fn sender_stacks_update(
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

/// Update stacks for the receiver
fn receiver_stacks_update(
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
