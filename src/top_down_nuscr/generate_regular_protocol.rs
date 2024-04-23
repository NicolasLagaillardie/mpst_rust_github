use super::auxiliary_objects::regex_nuscr::*;

use rand::{distributions::Alphanumeric, Rng};

use std::fs::{remove_file, File};
use std::io::{BufRead, BufReader, Error, Lines, Write};
use std::iter::Map;
use std::process::Command;

fn generate_regular_protocol(
    lines_iter: &mut Map<Lines<BufReader<File>>, impl FnMut(Result<String, Error>) -> String>,
    file: &mut File,
    mut name_protocol: String,
) -> Result<String, Box<dyn std::error::Error>> {
    match lines_iter.next() {
        None => Ok(name_protocol),
        Some(line) => {
            if check_global(&line) {
                let captured_fields = GLOBAL_PROTOCOL.captures(&line).unwrap();

                name_protocol = captured_fields["name"].to_string();

                writeln!(file, "{}", line.replace("timed ", ""))?;
            } else if check_message_with_payload(&line) {
                let starting_spaces = line
                    .chars()
                    .take_while(|ch| ch.is_whitespace() && *ch != '\n')
                    .map(|_| " ")
                    .collect::<String>();

                let captured_fields = MESSAGE_WITH_PAYLOAD.captures(&line).unwrap();

                let message = &captured_fields["message"];
                let payload = &captured_fields["payload"];
                let sender = &captured_fields["sender"];
                let receiver = &captured_fields["receiver"];

                writeln!(
                    file,
                    "{}{}({}) from {} to {};",
                    starting_spaces, message, payload, sender, receiver
                )?;
            } else if check_message(&line) {
                let starting_spaces = line
                    .chars()
                    .take_while(|ch| ch.is_whitespace() && *ch != '\n')
                    .map(|_| " ")
                    .collect::<String>();

                let captured_fields = MESSAGE.captures(&line).unwrap();

                let message = &captured_fields["message"];
                let sender = &captured_fields["sender"];
                let receiver = &captured_fields["receiver"];

                writeln!(
                    file,
                    "{}{}() from {} to {};",
                    starting_spaces, message, sender, receiver
                )?;
            } else {
                writeln!(file, "{}", line)?;
            }

            generate_regular_protocol(lines_iter, file, name_protocol)
        }
    }
}

pub(crate) fn check_fsm(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Iterator over the lines of the input file
    // Ccheck that the protocol is sound, nuscr-wise
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut lines_iter = reader.lines().into_iter().map(|line| line.unwrap());

    let name_file = format!(
        "{}.nuscr",
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect::<String>()
    );

    let mut output_file = File::create(&name_file)?;

    let name_protocol =
        generate_regular_protocol(&mut lines_iter, &mut output_file, "".to_string())?;

    let output = if cfg!(target_os = "windows") {
        return Err("OS not supported".into());
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!(
                "nuscr --generate-sexp={} {}",
                name_protocol, name_file
            ))
            .output()?
    };

    remove_file(&name_file)?;

    if output.stdout.is_empty() {
        return Err("There was an issue with the nuscr protocol.".into());
    }

    Ok(())
}
