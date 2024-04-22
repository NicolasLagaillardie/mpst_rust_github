#![warn(clippy::too_many_arguments)]

use super::regex::{
    check_message_with_payload, check_message_with_payload_and_resetting_clock,
    check_message_with_resetting_clock, MESSAGE, MESSAGE_WITH_PAYLOAD,
    MESSAGE_WITH_PAYLOAD_AND_RESET, MESSAGE_WITH_RESET,
};
use super::{messages_and_stacks_update::messages_and_stacks_update, MessageParameters, Tree};
use std::collections::{HashMap, HashSet};

/// Check when the current line is a message
pub(crate) fn update_messages(
    temp_line: &str,
    roles: &[String],
    line_number: &usize,
    clocks: &mut HashMap<String, HashSet<String>>,
    payloads: &mut HashSet<String>,
    main_tree: &mut Tree,
) -> Result<(), Box<dyn std::error::Error>> {
    let captured_fields = MESSAGE.captures(temp_line).unwrap();

    let message = &captured_fields["message"];
    let sender = &captured_fields["sender"];
    let receiver = &captured_fields["receiver"];
    let left_bracket = &captured_fields["left_bracket"];
    let left_bound = &captured_fields["left_bound"];
    let right_bound = &captured_fields["right_bound"];
    let right_bracket = &captured_fields["right_bracket"];
    let clock = &captured_fields["clock"];

    // Check if sender and receiver exist in roles
    if !roles.contains(&String::from(sender)) {
        return Err(format!(
            "{} is not in the roles: {:?}. See line: {}.",
            sender, roles, line_number
        )
        .into());
    }
    if !roles.contains(&String::from(receiver)) {
        return Err(format!(
            "{} is not in the roles: {:?}. See line : {}.",
            receiver, roles, line_number
        )
        .into());
    }
    if sender == receiver {
        return Err(format!(
            "Sender and receiver must be different. See line: {}",
            line_number
        )
        .into());
    }

    // Check if bounds are sound
    if left_bound > right_bound && (left_bracket == "]" || right_bracket == "[") {
        return Err(format!("The time bounds are not sound in line: {}", line_number).into());
    }

    // Add clock to clocks of sender and receiver
    if let Some(clocks_sender) = clocks.get_mut(sender) {
        clocks_sender.insert((clock).to_string());
    } else {
        let mut temp_hashset = HashSet::new();
        temp_hashset.insert((clock).to_string());
        clocks.insert(sender.to_string(), temp_hashset);
    }
    if let Some(clocks_receiver) = clocks.get_mut(receiver) {
        clocks_receiver.insert((clock).to_string());
    } else {
        let mut temp_hashset = HashSet::new();
        temp_hashset.insert((clock).to_string());
        clocks.insert(receiver.to_string(), temp_hashset);
    }

    let mut message_parameters = MessageParameters {
        sender: sender.to_string(),
        receiver: receiver.to_string(),
        message: message.to_string(),
        clock: clock.to_string(),
        left_bound: left_bound.to_string(),
        left_bracket: left_bracket.to_string(),
        right_bound: right_bound.to_string(),
        right_bracket: right_bracket.to_string(),
        reset: " ".to_string(),
    };

    if check_message_with_payload_and_resetting_clock(temp_line) {
        let captured_fields = MESSAGE_WITH_PAYLOAD_AND_RESET.captures(temp_line).unwrap();

        let payload = &captured_fields["payload"];
        let reset = &captured_fields["reset"];

        payloads.insert(payload.into());
        main_tree
            .message_with_payloads
            .insert(message.into(), payload.into());

        message_parameters.reset = reset.to_string();

        messages_and_stacks_update(
            &main_tree.index,
            &mut main_tree.messages,
            &mut main_tree.previous_message_wrt_clocks,
            &mut main_tree.last_message,
            &mut main_tree.stacks,
            &mut main_tree.last_stack,
            &message_parameters,
        )?;
    } else if check_message_with_resetting_clock(temp_line) {
        let captured_fields = MESSAGE_WITH_RESET.captures(temp_line).unwrap();

        let reset = &captured_fields["reset"];

        main_tree
            .message_with_payloads
            .insert(message.into(), "".into());

        message_parameters.reset = reset.to_string();

        messages_and_stacks_update(
            &main_tree.index,
            &mut main_tree.messages,
            &mut main_tree.previous_message_wrt_clocks,
            &mut main_tree.last_message,
            &mut main_tree.stacks,
            &mut main_tree.last_stack,
            &message_parameters,
        )?;
    } else if check_message_with_payload(temp_line) {
        let captured_fields = MESSAGE_WITH_PAYLOAD.captures(temp_line).unwrap();

        let payload = &captured_fields["payload"];

        payloads.insert(payload.into());
        main_tree
            .message_with_payloads
            .insert(message.into(), payload.into());

        messages_and_stacks_update(
            &main_tree.index,
            &mut main_tree.messages,
            &mut main_tree.previous_message_wrt_clocks,
            &mut main_tree.last_message,
            &mut main_tree.stacks,
            &mut main_tree.last_stack,
            &message_parameters,
        )?;
    } else {
        main_tree
            .message_with_payloads
            .insert(message.into(), "".into());

        messages_and_stacks_update(
            &main_tree.index,
            &mut main_tree.messages,
            &mut main_tree.previous_message_wrt_clocks,
            &mut main_tree.last_message,
            &mut main_tree.stacks,
            &mut main_tree.last_stack,
            &message_parameters,
        )?;
    }

    Ok(())
}
