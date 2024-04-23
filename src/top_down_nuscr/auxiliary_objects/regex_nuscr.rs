use {once_cell::sync::Lazy, regex::Regex};

pub(crate) static GLOBAL_PROTOCOL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"timed( +)global( +)protocol( +)(?P<name>\w+)\(((role( +)\w+( *),?( *))+)\)( *)\{?")
        .unwrap()
});

pub(crate) static ROLE: Lazy<Regex> = Lazy::new(|| Regex::new(r"role (?P<name>\w+)").unwrap());

pub(crate) static CHOICE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"choice( +)at( +)(?P<choice>\w+)( *)\{?").unwrap());

pub(crate) static OR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\}( *)or( *)\{").unwrap());

pub(crate) static MESSAGE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<message>\w+)\(\w*\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)(?P<left_bracket>(\[|\]))( *)(?P<left_bound>\d+)( *);( *)(?P<right_bound>\d+)( *)(?P<right_bracket>(\[|\]))( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\(\w*\)( *);").unwrap()
});

pub(crate) static MESSAGE_WITH_PAYLOAD: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<message>\w+)\((?P<payload>\w+)\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)(?P<left_bracket>(\[|\]))( *)(?P<left_bound>\d+)( *);( *)(?P<right_bound>\d+)( *)(?P<right_bracket>(\[|\]))( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\(\)( *);").unwrap()
});

pub(crate) static MESSAGE_WITH_RESET: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<message>\w+)\(\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)(?P<left_bracket>(\[|\]))( *)(?P<left_bound>\d+)( *);( *)(?P<right_bound>\d+)( *)(?P<right_bracket>(\[|\]))( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\((?P<reset>\w+)\)( *);").unwrap()
});

pub(crate) static MESSAGE_WITH_PAYLOAD_AND_RESET: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<message>\w+)\((?P<payload>\w+)\)( +)from( +)(?P<sender>\w+)( +)to( +)(?P<receiver>\w+)( +)within( +)(?P<left_bracket>(\[|\]))( *)(?P<left_bound>\d+)( *);( *)(?P<right_bound>\d+)( *)(?P<right_bracket>(\[|\]))( +)using( +)(?P<clock>\w+)( +)and( +)resetting( +)\((?P<reset>\w+)\)( *);").unwrap()
});

pub(crate) static REC: Lazy<Regex> = Lazy::new(|| Regex::new(r"rec( +)(?P<loop>\w+)").unwrap());

pub(crate) static CONTINUE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"continue( +)(?P<loop>\w+)( *);").unwrap());

// Check whether the input is the first line of any new nuscrT protocol
pub(crate) fn check_global(input: &str) -> bool {
    GLOBAL_PROTOCOL.is_match(input)
}

// Check whether this is a choice
pub(crate) fn check_choice(input: &str) -> bool {
    CHOICE.is_match(input)
}

// Check whether this is a branching
pub(crate) fn check_or(input: &str) -> bool {
    OR.is_match(input)
}

// Check whether this is a shared message
pub(crate) fn check_message(input: &str) -> bool {
    MESSAGE.is_match(input)
}

// Check whether this is a shared message with payload
pub(crate) fn check_message_with_payload(input: &str) -> bool {
    MESSAGE_WITH_PAYLOAD.is_match(input)
}

// Check whether this is a shared message with resetting clock
pub(crate) fn check_message_with_resetting_clock(input: &str) -> bool {
    MESSAGE_WITH_RESET.is_match(input)
}

// Check whether this is a shared message with payload and resetting clock
pub(crate) fn check_message_with_payload_and_resetting_clock(input: &str) -> bool {
    MESSAGE_WITH_PAYLOAD_AND_RESET.is_match(input)
}

// Check whether this is the start of a recursive loop
pub(crate) fn check_rec(input: &str) -> bool {
    REC.is_match(input)
}

// Check whether this is the continue of a recursive loop
pub(crate) fn check_continue(input: &str) -> bool {
    CONTINUE.is_match(input)
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
        assert!(!check_or(&format!("   }} or {}  {{    ", &random_string)));
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

        assert!(check_continue(&format!("continue {};", &random_string)));
        assert!(!check_continue(&format!("conti nue {}", &random_string)));
    }
}
