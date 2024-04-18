// #![allow(dead_code, unused_variables, clippy::needless_borrows_for_generic_args)]

//! TODO

use std::fs::File;

use std::io::{BufRead, BufReader, Error, Write};

use crate::macros_simple::role;

use {once_cell::sync::Lazy, regex::Regex};

use std::collections::{HashMap, HashSet};

static GLOBAL_PROTOCOL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"timed( +)global( +)protocol( +)(?P<name>\w+)\(((role( +)\w+( *),?( *))+)\)( *)\{")
        .unwrap()
});

static ROLE: Lazy<Regex> = Lazy::new(|| Regex::new(r"role (?P<name>\w+)").unwrap());

static CHOICE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"choice( +)at( +)(?P<choice>\w+)( *)\{?").unwrap());

static OR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\}( *)or( *)\{?").unwrap());

static MESSAGE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<message>\w+)\(\w*\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)(?P<left_bracket>(\[|\]))( *)(?P<left_bound>\d+)( *);( *)(?P<right_bound>\d+)( *)(?P<right_bracket>(\[|\]))( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\(\w*\)( *);").unwrap()
});

static MESSAGE_WITH_PAYLOAD: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<message>\w+)\((?P<payload>\w+)\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)(?P<left_bracket>(\[|\]))( *)(?P<left_bound>\d+)( *);( *)(?P<right_bound>\d+)( *)(?P<right_bracket>(\[|\]))( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\(\)( *);").unwrap()
});

static MESSAGE_WITH_RESET: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<message>\w+)\(\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)(?P<left_bracket>(\[|\]))( *)(?P<left_bound>\d+)( *);( *)(?P<right_bound>\d+)( *)(?P<right_bracket>(\[|\]))( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\((?P<reset>\w+)\)( *);").unwrap()
});

static MESSAGE_WITH_PAYLOAD_AND_RESET: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<message>\w+)\((?P<payload>\w+)\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)(?P<left_bracket>(\[|\]))( *)(?P<left_bound>\d+)( *);( *)(?P<right_bound>\d+)( *)(?P<right_bracket>(\[|\]))( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\((?P<reset>\w+)\)( *);").unwrap()
});

static REC: Lazy<Regex> = Lazy::new(|| Regex::new(r"rec( +)(?P<loop>\w+)").unwrap());

static CONTINUE: Lazy<Regex> = Lazy::new(|| Regex::new(r"continue( +)(?P<loop>\w+)( *)").unwrap());

/// Check whether the input is the first line of any new nuscrT protocol
fn check_global(input: &str) -> bool {
    GLOBAL_PROTOCOL.is_match(input)
}

/// Check each role
fn check_role(input: &str) -> bool {
    ROLE.is_match(input)
}

/// Check whether this is a choice
fn check_choice(input: &str) -> bool {
    CHOICE.is_match(input)
}

/// Check whether this is a branching
fn check_or(input: &str) -> bool {
    OR.is_match(input)
}

/// Check whether this is a shared message
fn check_message(input: &str) -> bool {
    MESSAGE.is_match(input)
}

/// Check whether this is a shared message with payload
fn check_message_with_payload(input: &str) -> bool {
    MESSAGE_WITH_PAYLOAD.is_match(input)
}

/// Check whether this is a shared message with resetting clock
fn check_message_with_resetting_clock(input: &str) -> bool {
    MESSAGE_WITH_RESET.is_match(input)
}

/// Check whether this is a shared message with payload and resetting clock
fn check_message_with_payload_and_resetting_clock(input: &str) -> bool {
    MESSAGE_WITH_PAYLOAD_AND_RESET.is_match(input)
}

/// Check whether this is the start of a recursive loop
fn check_rec(input: &str) -> bool {
    REC.is_match(input)
}

/// Check whether this is the continue of a recursive loop
fn check_continue(input: &str) -> bool {
    CONTINUE.is_match(input)
}

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
    let mut messages: HashMap<String, Vec<String>> = HashMap::new();
    let mut last_messages: HashMap<String, String> = HashMap::new();
    let mut clocks: HashMap<String, Vec<String>> = HashMap::new();

    let mut opening_brackets = 0;
    let mut closing_brackets = 0;

    for (line_number, line) in reader.lines().enumerate() {
        let temp_line = line?;

        println!(
            "global: {} / role: {} / choice: {} / or: {} / message: {} / message payload: {} / message reset: {} / message payload and reset: {} / rec: {} / continue: {}",
            check_global(&temp_line),
            check_role(&temp_line),
            check_choice(&temp_line),
            check_or(&temp_line),
            check_message(&temp_line),
            check_message_with_payload(&temp_line),
            check_message_with_resetting_clock(&temp_line),
            check_message_with_payload_and_resetting_clock(&temp_line),
            check_rec(&temp_line),
            check_continue(&temp_line)
        );
        println!("{}", &temp_line);

        opening_brackets += temp_line.matches("{").count();
        closing_brackets += temp_line.matches("}").count();

        if opening_brackets < closing_brackets {
            return Err("There are too many closing brackets".into());
        }

        if check_global(&temp_line) && line_number == 0 {
            let captured_fields = GLOBAL_PROTOCOL.captures(&temp_line).unwrap();

            let name = &captured_fields["name"];

            if output.is_none() {
                output = Some(File::create(&format!("{}{}.rs", output_path, name))?);
            }

            for (_, [role]) in ROLE.captures_iter(&temp_line).map(|c| c.extract()) {
                roles.push(role.into());
                last_messages.insert(role.to_string(), format!("Message0From{}", role).into());
                messages.insert(role.to_string(), vec![]);
            }

            match output.as_mut() {
                Some(generated_file) => {
                    // Write the imports of necessary crates
                    write!(
                        generated_file,
                        "use mpstthree::binary::struct_trait::end::End;\n"
                    )?;
                    write!(
                    generated_file,
                    "use mpstthree::binary_atmp::struct_trait::{{recv::RecvTimed, send::SendTimed}};\n"
                )?;
                    write!(generated_file, "use mpstthree::generate_atmp;\n")?;
                    write!(
                        generated_file,
                        "use mpstthree::role::broadcast::RoleBroadcast;\n"
                    )?;
                    write!(generated_file, "use mpstthree::role::end::RoleEnd;\n")?;
                    write!(generated_file, "use std::collections::HashMap;\n")?;
                    write!(generated_file, "use std::error::Error;\n")?;
                    write!(generated_file, "use std::time::Instant;\n\n")?;

                    write!(
                        generated_file,
                        "{}",
                        format!("generate_atmp!(MeshedChannels, {});\n\n", roles.join(", "))
                    )?;
                }
                None => return Err("Generated file was not initialised.".into()),
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
                    return Err(format!("{} is not in the roles: {:?}", sender, roles).into());
                }
                if !roles.contains(&String::from(receiver)) {
                    return Err(format!("{} is not in the roles: {:?}", receiver, roles).into());
                }
                if sender == receiver {
                    return Err("Sender and receiver must be different".into());
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

                    let messages_sender = messages.get_mut(sender).unwrap();
                    let size_messages_sender = messages_sender.len();

                    messages_sender.push(
                            format!(
                                "type Message{}From{}To{} = SendTimed<{}, '{}', {}, {}, {}, {}, '{}', Message{}From{}To{}>;",
                                size_messages_sender,
                                sender,
                                receiver,
                                message,
                                clock,
                                left_bound,
                                left_bracket == "[",
                                right_bound,
                                right_bracket == "]",
                                reset,
                                size_messages_sender + 1,
                                sender,
                                receiver
                            ).into()
                        );

                    let last_messages_sender = last_messages.get_mut(sender).unwrap();

                    *last_messages_sender = String::from(format!(
                        "Message{}From{}To{}",
                        size_messages_sender + 1,
                        sender,
                        receiver
                    ));
                } else if check_message_with_resetting_clock(&temp_line) {
                    let captured_fields = MESSAGE_WITH_RESET.captures(&temp_line).unwrap();

                    let reset = &captured_fields["reset"];

                    message_with_payloads.insert(message.into(), "".into());

                    let messages_sender = messages.get_mut(sender).unwrap();
                    let size_messages_sender = messages_sender.len();

                    messages_sender.push(
                            format!(
                                "type Message{}From{}To{} = SendTimed<{}, '{}', {}, {}, {}, {}, '{}', Message{}From{}To{}>;",
                                size_messages_sender,
                                sender,
                                receiver,
                                message,
                                clock,
                                left_bound,
                                left_bracket == "[",
                                right_bound,
                                right_bracket == "]",
                                reset,
                                size_messages_sender + 1,
                                sender,
                                receiver
                            ).into()
                        );

                    let last_messages_sender = last_messages.get_mut(sender).unwrap();

                    *last_messages_sender = String::from(format!(
                        "Message{}From{}To{}",
                        size_messages_sender + 1,
                        sender,
                        receiver
                    ));
                } else if check_message_with_payload(&temp_line) {
                    let captured_fields = MESSAGE_WITH_PAYLOAD.captures(&temp_line).unwrap();

                    let payload = &captured_fields["payload"];

                    payloads.insert(payload.into());
                    message_with_payloads.insert(message.into(), payload.into());

                    let messages_sender = messages.get_mut(sender).unwrap();
                    let size_messages_sender = messages_sender.len();

                    messages_sender.push(
                            format!(
                                "type Message{}From{}To{} = SendTimed<{}, '{}', {}, {}, {}, {}, ' ', Message{}From{}To{}>;",
                                size_messages_sender,
                                sender,
                                receiver,
                                message,
                                clock,
                                left_bound,
                                left_bracket == "[",
                                right_bound,
                                right_bracket == "]",
                                size_messages_sender + 1,
                                sender,
                                receiver
                            ).into()
                        );

                    let last_messages_sender = last_messages.get_mut(sender).unwrap();

                    *last_messages_sender = String::from(format!(
                        "Message{}From{}To{}",
                        size_messages_sender + 1,
                        sender,
                        receiver
                    ));
                } else {
                    message_with_payloads.insert(message.into(), "".into());

                    let messages_sender = messages.get_mut(sender).unwrap();
                    let size_messages_sender = messages_sender.len();

                    messages_sender.push(
                            format!(
                                "type Message{}From{}To{} = SendTimed<{}, '{}', {}, {}, {}, {}, ' ', Message{}From{}To{}>;",
                                size_messages_sender,
                                sender,
                                receiver,
                                message,
                                clock,
                                left_bound,
                                left_bracket == "[",
                                right_bound,
                                right_bracket == "]",
                                size_messages_sender + 1,
                                sender,
                                receiver
                            ).into()
                        );

                    let last_messages_sender = last_messages.get_mut(sender).unwrap();

                    *last_messages_sender = String::from(format!(
                        "Message{}From{}To{}",
                        size_messages_sender + 1,
                        sender,
                        receiver
                    ));
                }
            }

            if check_choice(&temp_line) {
            } else if check_or(&temp_line) {
            } else if check_rec(&temp_line) {
            } else if check_continue(&temp_line) {
            }
        } else {
            return Err("This is not a timed global protocol.".into());
        }
    }

    match output.as_mut() {
        Some(generated_file) => {
            for payload in payloads.into_iter() {
                write!(*generated_file, "{}", format!("struct {};\n", payload))?;
            }
            for (name_message, payload) in message_with_payloads.into_iter() {
                if payload.is_empty() {
                    write!(*generated_file, "{}", format!("struct {};\n", name_message))?;
                } else {
                    write!(
                        *generated_file,
                        "{}",
                        format!("struct {} {{ payload: {} }}\n", name_message, payload)
                    )?;
                }
            }
            write!(generated_file, "\n")?;

            for (role, role_messages) in messages.into_iter() {
                for message in role_messages {
                    write!(*generated_file, "{}", format!("{}\n", message))?;
                }
                write!(
                    *generated_file,
                    "type {} = End;\n",
                    format!("{}", last_messages.get(&role).unwrap())
                )?;
            }
            write!(generated_file, "\n")?;

            write!(generated_file, "// Write your functions here.\n\n")?;
            write!(generated_file, "fn main(){{}}")?;
        }
        None => return Err("Generated file was not initialised.".into()),
    }

    if opening_brackets != closing_brackets {
        return Err(
            "The number of opening and closing brackets is not the same at the end of the process."
                .into(),
        );
    }

    Ok(())
}

#[cfg(test)]
mod test_check_generator {
    use super::*;
    use rand::{distributions::Alphanumeric, Rng};

    #[test]
    fn test_check_global() {
        let random_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();

        assert!(check_global(&format!(
            "timed global protocol {}(role A) {{",
            &random_string
        )));
        assert!(check_global(&format!(
            "timed global protocol {}(role A, role B, role C) {{",
            &random_string
        )));
        assert!(check_global(&format!(
            "timed global protocol {}(role A, role B, role C,) {{",
            &random_string
        )));
        assert!(check_global(&format!(
            "timed global protocol {}(role A   , role B, role    C,  ) {{",
            &random_string
        )));
        assert!(!check_global(&format!(
            "global protocol {}(role A,) {{",
            &random_string
        )));
        assert!(!check_global(&format!(
            "global protocol {}(role A, role B, role C) {{",
            &random_string
        )));
    }

    #[test]
    fn test_check_role() {
        let random_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();

        assert!(check_role(&format!("role {}", &random_string)));
        assert!(!check_role(&format!("r {}", &random_string)));
        assert!(!check_role(&format!("ro {}", &random_string)));
        assert!(!check_role(&format!("rol {}", &random_string)));
    }

    #[test]
    fn test_check_choice() {
        let random_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();

        assert!(check_choice(&format!("choice at {}", &random_string)));
        assert!(check_choice(&format!("choice at {}{{", &random_string)));
        assert!(check_choice(&format!("choice at {} {{", &random_string)));
        assert!(!check_choice(&format!("choice t {} {{", &random_string)));
        assert!(!check_choice(&format!("offer at {} {{", &random_string)));
    }

    #[test]
    fn test_check_or() {
        let random_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();

        assert!(check_or("}}or{{"));
        assert!(check_or("}} or{{"));
        assert!(check_or("}}or {{"));
        assert!(check_or("}}   or   {{"));
        assert!(check_or("   }} or   {{    "));
        assert!(check_or(&format!("   }} or {}  {{    ", &random_string)));
    }

    #[test]
    fn test_check_message() {
        let random_string_message = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_payload = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_sender = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_receiver = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_left = rand::thread_rng().gen_range(1..=100);
        let random_string_right = rand::thread_rng().gen_range(1..=100);
        let random_string_clock = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_reset = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();

        assert!(check_message(&format!(
            "{}() from {} to {} within [{};{}] using {} and resetting ();",
            &random_string_message,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock
        )));

        assert!(check_message(&format!(
            "{}() from {} to {} within ]{};{}] using {} and resetting ();",
            &random_string_message,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock
        )));

        assert!(check_message(&format!(
            "{}() from {} to {} within [{};{}[ using {} and resetting ();",
            &random_string_message,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock
        )));

        assert!(check_message(&format!(
            "{}() from {} to {} within ]{};{}[ using {} and resetting ();",
            &random_string_message,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock
        )));

        assert!(check_message(&format!(
            "{}({}) from {} to {} within [{};{}] using {} and resetting ();",
            &random_string_message,
            &random_string_payload,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock
        )));

        assert!(check_message(&format!(
            "{}() from {} to {} within [{};{}] using {} and resetting ({});",
            &random_string_message,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock,
            &random_string_reset
        )));

        assert!(check_message(&format!(
            "{}({}) from {} to {} within [{};{}] using {} and resetting ({});",
            &random_string_message,
            &random_string_payload,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock,
            &random_string_reset
        )));
    }

    #[test]
    fn test_check_message_with_payload() {
        let random_string_message = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_payload = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_sender = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_receiver = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_left = rand::thread_rng().gen_range(1..=100);
        let random_string_right = rand::thread_rng().gen_range(1..=100);
        let random_string_clock = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_reset = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();

        assert!(!check_message_with_payload(&format!(
            "{}() from {} to {} within [{};{}] using {} and resetting ();",
            &random_string_message,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock
        )));

        assert!(check_message_with_payload(&format!(
            "{}({}) from {} to {} within [{};{}] using {} and resetting ();",
            &random_string_message,
            &random_string_payload,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock
        )));

        assert!(check_message_with_payload(&format!(
            "{}({}) from {} to {} within ]{};{}] using {} and resetting ();",
            &random_string_message,
            &random_string_payload,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock
        )));

        assert!(check_message_with_payload(&format!(
            "{}({}) from {} to {} within [{};{}[ using {} and resetting ();",
            &random_string_message,
            &random_string_payload,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock
        )));

        assert!(check_message_with_payload(&format!(
            "{}({}) from {} to {} within ]{};{}[ using {} and resetting ();",
            &random_string_message,
            &random_string_payload,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock
        )));

        assert!(!check_message_with_payload(&format!(
            "{}() from {} to {} within [{};{}] using {} and resetting ({});",
            &random_string_message,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock,
            &random_string_reset
        )));

        assert!(!check_message_with_payload(&format!(
            "{}({}) from {} to {} within [{};{}] using {} and resetting ({});",
            &random_string_message,
            &random_string_payload,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock,
            &random_string_reset
        )));
    }

    #[test]
    fn test_check_message_with_resetting_clock() {
        let random_string_message = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_payload = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_sender = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_receiver = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_left = rand::thread_rng().gen_range(1..=100);
        let random_string_right = rand::thread_rng().gen_range(1..=100);
        let random_string_clock = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_reset = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();

        assert!(!check_message_with_resetting_clock(&format!(
            "{}() from {} to {} within [{};{}] using {} and resetting ();",
            &random_string_message,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock
        )));

        assert!(!check_message_with_resetting_clock(&format!(
            "{}({}) from {} to {} within [{};{}] using {} and resetting ();",
            &random_string_message,
            &random_string_payload,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock
        )));

        assert!(check_message_with_resetting_clock(&format!(
            "{}() from {} to {} within [{};{}] using {} and resetting ({});",
            &random_string_message,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock,
            &random_string_reset
        )));

        assert!(check_message_with_resetting_clock(&format!(
            "{}() from {} to {} within ]{};{}] using {} and resetting ({});",
            &random_string_message,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock,
            &random_string_reset
        )));

        assert!(check_message_with_resetting_clock(&format!(
            "{}() from {} to {} within [{};{}[ using {} and resetting ({});",
            &random_string_message,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock,
            &random_string_reset
        )));

        assert!(check_message_with_resetting_clock(&format!(
            "{}() from {} to {} within ]{};{}[ using {} and resetting ({});",
            &random_string_message,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock,
            &random_string_reset
        )));

        assert!(!check_message_with_resetting_clock(&format!(
            "{}({}) from {} to {} within [{};{}] using {} and resetting ({});",
            &random_string_message,
            &random_string_payload,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock,
            &random_string_reset
        )));
    }

    #[test]
    fn test_check_message_with_payload_and_resetting_clock() {
        let random_string_message = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_payload = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_sender = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_receiver = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_left = rand::thread_rng().gen_range(1..=100);
        let random_string_right = rand::thread_rng().gen_range(1..=100);
        let random_string_clock = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        let random_string_reset = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();

        assert!(!check_message_with_payload_and_resetting_clock(&format!(
            "{}() from {} to {} within [{};{}] using {} and resetting ();",
            &random_string_message,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock
        )));

        assert!(!check_message_with_payload_and_resetting_clock(&format!(
            "{}({}) from {} to {} within [{};{}] using {} and resetting ();",
            &random_string_message,
            &random_string_payload,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock
        )));

        assert!(!check_message_with_payload_and_resetting_clock(&format!(
            "{}() from {} to {} within [{};{}] using {} and resetting ({});",
            &random_string_message,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock,
            &random_string_reset
        )));

        assert!(check_message_with_payload_and_resetting_clock(&format!(
            "{}({}) from {} to {} within [{};{}] using {} and resetting ({});",
            &random_string_message,
            &random_string_payload,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock,
            &random_string_reset
        )));

        assert!(check_message_with_payload_and_resetting_clock(&format!(
            "{}({}) from {} to {} within ]{};{}] using {} and resetting ({});",
            &random_string_message,
            &random_string_payload,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock,
            &random_string_reset
        )));

        assert!(check_message_with_payload_and_resetting_clock(&format!(
            "{}({}) from {} to {} within [{};{}[ using {} and resetting ({});",
            &random_string_message,
            &random_string_payload,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock,
            &random_string_reset
        )));

        assert!(check_message_with_payload_and_resetting_clock(&format!(
            "{}({}) from {} to {} within ]{};{}[ using {} and resetting ({});",
            &random_string_message,
            &random_string_payload,
            &random_string_sender,
            &random_string_receiver,
            &random_string_left,
            &random_string_right,
            &random_string_clock,
            &random_string_reset
        )));
    }

    #[test]
    fn test_check_rec() {
        let random_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();

        assert!(check_rec(&format!("rec {}", &random_string)));
        assert!(check_rec(&format!("rec {} {{", &random_string)));
        assert!(!check_rec(&format!("rece {}", &random_string)));
        assert!(check_rec(&format!("rec {} }}", &random_string)));
    }

    #[test]
    fn test_check_continue() {
        let random_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();

        assert!(check_continue(&format!("continue {}", &random_string)));
        assert!(!check_continue(&format!("conti nue {}", &random_string)));
    }
}
