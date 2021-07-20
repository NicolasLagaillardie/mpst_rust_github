pub mod end;
pub mod recv;
pub mod send;
pub mod session;
use std::error::Error;

/// Separate the different _fields_ of a stringified type.
#[doc(hidden)]
fn get_blocks(full_block: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result = Vec::new();
    let mut temp = String::from("");
    let mut index = -1;

    for i in full_block.chars() {
        if i == '&' || i.is_whitespace() {
        } else if i == '>' && index == 0 {
            result.push(temp.to_string());
            temp = String::from("");
        } else if i == '<' && index >= 0 {
            temp = format!("{}{}", temp, i);
            index += 1;
        } else if i == '>' && index >= 0 {
            temp = format!("{}{}", temp, i);
            index -= 1;
        } else if i == ',' && index == 0 {
            result.push(temp);
            temp = String::from("");
        } else if index >= 0 {
            temp = format!("{}{}", temp, i);
        } else if i == '<' {
            index += 1;
        } else if i == '>' {
            index -= 1;
        }
    }

    if !temp.is_empty() {
        let mut chars = temp.chars();
        chars.next_back();

        result.push(chars.as_str().to_string());
    }

    Ok(result)
}
