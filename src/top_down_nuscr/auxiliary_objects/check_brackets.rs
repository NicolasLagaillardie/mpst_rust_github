//! Check if there are as many opening brackets
//! than closing ones
pub(crate) fn check_brackets(
    opening_brackets: &mut usize,
    closing_brackets: &mut usize,
    line: &str,
    line_number: &usize,
) -> Result<(), Box<dyn std::error::Error>> {
    *opening_brackets += line.matches('{').count();
    *closing_brackets += line.matches('}').count();

    if opening_brackets < closing_brackets {
        Err(format!(
            "There are too many closing brackets. See line: {}",
            line_number
        )
        .into())
    } else {
        Ok(())
    }
}
