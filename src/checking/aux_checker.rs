use std::any::type_name;
use std::collections::HashMap;
use std::error::Error;

#[doc(hidden)]
pub fn checker_aux<S: ::std::hash::BuildHasher>(
    sessionmpst: [&str; 4],
    role: &str,
    hm: &HashMap<String, &Vec<String>, S>,
    seen: &mut Vec<String>,
) -> Result<String, Box<dyn Error>> {
    let head_session1 = get_head(&sessionmpst[0]);
    let head_session2 = get_head(&sessionmpst[1]);
    let head_stack = get_head(&sessionmpst[2]);
    let sender = get_name(&get_head(&sessionmpst[3]));

    // println!("sender checker_aux: {:?}", &sender);
    // println!("sessionmpst checker_aux: {:?}", &sessionmpst);
    // println!("head_stack checker_aux: {:?}", &head_stack);

    if !head_stack.contains("RoleEnd") {
        let receiver = get_name(&head_stack);

        let result = match sender.as_str() {
            "A" => match_headers(
                ["B", head_session1.as_str(), "C", head_session2.as_str()],
                sessionmpst,
                [sender, receiver],
                [0, 0, 4, 4],
                role,
                hm,
                seen,
            )?,
            "B" => match_headers(
                ["A", head_session1.as_str(), "C", head_session2.as_str()],
                sessionmpst,
                [sender, receiver],
                [0, 1, 4, 5],
                role,
                hm,
                seen,
            )?,
            "C" => match_headers(
                ["A", head_session1.as_str(), "B", head_session2.as_str()],
                sessionmpst,
                [sender, receiver],
                [1, 1, 5, 5],
                role,
                hm,
                seen,
            )?,
            "All" => match receiver.as_str() {
                "A" => match_recv_from_all("A", ["B", "C"], sessionmpst, role, hm, seen)?,
                "B" => match_recv_from_all("B", ["A", "C"], sessionmpst, role, hm, seen)?,
                "C" => match_recv_from_all("C", ["A", "B"], sessionmpst, role, hm, seen)?,
                _ => panic!("Wrong receiver on All, not recognized: {}", receiver),
            },
            _ => panic!("Wrong sender, not recognized: {}", sender),
        };
        Ok(result)
    } else {
        Ok(String::from("0"))
    }
}

#[doc(hidden)]
fn match_recv_from_all<S: ::std::hash::BuildHasher>(
    sender: &str,
    receivers: [&str; 2],
    sessionmpst: [&str; 4],
    role: &str,
    hm: &HashMap<String, &Vec<String>, S>,
    seen: &mut Vec<String>,
) -> Result<String, Box<dyn Error>> {
    // println!("sessionmpst match_recv_from_all: {:?}", &sessionmpst);

    match role {
        receiver if receiver == receivers[0] => checker_aux(
            [
                sessionmpst[0],
                sessionmpst[1],
                &sessionmpst[2].replacen(
                    &format!("RoleAllto{}", sender),
                    &format!("Role{}", receiver),
                    1,
                ),
                sessionmpst[3],
            ],
            role,
            hm,
            seen,
        ),
        receiver if receiver == receivers[1] => checker_aux(
            [
                sessionmpst[0],
                sessionmpst[1],
                &sessionmpst[2].replacen(
                    &format!("RoleAllto{}", sender),
                    &format!("Role{}", receiver),
                    1,
                ),
                sessionmpst[3],
            ],
            role,
            hm,
            seen,
        ),
        _ => panic!("Wrong role, not recognized: {}", role),
    }
}

#[doc(hidden)]
fn match_headers<S: ::std::hash::BuildHasher>(
    roles_and_sessions: [&str; 4], // role 1, session 1, role 2, session 2
    sessionmpst: [&str; 4],
    involved: [String; 2],
    index: [usize; 4],
    role: &str,
    hm: &HashMap<String, &Vec<String>, S>,
    seen: &mut Vec<String>,
) -> Result<String, Box<dyn Error>> {
    // println!("head_sessions match_headers: {:?}", &head_sessions);
    // println!("sessionmpst match_headers: {:?}", &sessionmpst);
    // println!("headers match_headers: {:?}", &headers);
    // println!("involved match_headers: {:?}", &involved);
    // println!("index match_headers: {:?}", &index);
    // println!("seen match_headers: {:?}", &seen);
    // println!("role match_headers: {:?}", &role);

    match involved[1].as_str() {
        h if h == roles_and_sessions[0] => match_full_types(
            roles_and_sessions[1],
            [
                &get_tail(&sessionmpst[0]),
                sessionmpst[1],
                &get_tail(&sessionmpst[2]),
                sessionmpst[3],
            ],
            involved,
            &get_head_payload(&sessionmpst[0]),
            role,
            hm,
            seen,
        ),
        h if h == roles_and_sessions[2] => match_full_types(
            roles_and_sessions[3],
            [
                sessionmpst[0],
                &get_tail(&sessionmpst[1]),
                &get_tail(&sessionmpst[2]),
                sessionmpst[3],
            ],
            involved,
            &get_head_payload(&sessionmpst[1]),
            role,
            hm,
            seen,
        ),
        h if h == "All" => all_type(sessionmpst, index, role, hm, seen),
        _ => panic!("Wrong receiver, not recognized: {}", involved[1]),
    }
}

#[doc(hidden)]
fn match_full_types<S: ::std::hash::BuildHasher>(
    head_session: &str,
    sessionmpst: [&str; 4],
    involved: [String; 2],
    payload: &str,
    role: &str,
    hm: &HashMap<String, &Vec<String>, S>,
    seen: &mut Vec<String>,
) -> Result<String, Box<dyn Error>> {
    // println!("sessionmpst match_full_types: {:?}", &sessionmpst);
    // println!("head_session match_full_types: {:?}", &head_session);

    match head_session {
        "Send" => send_type(sessionmpst, involved, payload, role, hm, seen, " + "),
        "Recv" => recv_type(sessionmpst, involved, payload, role, hm, seen, " & "),
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
        .replace("role::a::", "")
        .replace("role::b::", "")
        .replace("role::c::", "")
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
fn get_name(head: &str) -> String {
    match head {
        "RoleAtoAll" => String::from("All"),
        "RoleBtoAll" => String::from("All"),
        "RoleCtoAll" => String::from("All"),
        "RoleAlltoA" => String::from("A"),
        "RoleAlltoB" => String::from("B"),
        "RoleAlltoC" => String::from("C"),
        "RoleA" => String::from("A"),
        "RoleB" => String::from("B"),
        "RoleC" => String::from("C"),
        "RoleEnd" => String::from("End"),
        _ => panic!("Wrong head, not recognized: {}", head),
    }
}

#[doc(hidden)]
fn get_head(s: &str) -> String {
    // println!("get_head: {}", &s);

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

    // println!("result get_head : {}", &result[0]);

    String::from(result[0])
}

#[doc(hidden)]
fn get_head_payload(s: &str) -> String {
    // println!("get_head_payload: {}", &s);

    let payload = &get_two_tails(s)[0];

    // println!("payload get_head_payload: {}", &payload);

    if payload.contains("::") {
        String::from(payload.split("::").collect::<Vec<_>>()[1])
    } else {
        String::from(payload)
    }
}

#[doc(hidden)]
fn get_two_tails(s: &str) -> [String; 2] {
    // println!("get_two_tails: {}", &s);

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

    // println!("result get_two_tails: {}", &s);

    if result[1] == "" {
        let mut temp: [String; 2] = Default::default();
        temp[0].push_str(&result[0]);
        temp[1].push_str(&result[0]);
        temp
    } else {
        result
    }
}

#[doc(hidden)]
fn get_fields(s: &str) -> [String; 4] {
    // println!("get_fields: {}", &s);

    let mut result: [String; 4] = Default::default();
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

    // println!("result get_fields: {}", &s);

    result
}

#[doc(hidden)]
fn divide_either(s: &str) -> [String; 8] {
    // println!("divide_either: {}", &s);

    let mut result: [String; 8] = Default::default();
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

    // println!("result divide_either: {:?}", &result);

    result
}

#[doc(hidden)]
fn get_tail(s: &str) -> String {
    let result: Vec<&str> = s.split('<').collect();
    result[1..].join("<")
}

#[doc(hidden)]
fn get_dual(s: &str) -> String {
    // println!("Dual: {}", &s);

    let result = &s.replace("Send<", "Revc<");
    let result = &result.replace("Recv<", "Send<");
    let result = &result.replace("Revc<", "Recv<");
    let result = switch_role(&result, "A", "ADual");
    let result = switch_role(&result, "C", "CDual");
    let result = switch_role(&result, "B", "BDual");
    let result = switch_role(&result, "AtoAll", "AlltoA");
    let result = switch_role(&result, "BtoAll", "AlltoB");
    switch_role(&result, "CtoAll", "AlltoC")
}

#[doc(hidden)]
fn switch_role(s: &str, a: &str, b: &str) -> String {
    let result = &s.replace(&format!("Role{}<", a), &format!("Role{}<", "XxX"));
    let result = &result.replace(&format!("Role{}<", b), &format!("Role{}<", a));
    let result = &result.replace(&format!("Role{}<", "XxX"), &format!("Role{}<", a));
    String::from(result)
}

#[doc(hidden)]
fn send_type<S: ::std::hash::BuildHasher>(
    sessionmpst: [&str; 4],
    involved: [String; 2],
    payload: &str,
    role: &str,
    hm: &HashMap<String, &Vec<String>, S>,
    seen: &mut Vec<String>,
    symbol: &str,
) -> Result<String, Box<dyn Error>> {
    // println!("sessionmpst send_type: {:?}", &sessionmpst);
    // println!("payload send_type: {:?}", &payload);

    if seen.contains(&String::from(payload)) {
        Ok(String::from("X"))
    } else if hm.contains_key(payload) {
        recurs_type(payload, role, hm, seen, symbol)
    } else {
        Ok(format!(
            "{}!{}.{}",
            involved[0],
            involved[1],
            checker_aux(sessionmpst, role, &hm, seen)?
        ))
    }
}

#[doc(hidden)]
fn recv_type<S: ::std::hash::BuildHasher>(
    sessionmpst: [&str; 4],
    involved: [String; 2],
    payload: &str,
    role: &str,
    hm: &HashMap<String, &Vec<String>, S>,
    seen: &mut Vec<String>,
    symbol: &str,
) -> Result<String, Box<dyn Error>> {
    // println!("sessionmpst recv_type: {:?}", &sessionmpst);
    // println!("payload recv_type: {:?}", &payload);

    if payload.contains("Either") {
        let branching: [String; 8] = divide_either(payload);
        Ok(format!(
            "( {} & {} )",
            checker_aux(
                [&branching[0], &branching[1], &branching[2], &branching[3]],
                role,
                &hm,
                seen
            )?,
            checker_aux(
                [&branching[4], &branching[5], &branching[6], &branching[7]],
                role,
                &hm,
                seen
            )?,
        ))
    } else if seen.contains(&String::from(payload)) {
        Ok(String::from("X"))
    } else if hm.contains_key(payload) {
        recurs_type(payload, role, hm, seen, symbol)
    } else {
        Ok(format!(
            "{}?{}.{}",
            involved[0],
            involved[1],
            checker_aux(sessionmpst, role, &hm, seen)?
        ))
    }
}

#[doc(hidden)]
fn recurs_type<S: ::std::hash::BuildHasher>(
    payload: &str,
    role: &str,
    hm: &HashMap<String, &Vec<String>, S>,
    seen: &mut Vec<String>,
    symbol: &str,
) -> Result<String, Box<dyn Error>> {
    // println!("payload recurs_type: {}", &payload);

    let mut vec_result = Vec::new();
    let mut recurs = false;

    seen.push(String::from(payload));

    match hm.get(payload) {
        Some(&branches) => {
            for branch in branches.iter() {
                let sessions_and_stack = get_fields(&parse_type(branch));
                let result_branch = checker_aux(
                    [
                        &sessions_and_stack[0],
                        &sessions_and_stack[1],
                        &sessions_and_stack[2],
                        &sessions_and_stack[3],
                    ],
                    role,
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

    // println!("result recurs_type: {}", &result);

    if recurs {
        Ok(format!("µX( {} )", result))
    } else {
        Ok(format!("( {} )", result))
    }
}

#[doc(hidden)]
fn all_type<S: ::std::hash::BuildHasher>(
    sessionmpst: [&str; 4],
    index: [usize; 4],
    role: &str,
    hm: &HashMap<String, &Vec<String>, S>,
    seen: &mut Vec<String>,
) -> Result<String, Box<dyn Error>> {
    // println!("sessionmpst all_type: {:?}", &sessionmpst);

    let payload_1 = get_head_payload(&sessionmpst[0]);
    let payload_2 = get_head_payload(&sessionmpst[1]);
    if payload_1.contains("Either") && payload_2.contains("Either") {
        let branching_1: [String; 8] = divide_either(&payload_1);
        let branching_2: [String; 8] = divide_either(&payload_2);
        let tails: [String; 2] = get_two_tails(&sessionmpst[2]);

        // println!("tails: {:?}", &tails);

        Ok(format!(
            "( {} + {} )",
            checker_aux(
                [
                    &get_dual(&branching_1[index[0]]),
                    &get_dual(&branching_2[index[1]]),
                    &tails[0],
                    sessionmpst[3]
                ],
                role,
                &hm,
                seen
            )?,
            checker_aux(
                [
                    &get_dual(&branching_1[index[2]]),
                    &get_dual(&branching_2[index[3]]),
                    &tails[1],
                    sessionmpst[3]
                ],
                role,
                &hm,
                seen
            )?,
        ))
    } else if payload_1.contains("Either") {
        let branching_1: [String; 8] = divide_either(&payload_1);
        let tails: [String; 2] = get_two_tails(&sessionmpst[2]);

        // println!("tails: {:?}", &tails);

        Ok(format!(
            "( {} + {} )",
            checker_aux(
                [
                    &get_dual(&branching_1[index[0]]),
                    &get_dual(&sessionmpst[1]),
                    &tails[0],
                    sessionmpst[3]
                ],
                role,
                &hm,
                seen
            )?,
            checker_aux(
                [
                    &get_dual(&branching_1[index[2]]),
                    &get_dual(&sessionmpst[1]),
                    &tails[1],
                    sessionmpst[3]
                ],
                role,
                &hm,
                seen
            )?,
        ))
    } else if payload_2.contains("Either") {
        let branching_2: [String; 8] = divide_either(&payload_2);
        let tails: [String; 2] = get_two_tails(&sessionmpst[2]);

        // println!("tails: {:?}", &tails);

        Ok(format!(
            "( {} + {} )",
            checker_aux(
                [
                    &get_dual(&sessionmpst[0]),
                    &get_dual(&branching_2[index[1]]),
                    &tails[0],
                    sessionmpst[3]
                ],
                role,
                &hm,
                seen
            )?,
            checker_aux(
                [
                    &get_dual(&sessionmpst[0]),
                    &get_dual(&branching_2[index[3]]),
                    &tails[1],
                    sessionmpst[3]
                ],
                role,
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
