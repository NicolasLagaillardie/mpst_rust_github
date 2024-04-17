// #![allow(dead_code, unused_variables, clippy::needless_borrows_for_generic_args)]

//! TODO

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use {once_cell::sync::Lazy, regex::Regex};

/// Check whether the input is the first line of any new nuscrT protocol
fn check_global(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"timed( +)global( +)protocol( +)(?P<name>\w+)\(((role( +)\w+( *),?( *))+)\)( *)\{",
        )
        .unwrap()
    });
    RE.is_match(input)
}

/// Check each role
fn check_role(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"role (?P<name>\w+)").unwrap());
    RE.is_match(input)
}

/// Check whether this is a choice
fn check_choice(input: &str) -> bool {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"choice( +)at( +)(?P<choice>\w+)( *)\{?").unwrap());
    RE.is_match(input)
}

/// Check whether this is a branching
fn check_or(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\}( *)or( *)\{?").unwrap());
    RE.is_match(input)
}

/// Check whether this is a shared message
fn check_message(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?P<message>\w+)\(\w*\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)(\[|\])( *)(?P<left>\d+)( *);( *)(?P<right>\d+)( *)(\[|\])( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\(\w*\)( *);").unwrap()
    });
    RE.is_match(input)
}

/// Check whether this is a shared message with payload
fn check_message_with_payload(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?P<message>\w+)\((?P<payload>\w+)\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)(\[|\])( *)(?P<left>\d+)( *);( *)(?P<right>\d+)( *)(\[|\])( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\(\)( *);").unwrap()
    });
    RE.is_match(input)
}

/// Check whether this is a shared message with resetting clock
fn check_message_with_resetting_clock(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?P<message>\w+)\(\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)(\[|\])( *)(?P<left>\d+)( *);( *)(?P<right>\d+)( *)(\[|\])( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\((?P<reset>\w+)\)( *);").unwrap()
    });
    RE.is_match(input)
}

/// Check whether this is a shared message with payload and resetting clock
fn check_message_with_payload_and_resetting_clock(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?P<message>\w+)\((?P<payload>\w+)\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)(\[|\])( *)(?P<left>\d+)( *);( *)(?P<right>\d+)( *)(\[|\])( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\((?P<reset>\w+)\)( *);").unwrap()
    });
    RE.is_match(input)
}

/// Check whether this is the start of a recursive loop
fn check_rec(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"rec( +)(?P<loop>\w+)").unwrap());
    RE.is_match(input)
}

/// Check whether this is the continue of a recursive loop
fn check_continue(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"continue( +)(?P<loop>\w+)( *)").unwrap());
    RE.is_match(input)
}

/// Check whether there are opening closures
fn check_opening_closures(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{").unwrap());
    RE.is_match(input)
}

/// Check whether there are closing closures
fn check_closing_closures(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\}").unwrap());
    RE.is_match(input)
}

/// Generate endpoints from a nuscr file
/// with timed global protocol
pub fn generator(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let temp_line = line?;
        println!(
            "global: {} / role: {} / choice: {} / or: {} / message: {} / message payload: {} / message reset: {} / message payload and reset: {} / rec: {} / continue: {} / opening closure: {} / closing closure: {}",
            check_global(&temp_line),
            check_role(&temp_line),
            check_choice(&temp_line),
            check_or(&temp_line),
            check_message(&temp_line),
            check_message_with_payload(&temp_line),
            check_message_with_resetting_clock(&temp_line),
            check_message_with_payload_and_resetting_clock(&temp_line),
            check_rec(&temp_line),
            check_continue(&temp_line),
            check_opening_closures(&temp_line),
            check_closing_closures(&temp_line),
        );
        println!("{}", &temp_line);
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

        assert!(check_or(&format!("}}or{{")));
        assert!(check_or(&format!("}} or{{")));
        assert!(check_or(&format!("}}or {{")));
        assert!(check_or(&format!("}}   or   {{")));
        assert!(check_or(&format!("   }} or   {{    ")));
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

    #[test]
    fn test_check_opening_closures() {
        assert!(check_opening_closures("{"));
        assert!(check_opening_closures("{{"));
        assert!(!check_opening_closures("}"));
        assert!(check_opening_closures("{{}{}"));
    }

    #[test]
    fn test_check_closing_closures() {
        assert!(check_closing_closures("}"));
        assert!(check_closing_closures("}}"));
        assert!(check_closing_closures("}{{}}"));
        assert!(!check_closing_closures("{"));
    }
}
