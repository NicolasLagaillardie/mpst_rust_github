// #![allow(dead_code, unused_variables, clippy::needless_borrows_for_generic_args)]

//! TODO

use std::fs::File;

use std::io::{BufRead, BufReader};

use std::collections::{HashMap, HashSet};

use super::auxiliary_objects::{
    check_brackets::check_brackets, code_generate::*,
    messages_and_stacks_update::messages_and_stacks_update, regex::*, MessageParameters,
};

/// Generate endpoints from a nuscr file
/// with timed global protocol
pub fn generator(filepath: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut output: Option<File> = None;

    // Lists for elements to add to the output file
    let mut roles: Vec<String> = vec![];
    let mut payloads: HashSet<String> = HashSet::new();
    let mut message_with_payloads: HashMap<String, String> = HashMap::new();
    let mut choices: Vec<String> = vec![];
    let mut loops: Vec<String> = vec![];
    let mut messages: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    let mut last_messages: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut clocks: HashMap<String, Vec<String>> = HashMap::new();
    let mut stacks: HashMap<String, Vec<String>> = HashMap::new();
    let mut last_stacks: HashMap<String, String> = HashMap::new();

    let mut opening_brackets = 0;
    let mut closing_brackets = 0;
    let mut is_recursive = false;

    for (line_number, line) in reader.lines().enumerate() {
        let temp_line = line?;

        check_brackets(
            &mut opening_brackets,
            &mut closing_brackets,
            &temp_line,
            &line_number,
        )?;

        if check_global(&temp_line) && line_number == 0 {
            let captured_fields = GLOBAL_PROTOCOL.captures(&temp_line).unwrap();

            let name = &captured_fields["name"];

            if output.is_none() {
                output = Some(File::create(&format!("{}{}.rs", output_path, name))?);
            }

            for (_, [role]) in ROLE.captures_iter(&temp_line).map(|c| c.extract()) {
                roles.push(role.into());
                messages.insert(role.to_string(), HashMap::new());
                last_messages.insert(role.to_string(), HashMap::new());
                stacks.insert(role.to_string(), vec![]);
                last_stacks.insert(role.to_string(), format!("Ordering0For{}", role));
            }

            roles.sort();

            for (role, channels) in messages.iter_mut() {
                for other_role in roles.iter() {
                    if other_role != role {
                        channels.insert(other_role.to_string(), vec![]);
                    }
                }
            }

            for (role, channels) in last_messages.iter_mut() {
                for other_role in roles.iter() {
                    if other_role != role {
                        channels.insert(
                            other_role.to_string(),
                            format!("Message0From{}To{}", role, other_role),
                        );
                    }
                }
            }
        } else if !check_global(&temp_line) && line_number > 0 {
            if check_message(&temp_line) {
                let captured_fields = MESSAGE.captures(&temp_line).unwrap();

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

                // Add clock to clocks of sender and receiver
                if let Some(clocks_sender) = clocks.get_mut(sender) {
                    clocks_sender.push((clock).to_string());
                } else {
                    clocks.insert(sender.to_string(), vec![(clock).to_string()]);
                }
                if let Some(clocks_receiver) = clocks.get_mut(receiver) {
                    clocks_receiver.push((clock).to_string());
                } else {
                    clocks.insert(receiver.to_string(), vec![(clock).to_string()]);
                }

                if check_message_with_payload_and_resetting_clock(&temp_line) {
                    let captured_fields =
                        MESSAGE_WITH_PAYLOAD_AND_RESET.captures(&temp_line).unwrap();

                    let payload = &captured_fields["payload"];
                    let reset = &captured_fields["reset"];

                    payloads.insert(payload.into());
                    message_with_payloads.insert(message.into(), payload.into());

                    messages_and_stacks_update(
                        &mut messages,
                        &mut last_messages,
                        &mut stacks,
                        &mut last_stacks,
                        &MessageParameters {
                            sender: sender.to_string(),
                            receiver: receiver.to_string(),
                            message: message.to_string(),
                            clock: clock.to_string(),
                            left_bound: left_bound.to_string(),
                            left_bracket: left_bracket.to_string(),
                            right_bound: right_bound.to_string(),
                            right_bracket: right_bracket.to_string(),
                            reset: reset.to_string(),
                        },
                    );
                } else if check_message_with_resetting_clock(&temp_line) {
                    let captured_fields = MESSAGE_WITH_RESET.captures(&temp_line).unwrap();

                    let reset = &captured_fields["reset"];

                    message_with_payloads.insert(message.into(), "".into());

                    messages_and_stacks_update(
                        &mut messages,
                        &mut last_messages,
                        &mut stacks,
                        &mut last_stacks,
                        &MessageParameters {
                            sender: sender.to_string(),
                            receiver: receiver.to_string(),
                            message: message.to_string(),
                            clock: clock.to_string(),
                            left_bound: left_bound.to_string(),
                            left_bracket: left_bracket.to_string(),
                            right_bound: right_bound.to_string(),
                            right_bracket: right_bracket.to_string(),
                            reset: reset.to_string(),
                        },
                    );
                } else if check_message_with_payload(&temp_line) {
                    let captured_fields = MESSAGE_WITH_PAYLOAD.captures(&temp_line).unwrap();

                    let payload = &captured_fields["payload"];

                    payloads.insert(payload.into());
                    message_with_payloads.insert(message.into(), payload.into());

                    messages_and_stacks_update(
                        &mut messages,
                        &mut last_messages,
                        &mut stacks,
                        &mut last_stacks,
                        &MessageParameters {
                            sender: sender.to_string(),
                            receiver: receiver.to_string(),
                            message: message.to_string(),
                            clock: clock.to_string(),
                            left_bound: left_bound.to_string(),
                            left_bracket: left_bracket.to_string(),
                            right_bound: right_bound.to_string(),
                            right_bracket: right_bracket.to_string(),
                            reset: " ".to_string(),
                        },
                    );
                } else {
                    message_with_payloads.insert(message.into(), "".into());

                    messages_and_stacks_update(
                        &mut messages,
                        &mut last_messages,
                        &mut stacks,
                        &mut last_stacks,
                        &MessageParameters {
                            sender: sender.to_string(),
                            receiver: receiver.to_string(),
                            message: message.to_string(),
                            clock: clock.to_string(),
                            left_bound: left_bound.to_string(),
                            left_bracket: left_bracket.to_string(),
                            right_bound: right_bound.to_string(),
                            right_bracket: right_bracket.to_string(),
                            reset: " ".to_string(),
                        },
                    );
                }
            }

            if check_choice(&temp_line) {
                let captured_fields = CHOICE.captures(&temp_line).unwrap();

                choices.push(captured_fields["choice"].to_string());
            } else if check_or(&temp_line) {
            } else if check_rec(&temp_line) {
                is_recursive = true;

                let captured_fields = REC.captures(&temp_line).unwrap();

                loops.push(captured_fields["loop"].to_string());
            } else if check_continue(&temp_line) {
                let captured_fields = CONTINUE.captures(&temp_line).unwrap();

                if !loops.contains(&(captured_fields["loop"].to_string())) {
                    return Err(format!(
                        "There is a continue loop without an initialisation. See line: {}",
                        line_number
                    )
                    .into());
                }
            }
        } else {
            return Err("This is not a timed global protocol.".into());
        }
    }

    if opening_brackets != closing_brackets {
        return Err(
            "The number of opening and closing brackets is not the same at the end of the process."
                .into(),
        );
    }

    generate_imports(&mut output, &roles, is_recursive)?;

    generate_structs(&mut output, &payloads, &message_with_payloads)?;

    generate_sessions(&mut output, &messages, &last_messages)?;

    generate_stacks(&mut output, &stacks, &last_stacks)?;

    generate_endpoints(&mut output, &roles)?;

    generate_fn(&mut output)?;

    Ok(())
}
