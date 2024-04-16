// #![allow(dead_code, unused_variables, clippy::needless_borrows_for_generic_args)]

//! TODO

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use {once_cell::sync::Lazy, regex::Regex};

/// Check whether the input is the first line of any new nuscrT protocol
fn check_global(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"timed( +)global( +)protocol( +)(?P<name>\w+)\(((role \w+,? *)+)\)( *)\{")
            .unwrap()
    });
    RE.is_match(input)
}

/// Check each role
fn check_role(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(role ?P<name>\w+,?)").unwrap());
    RE.is_match(input)
}

/// Check whether this is a choice
fn check_choice(input: &str) -> bool {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"( *)choice( +)at( +)(?P<choice>\w+)( *)\{?").unwrap());
    RE.is_match(input)
}

/// Check whether this is a branching
fn check_or(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"( *)\}( *)or( *)\{?").unwrap());
    RE.is_match(input)
}

/// Check whether this is a shared message
fn check_message(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"( *)(?P<message>\w+)\(\w*\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)\[(?P<left>\d+);(?P<right>\d+)\]( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\(\w*\)( *);").unwrap()
    });
    RE.is_match(input)
}

/// Check whether this is a shared message with payload
fn check_message_with_payload(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"( *)(?P<message>\w+)\((?P<payload>\w+)\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)\[(?P<left>\d+);(?P<right>\d+)\]( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\(\)( *);").unwrap()
    });
    RE.is_match(input)
}

/// Check whether this is a shared message with resetting clock
fn check_message_with_resetting_clock(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"( *)(?P<message>\w+)\(\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)\[(?P<left>\d+);(?P<right>\d+)\]( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\((?P<reset>\w+)\)( *);").unwrap()
    });
    RE.is_match(input)
}

/// Check whether this is a shared message with payload and resetting clock
fn check_message_with_payload_and_resetting_clock(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"( *)(?P<message>\w+)\((?P<payload>\w+)\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)\[(?P<left>\d+);(?P<right>\d+)\]( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\((?P<reset>\w+)\)( *);").unwrap()
    });
    RE.is_match(input)
}

/// Check whether this is the start of a recursive loop
fn check_rec(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"( *)rec( +)(?P<loop>\w+)( *)\{?").unwrap());
    RE.is_match(input)
}

/// Check whether this is the continue of a recursive loop
fn check_continue(input: &str) -> bool {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"( *)continue( +)(?P<loop>\w+)( *)").unwrap());
    RE.is_match(input)
}

/// Check whether there are opening closures
fn check_opening_closures(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(( *)\{( *)+)").unwrap());
    RE.is_match(input)
}

/// Check whether there are closing closures
fn check_closing_closures(input: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(( *)\}( *)+)").unwrap());
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

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_add() {
//         assert_eq!(add(1, 2), 3);
//     }

//     #[test]
//     fn test_bad_add() {
//         // This assert would fire and test will fail.
//         // Please note, that private functions can be tested too!
//         assert_eq!(bad_add(1, 2), 3);
//     }
// }
