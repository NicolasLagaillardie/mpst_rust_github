use std::any::type_name;
use std::collections::HashMap;
use std::error::Error;

#[doc(hidden)]
pub fn checker_aux(
    session1: &str,
    session2: &str,
    stack: &str,
    hm: &HashMap<String, &Vec<String>>,
    seen: &mut Vec<String>,
) -> Result<String, Box<dyn Error>> {
    let head_stack = get_head(&stack);
    let head_session1 = get_head(&session1);
    let head_session2 = get_head(&session2);

    if !head_stack.contains("RoleEnd") {
        let (sender, receiver) = get_name(&head_stack);

        let result = match sender.as_str() {
            "A" => match_headers(
                ["B", "C"],
                [head_session1.as_str(), head_session2.as_str()],
                [session1, session2, stack],
                [sender, receiver],
                [0, 0, 3, 3],
                hm,
                seen,
            )?,
            "B" => match_headers(
                ["A", "C"],
                [head_session1.as_str(), head_session2.as_str()],
                [session1, session2, stack],
                [sender, receiver],
                [0, 1, 3, 4],
                hm,
                seen,
            )?,
            "C" => match_headers(
                ["A", "B"],
                [head_session1.as_str(), head_session2.as_str()],
                [session1, session2, stack],
                [sender, receiver],
                [1, 1, 4, 4],
                hm,
                seen,
            )?,
            _ => panic!("Wrong sender, not recognized: {}", sender),
        };
        Ok(result)
    } else {
        Ok(String::from("0"))
    }
}

#[doc(hidden)]
fn match_headers(
    headers: [&str; 2],
    head_sessions: [&str; 2],
    sessionmpst: [&str; 3], // session1, session2, stack
    involved: [String; 2],  // sender, receiver
    index: [usize; 4],
    hm: &HashMap<String, &Vec<String>>,
    seen: &mut Vec<String>,
) -> Result<String, Box<dyn Error>> {
    match involved[1].as_str() {
        h if h == headers[0] => match_full_types(
            head_sessions[0],
            [
                &get_tail(&sessionmpst[0]),
                sessionmpst[1],
                &get_tail(&sessionmpst[2]),
            ],
            involved,
            &get_head_payload(&sessionmpst[0]),
            hm,
            seen,
        ),
        h if h == headers[1] => match_full_types(
            head_sessions[1],
            [
                sessionmpst[0],
                &get_tail(&sessionmpst[1]),
                &get_tail(&sessionmpst[2]),
            ],
            involved,
            &get_head_payload(&sessionmpst[1]),
            hm,
            seen,
        ),
        h if h == "All" => all_type(sessionmpst, index, hm, seen),
        _ => panic!("Wrong receiver, not recognized: {}", involved[1]),
    }
}

#[doc(hidden)]
fn match_full_types(
    head_session: &str,
    sessionmpst: [&str; 3], // session1, session2, stack
    involved: [String; 2],  // sender, receiver
    payload: &str,
    hm: &HashMap<String, &Vec<String>>,
    seen: &mut Vec<String>,
) -> Result<String, Box<dyn Error>> {
    match head_session {
        "Send" => send_type(sessionmpst, involved, payload, hm, seen, " + "),
        "Recv" => recv_type(sessionmpst, involved, payload, hm, seen, " & "),
        _ => panic!("Wrong session type, not recognized: {}", head_session),
    }
}

#[doc(hidden)]
pub fn parse_type_of<T>(_: T) -> String {
    parse_type(type_name::<T>())
}

#[doc(hidden)]
fn parse_type(s: &str) -> String {
    let mut result = s;
    if s.contains(":&") {
        result = &result.split(":&").collect::<Vec<_>>()[1];
    }
    let result = &result
        .replace("&", "")
        .replace("mpstthree::", "")
        .replace("sessionmpst::", "")
        .replace("binary::", "")
        .replace("role::a_to_b::", "")
        .replace("role::a_to_c::", "")
        .replace("role::b_to_a::", "")
        .replace("role::b_to_c::", "")
        .replace("role::c_to_a::", "")
        .replace("role::c_to_b::", "")
        .replace("role::a_to_all::", "")
        .replace("role::b_to_all::", "")
        .replace("role::c_to_all::", "")
        .replace("role::all_to_a::", "")
        .replace("role::all_to_b::", "")
        .replace("role::all_to_c::", "")
        .replace("role::end::", "")
        .replace("either::", "")
        .replace("checker::", "");
    result.chars().filter(|c| !c.is_whitespace()).collect()
}

#[doc(hidden)]
fn get_name(head: &str) -> (String, String) {
    let (sender, receiver) = match head {
        "RoleAtoAll" => (String::from("A"), String::from("All")),
        "RoleBtoAll" => (String::from("B"), String::from("All")),
        "RoleCtoAll" => (String::from("C"), String::from("All")),
        "RoleAtoB" => (String::from("A"), String::from("B")),
        "RoleAtoC" => (String::from("A"), String::from("C")),
        "RoleBtoA" => (String::from("B"), String::from("A")),
        "RoleBtoC" => (String::from("B"), String::from("C")),
        "RoleCtoA" => (String::from("C"), String::from("A")),
        "RoleCtoB" => (String::from("C"), String::from("B")),
        "RoleEnd" => (String::from("End"), String::from("End")),
        _ => panic!("Wrong head, not recognized: {}", head),
    };

    (sender, receiver)
}

#[doc(hidden)]
fn get_head(s: &str) -> String {
    let mut result: Vec<&str> = s.split('<').collect();
    if result[0] == "Either" {
        return String::from(s);
    }
    result = result
        .iter()
        .map(|&x| {
            if x.contains(',') {
                let temp: Vec<&str> = x.split(',').collect();
                temp[1]
            } else {
                x
            }
        })
        .collect::<Vec<_>>();
    String::from(result[0])
}

#[doc(hidden)]
fn get_head_payload(s: &str) -> String {
    let payload = &get_two_tails(s)[0];
    if payload.contains("::") {
        String::from(payload.split("::").collect::<Vec<_>>()[1])
    } else {
        String::from(payload)
    }
}

#[doc(hidden)]
fn get_two_tails(s: &str) -> [String; 2] {
    let mut result: [String; 2] = Default::default();
    if s == "End" {
        result[1].push_str("End");
        return result;
    }
    let mut index = -1;
    let mut index_session = 0;
    for c in s.chars() {
        match c {
            ',' => {
                if index <= 0 && result[index_session] != "" {
                    index_session += 1;
                } else if index >= 0 {
                    result[index_session].push(c);
                }
            }
            '<' => {
                if index >= 0 {
                    result[index_session].push(c);
                }
                index += 1;
            }
            '>' => {
                if index >= 0 {
                    result[index_session].push(c);
                }
                index -= 1;
            }
            _ => {
                if index >= 0 {
                    result[index_session].push(c);
                }
            }
        }
    }
    result
}

#[doc(hidden)]
fn get_sessions_and_stack(s: &str) -> [String; 3] {
    let mut result: [String; 3] = Default::default();
    let mut index = -1;
    let mut index_session = 0;
    let new_s = &s.replace("SessionMpst", "");
    for c in new_s.chars() {
        match c {
            ',' => {
                if index <= 0 && result[index_session] != "" {
                    index_session += 1;
                } else {
                    result[index_session].push(c);
                }
            }
            '<' => {
                if index >= 0 {
                    result[index_session].push(c);
                }
                index += 1;
            }
            '>' => {
                if index >= 0 {
                    result[index_session].push(c);
                }
                index -= 1;
            }
            _ => {
                result[index_session].push(c);
            }
        }
    }
    result
}

#[doc(hidden)]
fn divide_either(s: &str) -> [String; 6] {
    let mut result: [String; 6] = Default::default();
    let mut index = -2;
    let mut index_session = 0;
    let new_s = s.replacen("Either", "", 1);
    let new_s = &new_s.replace("SessionMpst", "");
    for c in new_s.chars() {
        match c {
            ',' => {
                if index <= 0 && result[index_session] != "" {
                    index_session += 1;
                } else {
                    result[index_session].push(c);
                }
            }
            '<' => {
                if index >= 0 {
                    result[index_session].push(c);
                }
                index += 1;
            }
            '>' => {
                if index >= 0 {
                    result[index_session].push(c);
                }
                index -= 1;
            }
            _ => {
                result[index_session].push(c);
            }
        }
    }
    result
}

#[doc(hidden)]
fn get_tail(s: &str) -> String {
    let result: Vec<&str> = s.split('<').collect();
    result[1..].join("<")
}

#[doc(hidden)]
fn get_dual(s: &str) -> String {
    // Switch Send / Recv
    let result = &s.replace("Send<", "Revc<");
    let result = &result.replace("Recv<", "Send<");
    let result = &result.replace("Revc<", "Recv<");
    let result = switch_role(&result, "RoleAtoB", "RoleBtoA");
    let result = switch_role(&result, "RoleAtoC", "RoleCtoA");
    let result = switch_role(&result, "RoleCtoB", "RoleBtoC");
    let result = switch_role(&result, "RoleAtoAll", "RoleAlltoA");
    let result = switch_role(&result, "RoleBtoAll", "RoleAlltoB");
    switch_role(&result, "RoleCtoAll", "RoleAlltoC")
}

#[doc(hidden)]
fn switch_role(s: &str, a: &str, b: &str) -> String {
    let result = &s.replace(&format!("Role{}to{}", a, b), &format!("Role{}to{}", b, "X"));
    let result = &result.replace(&format!("Role{}to{}", b, a), &format!("Role{}to{}", a, b));
    let result = &result.replace(&format!("Role{}to{}", b, "X"), &format!("Role{}to{}", b, a));
    String::from(result)
}

#[doc(hidden)]
fn send_type(
    sessionmpst: [&str; 3], // session1, session2, stack
    involved: [String; 2],  // sender, receiver
    payload: &str,
    hm: &HashMap<String, &Vec<String>>,
    seen: &mut Vec<String>,
    symbol: &str,
) -> Result<String, Box<dyn Error>> {
    if seen.contains(&String::from(payload)) {
        Ok(String::from("X"))
    } else if hm.contains_key(payload) {
        recurs_type(payload, hm, seen, symbol)
    } else {
        Ok(format!(
            "{}!{}.{}",
            involved[0],
            involved[1],
            checker_aux(sessionmpst[0], sessionmpst[1], sessionmpst[2], &hm, seen)?
        ))
    }
}

#[doc(hidden)]
fn recv_type(
    sessionmpst: [&str; 3], // session1, session2, stack
    involved: [String; 2],  // sender, receiver
    payload: &str,
    hm: &HashMap<String, &Vec<String>>,
    seen: &mut Vec<String>,
    symbol: &str,
) -> Result<String, Box<dyn Error>> {
    if payload.contains("Either") {
        let branching: [String; 6] = divide_either(payload);
        Ok(format!(
            "( {} & {} )",
            checker_aux(&branching[0], &branching[1], &branching[2], &hm, seen)?,
            checker_aux(&branching[3], &branching[4], &branching[5], &hm, seen)?,
        ))
    } else if seen.contains(&String::from(payload)) {
        Ok(String::from("X"))
    } else if hm.contains_key(payload) {
        recurs_type(payload, hm, seen, symbol)
    } else {
        Ok(format!(
            "{}?{}.{}",
            involved[0],
            involved[1],
            checker_aux(sessionmpst[0], sessionmpst[1], sessionmpst[2], &hm, seen)?
        ))
    }
}

#[doc(hidden)]
fn recurs_type(
    payload: &str,
    hm: &HashMap<String, &Vec<String>>,
    seen: &mut Vec<String>,
    symbol: &str,
) -> Result<String, Box<dyn Error>> {
    let mut vec_result = Vec::new();
    let mut recurs = false;

    seen.push(String::from(payload));

    match hm.get(payload) {
        Some(&branches) => {
            for branch in branches.iter() {
                let sessions_and_stack = get_sessions_and_stack(&parse_type(branch));
                let result_branch = checker_aux(
                    &sessions_and_stack[0],
                    &sessions_and_stack[1],
                    &sessions_and_stack[2],
                    &hm,
                    seen,
                )?;
                recurs = result_branch.contains(&String::from(".X")) || recurs;
                vec_result.push(result_branch);
            }
        }
        _ => panic!("Error with hashmap and payload: {:?} and {}", hm, payload),
    }

    let result = vec_result.join(symbol);
    if recurs {
        Ok(format!("µX( {} )", result))
    } else {
        Ok(format!("( {} )", result))
    }
}

#[doc(hidden)]
fn all_type(
    sessionmpst: [&str; 3], // session1, session2, stack
    index: [usize; 4],      // index for branches
    hm: &HashMap<String, &Vec<String>>,
    seen: &mut Vec<String>,
) -> Result<String, Box<dyn Error>> {
    let payload_1 = get_head_payload(&sessionmpst[0]);
    let payload_2 = get_head_payload(&sessionmpst[1]);
    if payload_1.contains("Either") && payload_2.contains("Either") {
        let branching_1: [String; 6] = divide_either(&payload_1);
        let branching_2: [String; 6] = divide_either(&payload_2);
        let tails: [String; 2] = get_two_tails(&sessionmpst[2]);

        Ok(format!(
            "( {} + {} )",
            checker_aux(
                &get_dual(&branching_1[index[0]]),
                &get_dual(&branching_2[index[1]]),
                &tails[0],
                &hm,
                seen
            )?,
            checker_aux(
                &get_dual(&branching_1[index[2]]),
                &get_dual(&branching_2[index[3]]),
                &tails[1],
                &hm,
                seen
            )?,
        ))
    } else if payload_1.contains("Either") {
        let branching_1: [String; 6] = divide_either(&payload_1);
        let tails: [String; 2] = get_two_tails(&sessionmpst[2]);

        Ok(format!(
            "( {} + {} )",
            checker_aux(
                &get_dual(&branching_1[index[0]]),
                &get_dual(&sessionmpst[1]),
                &tails[0],
                &hm,
                seen
            )?,
            checker_aux(
                &get_dual(&branching_1[index[2]]),
                &get_dual(&sessionmpst[1]),
                &tails[1],
                &hm,
                seen
            )?,
        ))
    } else if payload_2.contains("Either") {
        let branching_2: [String; 6] = divide_either(&payload_2);
        let tails: [String; 2] = get_two_tails(&sessionmpst[2]);

        Ok(format!(
            "( {} + {} )",
            checker_aux(
                &get_dual(&sessionmpst[0]),
                &get_dual(&branching_2[index[1]]),
                &tails[0],
                &hm,
                seen
            )?,
            checker_aux(
                &get_dual(&sessionmpst[0]),
                &get_dual(&branching_2[index[3]]),
                &tails[1],
                &hm,
                seen
            )?,
        ))
    } else {
        panic!(
            "Wrong payloads, not recognized: {:?} , {:?} and {:?} ( {:?} / {:?} ) for {:?} , {:?} and {:?}",
            divide_either(&payload_1),
            divide_either(&payload_2),
            get_two_tails(&sessionmpst[2]),
            &payload_1,
            &payload_2,
            &sessionmpst[0],
            &sessionmpst[1],
            &sessionmpst[2]
        );
    }
}
