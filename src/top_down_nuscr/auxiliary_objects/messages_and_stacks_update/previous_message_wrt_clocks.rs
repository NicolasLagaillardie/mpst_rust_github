use super::MessageParameters;
use std::collections::HashMap;

pub(crate) fn fn_previous_message_wrt_clocks(
    previous_message_wrt_clocks: &mut HashMap<String, HashMap<String, (String, String)>>,
    elts: &MessageParameters,
    role: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if elts.left_bound > elts.right_bound {
        return Err("Wrong time interval: a left bound is after its related right bound.".into());
    }

    if let Some(role_time_bounds) = previous_message_wrt_clocks.get_mut(role) {
        if let Some(previous_time_bound) = role_time_bounds.get_mut(&elts.clock) {
            if ((elts.left_bound.parse::<i32>().unwrap() + (elts.left_bracket == "[") as i32)
                < (previous_time_bound.0.parse::<i32>().unwrap()
                    + previous_time_bound.1.parse::<bool>().unwrap() as i32))
                && ((elts.right_bound.parse::<i32>().unwrap() + (elts.right_bracket == "]") as i32)
                    < (previous_time_bound.0.parse::<i32>().unwrap()
                        + previous_time_bound.1.parse::<bool>().unwrap() as i32))
            {
                return Err("Two of the consecutive time bounds are not feasible.".into());
            } else {
                *previous_time_bound = (
                    elts.right_bound.to_string(),
                    (elts.right_bracket == "]").to_string(),
                );
            }
        } else {
            role_time_bounds.insert(
                elts.clock.to_string(),
                (
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
                elts.right_bound.to_string(),
                (elts.right_bracket == "]").to_string(),
            ),
        );

        previous_message_wrt_clocks.insert(elts.receiver.to_string(), temp_time_bound);
    }

    Ok(())
}
