use std::any::type_name;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::error::Error;

type TupleHashmaps<'a, BH1, BH2> = (
    &'a HashMap<String, &'a Vec<String>, BH1>,
    &'a HashMap<String, &'a Vec<String>, BH2>,
);

#[doc(hidden)]
pub(crate) fn checker_aux<BH1: ::std::hash::BuildHasher, BH2: ::std::hash::BuildHasher>(
    sessionmpst: [&str; 4],
    role: &str,
    branches: TupleHashmaps<BH1, BH2>, // branches_receivers, branches_sender
    seen: &mut Vec<String>,
) -> Result<String, Box<dyn Error>> {
    let head_session1 = get_head(&sessionmpst[0]);
    let head_session2 = get_head(&sessionmpst[1]);
    let head_stack = get_head(&sessionmpst[2]);
    let sender = get_name(&get_head(&sessionmpst[3]));

    // println!();
    // println!("sender checker_aux: {:?}", &sender);
    // println!("sessionmpst checker_aux: {:?}", &sessionmpst);
    // println!("head_stack checker_aux: {:?}", &head_stack);

    // for (key, value) in &*branches.0 {
    // println!("Values: {} / {:?}", key, value);
    // }

    if !head_stack.contains("RoleEnd") {
        let receiver = get_name(&head_stack);

        let result = match sender.as_str() {
            "A" => match_headers(
                ["B", head_session1.as_str(), "C", head_session2.as_str()],
                sessionmpst,
                [sender, receiver],
                [0, 0, 4, 4],
                role,
                branches,
                seen,
            )?,
            "B" => match_headers(
                ["A", head_session1.as_str(), "C", head_session2.as_str()],
                sessionmpst,
                [sender, receiver],
                [0, 1, 4, 5],
                role,
                branches,
                seen,
            )?,
            "C" => match_headers(
                ["A", head_session1.as_str(), "B", head_session2.as_str()],
                sessionmpst,
                [sender, receiver],
                [1, 1, 5, 5],
                role,
                branches,
                seen,
            )?,
            "All" => match receiver.as_str() {
                "A" => match_recv_from_all("A", ["B", "C"], sessionmpst, role, branches, seen)?,
                "B" => match_recv_from_all("B", ["A", "C"], sessionmpst, role, branches, seen)?,
                "C" => match_recv_from_all("C", ["A", "B"], sessionmpst, role, branches, seen)?,
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
fn match_recv_from_all<BH1: ::std::hash::BuildHasher, BH2: ::std::hash::BuildHasher>(
    sender: &str,
    receivers: [&str; 2],
    sessionmpst: [&str; 4],
    role: &str,
    branches: TupleHashmaps<BH1, BH2>,
    seen: &mut Vec<String>,
) -> Result<String, Box<dyn Error>> {
    // println!("sessionmpst match_recv_from_all: {:?}",
    // &sessionmpst);

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
            branches,
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
            branches,
            seen,
        ),
        _ => panic!("Wrong role, not recognized: {}", role),
    }
}

#[doc(hidden)]
fn match_headers<BH1: ::std::hash::BuildHasher, BH2: ::std::hash::BuildHasher>(
    roles_and_sessions: [&str; 4], // role 1, session 1, role 2, session 2
    sessionmpst: [&str; 4],
    involved: [String; 2],
    index: [usize; 4],
    role: &str,
    branches: TupleHashmaps<BH1, BH2>,
    seen: &mut Vec<String>,
) -> Result<String, Box<dyn Error>> {
    // println!(
    //     "roles_and_sessions match_headers: {:?}",
    //     &roles_and_sessions
    // );
    // println!("sessionmpst match_headers: {:?}",
    // &sessionmpst); println!("involved match_headers:
    // {:?}", &involved); println!("index match_headers:
    // {:?}", &index); println!("seen match_headers: {:?}",
    // &seen); println!("role match_headers: {:?}", &role);
    // println!(
    //     "roles_and_sessions match_headers: {:?}",
    //     &roles_and_sessions
    // );

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
            [
                &get_head_payload(&sessionmpst[0]),
                &get_head_payload(&sessionmpst[1]),
            ],
            role,
            branches,
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
            [
                &get_head_payload(&sessionmpst[1]),
                &get_head_payload(&sessionmpst[0]),
            ],
            role,
            branches,
            seen,
        ),
        h if h == "All" => all_type(sessionmpst, index, role, branches, seen),
        _ => panic!("Wrong receiver, not recognized: {}", involved[1]),
    }
}

#[doc(hidden)]
fn match_full_types<BH1: ::std::hash::BuildHasher, BH2: ::std::hash::BuildHasher>(
    head_session: &str,
    sessionmpst: [&str; 4],
    involved: [String; 2],
    payloads: [&str; 2],
    role: &str,
    branches: TupleHashmaps<BH1, BH2>,
    seen: &mut Vec<String>,
) -> Result<String, Box<dyn Error>> {
    // println!("sessionmpst match_full_types: {:?}",
    // &sessionmpst); println!("head_session
    // match_full_types: {:?}", &head_session);

    match head_session {
        "Send" => send_type(sessionmpst, involved, payloads, role, branches, seen, " + "),
        "Recv" => recv_type(
            sessionmpst,
            involved,
            payloads[0],
            role,
            branches,
            seen,
            " & ",
        ),
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

    // println!("Start result: {:?}", &result);

    let mut to_remove: Vec<String> = Vec::new();

    let mut temp_str = String::from("");

    let mut previous_char = '$';

    for c in result.chars() {
        match c {
            ':' if previous_char == ':' => {
                temp_str.push(':');
                if !to_remove.contains(&temp_str) {
                    to_remove.push(temp_str);
                }
                temp_str = String::from("");
            }
            ':' => {
                temp_str.push(c);
                previous_char = c;
            }
            '_' => {
                temp_str.push(c);
                previous_char = c;
            }
            '&' => {
                temp_str.push(c);
                previous_char = c;
            }
            c if c.is_alphanumeric() => {
                temp_str.push(c);
                previous_char = c;
            }
            _ => {
                temp_str = String::from("");
                previous_char = '$';
            }
        }
    }

    to_remove.sort_by_key(|b| Reverse(b.chars().count()));

    // println!("to_remove: {:?}", &to_remove);

    let mut temp = String::from(result);

    for elt in to_remove.iter() {
        temp = temp.replace(elt, "");
    }

    result = &temp;

    // println!("End result: {:?}", &result);

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
                if index <= 0 && !result[index_session].is_empty() {
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

    if result[1].is_empty() {
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
                if index <= 0 && !result[index_session].is_empty() {
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
                if index <= 0 && !result[index_session].is_empty() {
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
fn send_type<BH1: ::std::hash::BuildHasher, BH2: ::std::hash::BuildHasher>(
    sessionmpst: [&str; 4],
    involved: [String; 2],
    payloads: [&str; 2],
    role: &str,
    branches: TupleHashmaps<BH1, BH2>,
    seen: &mut Vec<String>,
    symbol: &str,
) -> Result<String, Box<dyn Error>> {
    // println!("sessionmpst send_type: {:?}", &sessionmpst);
    // println!("payload send_type: {:?}", &payloads);

    if seen.contains(&String::from(payloads[0])) {
        Ok(String::from("X"))
    } else if branches.0.contains_key(payloads[0]) && branches.0.contains_key(payloads[1]) {
        // println!("possible new stack: {:?}", &sessionmpst);
        recurs_type(payloads, role, branches, seen, symbol)
    } else {
        // println!(
        //     "payload send_type: {:?} / {:?} / {:?} / {:?}",
        //     involved[0], involved[1], &payloads, role
        // );

        Ok(format!(
            "{}!{}.{}",
            involved[0],
            involved[1],
            checker_aux(sessionmpst, role, branches, seen)?
        ))
    }
}

#[doc(hidden)]
fn recv_type<BH1: ::std::hash::BuildHasher, BH2: ::std::hash::BuildHasher>(
    sessionmpst: [&str; 4],
    involved: [String; 2],
    payload: &str,
    role: &str,
    branches: TupleHashmaps<BH1, BH2>,
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
                branches,
                seen
            )?,
            checker_aux(
                [&branching[4], &branching[5], &branching[6], &branching[7]],
                role,
                branches,
                seen
            )?,
        ))
    } else if seen.contains(&String::from(payload)) {
        Ok(String::from("X"))
    } else if branches.0.contains_key(payload) {
        recurs_type([payload, ""], role, branches, seen, symbol)
    } else {
        // println!(
        //     "payload recv_type: {:?} / {:?} / {:?} / {:?}",
        //     involved[0], involved[1], &payload, role
        // );

        Ok(format!(
            "{}?{}.{}",
            involved[0],
            involved[1],
            checker_aux(sessionmpst, role, branches, seen)?
        ))
    }
}

#[doc(hidden)]
fn change_order(
    sessions_and_stack: &[String; 4],
    full_role: &str,
    previous_role: &str,
) -> (String, String) {
    match full_role {
        "RoleA" => match previous_role {
            "RoleB" => (
                get_dual(&sessions_and_stack[0]),
                get_dual(&sessions_and_stack[1]),
            ),
            "RoleC" => (
                get_dual(&sessions_and_stack[1]),
                get_dual(&sessions_and_stack[0]),
            ),
            _ => panic!("Wrong roles {} / {}", previous_role, full_role),
        },
        "RoleB" => match previous_role {
            "RoleA" => (
                get_dual(&sessions_and_stack[0]),
                get_dual(&sessions_and_stack[1]),
            ),
            "RoleC" => (
                get_dual(&sessions_and_stack[0]),
                get_dual(&sessions_and_stack[1]),
            ),
            _ => panic!("Wrong roles {} / {}", previous_role, full_role),
        },
        "RoleC" => match previous_role {
            "RoleA" => (
                get_dual(&sessions_and_stack[0]),
                get_dual(&sessions_and_stack[1]),
            ),
            "RoleB" => (
                get_dual(&sessions_and_stack[0]),
                get_dual(&sessions_and_stack[1]),
            ),
            _ => panic!("Wrong roles {} / {}", previous_role, full_role),
        },
        _ => panic!("Wrong roles {} / {}", previous_role, full_role),
    }
}

#[doc(hidden)]
fn recurs_type<BH1: ::std::hash::BuildHasher, BH2: ::std::hash::BuildHasher>(
    payloads: [&str; 2],
    role: &str,
    branches: TupleHashmaps<BH1, BH2>,
    seen: &mut Vec<String>,
    symbol: &str,
) -> Result<String, Box<dyn Error>> {
    // println!("payload recurs_type: {:?}", &payloads);
    // println!(
    //     "branches_receivers recurs_type 1: {:?}",
    //     branches.0.get(payloads[0])
    // );
    // println!(
    //     "branches_receivers recurs_type 2: {:?}",
    //     branches.0.get(payloads[1])
    // );

    let mut vec_result = Vec::new();
    let mut recurs = false;

    seen.push(String::from(payloads[0]));
    seen.push(String::from(payloads[1]));

    match (
        branches.0.get(payloads[0]),
        branches.0.get(payloads[1]),
        branches.1.get(payloads[0]),
    ) {
        (Some(branches0), Some(branches1), Some(&stack)) => {
            for (i, _) in branches0.iter().enumerate() {
                let full_role: &str = &format!("Role{}", role);

                let sessions_and_stack_0 = get_fields(&parse_type(&branches0[i]));
                let sessions_and_stack_1 = get_fields(&parse_type(&branches1[i]));

                let new_sessions_and_stack = match role {
                    "A" => [
                        sessions_and_stack_0[0].clone(),
                        sessions_and_stack_1[0].clone(),
                        sessions_and_stack_0[2].clone(),
                        sessions_and_stack_0[3].clone(),
                    ],
                    "B" => [
                        sessions_and_stack_0[0].clone(),
                        sessions_and_stack_1[1].clone(),
                        sessions_and_stack_0[2].clone(),
                        sessions_and_stack_0[3].clone(),
                    ],
                    "C" => [
                        sessions_and_stack_0[1].clone(),
                        sessions_and_stack_1[1].clone(),
                        sessions_and_stack_0[2].clone(),
                        sessions_and_stack_0[3].clone(),
                    ],
                    _ => panic!("Wrong role: {}", role),
                };

                // println!("new_sessions_and_stack: {:?}",
                // new_sessions_and_stack);

                let previous_role: &str =
                    new_sessions_and_stack[3].split('<').collect::<Vec<_>>()[0];

                // println!("previous_role: {:?}", previous_role);

                // println!("full_role: {:?}", full_role);

                let new_order = change_order(&new_sessions_and_stack, full_role, previous_role);

                let new_stack = &parse_type(&stack[i]);

                // println!("new_stack: {:?}", new_stack);

                // println!(
                //     "Coco: {} / {} / {} / {}",
                //     &new_order.0,
                //     &new_order.1,
                //     new_stack,
                //     &format!("Role{}<RoleEnd>", role),
                // );

                let result_branch = checker_aux(
                    [
                        &new_order.0,
                        &new_order.1,
                        new_stack,
                        &format!("Role{}<RoleEnd>", role),
                    ],
                    role,
                    branches,
                    seen,
                )?;

                recurs = result_branch.contains(&String::from(".X"))
                    || result_branch.contains(&String::from(" X "))
                    || recurs;
                vec_result.push(result_branch);

                // println!("partial vec_result: {:?}",
                // vec_result);
            }
        }
        (Some(branches0), None, _) => {
            for branch in branches0.iter() {
                let sessions_and_stack = get_fields(&parse_type(branch));

                // if symbol != " + " {
                let result_branch = checker_aux(
                    [
                        &sessions_and_stack[0],
                        &sessions_and_stack[1],
                        &sessions_and_stack[2],
                        &sessions_and_stack[3],
                    ],
                    role,
                    branches,
                    seen,
                )?;
                recurs = result_branch.contains(&String::from(".X")) || recurs;
                vec_result.push(result_branch);
            }
        }
        _ => {
            panic!(
                "Error with hashmap and payload: {:?} / {:?} / {:?}",
                branches.0, branches.1, payloads
            )
        }
    }

    let result = vec_result.join(symbol);

    // println!("result recurs_type: {}", &result);
    // println!();
    // println!();

    if recurs {
        Ok(format!("µX( {} )", result))
    } else {
        Ok(format!("( {} )", result))
    }
}

#[doc(hidden)]
fn all_type<BH1: ::std::hash::BuildHasher, BH2: ::std::hash::BuildHasher>(
    sessionmpst: [&str; 4],
    index: [usize; 4],
    role: &str,
    branches: TupleHashmaps<BH1, BH2>,
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
                branches,
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
                branches,
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
                branches,
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
                branches,
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
                branches,
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
                branches,
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

//////////////////////////////////

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for
    // mod tests) scope.
    use super::*;

    #[test]
    #[should_panic]
    fn get_head_panic() {
        get_name("");
    }

    #[test]
    #[should_panic]
    fn match_full_types_panic() {
        let _ = match_full_types(
            "",
            ["", "", "", ""],
            [String::from(""), String::from("")],
            ["", ""],
            "",
            (&HashMap::new(), &HashMap::new()),
            &mut vec![],
        );
    }

    #[test]
    #[should_panic]
    fn match_headers_panic() {
        let _ = match_headers(
            ["", "", "", ""],
            ["", "", "", ""],
            [String::from(""), String::from("")],
            [0, 0, 0, 0],
            "",
            (&HashMap::new(), &HashMap::new()),
            &mut vec![],
        );
    }

    #[test]
    fn get_head_either() {
        let test = "Either<Left, Right>";
        assert_eq!(get_head(test), String::from(test));
    }

    #[test]
    fn send_type_x() {
        let test = send_type(
            ["", "", "", ""],
            [String::from(""), String::from("")],
            ["A", ""],
            "",
            (&HashMap::new(), &HashMap::new()),
            &mut vec![String::from("A")],
            "",
        )
        .unwrap();

        assert_eq!(test, String::from("X"));
    }

    #[test]
    #[should_panic]
    fn recurs_type_panic() {
        let _ = recurs_type(
            ["", ""],
            "",
            (&HashMap::new(), &HashMap::new()),
            &mut vec![],
            "",
        );
    }

    #[test]
    #[should_panic]
    fn match_recv_from_all_panic_at_checker_aux_0() {
        let _ = match_recv_from_all(
            "",
            ["", ""],
            ["", "", "", ""],
            "",
            (&HashMap::new(), &HashMap::new()),
            &mut vec![],
        );
    }

    #[test]
    #[should_panic]
    fn match_recv_from_all_panic_at_checker_aux_1() {
        let _ = match_recv_from_all(
            "",
            ["", ""],
            ["A", "", "", ""],
            "",
            (&HashMap::new(), &HashMap::new()),
            &mut vec![],
        );
    }

    #[test]
    #[should_panic]
    fn match_recv_from_all_panic() {
        let _ = match_recv_from_all(
            "",
            ["", ""],
            ["", "", "", ""],
            "A",
            (&HashMap::new(), &HashMap::new()),
            &mut vec![],
        );
    }

    #[test]
    #[should_panic]
    fn all_type_panic() {
        let _ = all_type(
            ["", "", "", ""],
            [0, 0, 0, 0],
            "",
            (&HashMap::new(), &HashMap::new()),
            &mut vec![],
        );
    }

    #[test]
    #[should_panic]
    fn checker_aux_panic() {
        let _ = checker_aux(
            [
                "End",
                "End",
                "RoleAlltoA<RoleEnd, RoleEnd>",
                "RoleA<RoleEnd>",
            ],
            "A",
            (&HashMap::new(), &HashMap::new()),
            &mut vec![],
        );
    }

    #[test]
    #[should_panic]
    fn checker_aux_panic_a() {
        let _ = checker_aux(
            [
                "End",
                "End",
                "RoleAlltoA<RoleEnd, RoleEnd>",
                "RoleA<RoleEnd>",
            ],
            "A",
            (&HashMap::new(), &HashMap::new()),
            &mut vec![],
        );
    }

    #[test]
    #[should_panic]
    fn checker_aux_panic_b() {
        let _ = checker_aux(
            [
                "End",
                "End",
                "RoleAlltoB<RoleEnd, RoleEnd>",
                "RoleB<RoleEnd>",
            ],
            "B",
            (&HashMap::new(), &HashMap::new()),
            &mut vec![],
        );
    }

    #[test]
    #[should_panic]
    fn checker_aux_panic_c() {
        let _ = checker_aux(
            [
                "End",
                "End",
                "RoleAlltoC<RoleEnd, RoleEnd>",
                "RoleC<RoleEnd>",
            ],
            "C",
            (&HashMap::new(), &HashMap::new()),
            &mut vec![],
        );
    }

    #[test]
    fn parse_type_test() {
        let test = parse_type("&mpstthree::binary::struct_trait::Recv<i32, mpstthree::binary::struct_trait::Send<i32, mpstthree::binary::struct_trait::Recv<06_a_usecase_recursive::Branche0CtoB<i32>, mpstthree::binary::struct_trait::End>>>");

        assert_eq!(
            test,
            String::from("Recv<i32,Send<i32,Recv<Branche0CtoB<i32>,End>>>")
        );
    }

    #[test]
    fn change_order_test() {
        let test = change_order(
            &[
                String::from("Send<End>"),
                String::from("Recv<End>"),
                String::from("RoleEnd"),
                String::from("RoleB<RoleEnd>>"),
            ],
            "RoleA",
            "RoleB",
        );

        assert_eq!(test.0, "Recv<End>");
        assert_eq!(test.1, "Send<End>");

        ////////////////////////////////////////////

        let test = change_order(
            &[
                String::from("Send<End>"),
                String::from("Recv<End>"),
                String::from("RoleEnd"),
                String::from("RoleA<RoleEnd>>"),
            ],
            "RoleB",
            "RoleA",
        );

        assert_eq!(test.0, "Recv<End>");
        assert_eq!(test.1, "Send<End>");

        ////////////////////////////////////////////

        let test = change_order(
            &[
                String::from("Send<End>"),
                String::from("Recv<End>"),
                String::from("RoleEnd"),
                String::from("RoleC<RoleEnd>>"),
            ],
            "RoleA",
            "RoleC",
        );

        assert_eq!(test.0, "Send<End>");
        assert_eq!(test.1, "Recv<End>");

        ////////////////////////////////////////////

        let test = change_order(
            &[
                String::from("Send<End>"),
                String::from("Recv<End>"),
                String::from("RoleEnd"),
                String::from("RoleA<RoleEnd>>"),
            ],
            "RoleC",
            "RoleA",
        );

        assert_eq!(test.0, "Recv<End>");
        assert_eq!(test.1, "Send<End>");

        ////////////////////////////////////////////

        let test = change_order(
            &[
                String::from("Send<End>"),
                String::from("Recv<End>"),
                String::from("RoleEnd"),
                String::from("RoleC<RoleEnd>>"),
            ],
            "RoleB",
            "RoleC",
        );

        assert_eq!(test.0, "Recv<End>");
        assert_eq!(test.1, "Send<End>");

        ////////////////////////////////////////////

        let test = change_order(
            &[
                String::from("Send<End>"),
                String::from("Recv<End>"),
                String::from("RoleEnd"),
                String::from("RoleB<RoleEnd>>"),
            ],
            "RoleC",
            "RoleB",
        );

        assert_eq!(test.0, "Recv<End>");
        assert_eq!(test.1, "Send<End>");
    }

    #[test]
    #[should_panic]
    fn change_order_panic_a_none() {
        change_order(
            &[
                String::from("Send<End>"),
                String::from("Recv<End>"),
                String::from("RoleEnd"),
                String::from("RoleB<RoleEnd>>"),
            ],
            "RoleA",
            "Role",
        );
    }

    #[test]
    #[should_panic]
    fn change_order_panic_b_none() {
        change_order(
            &[
                String::from("Send<End>"),
                String::from("Recv<End>"),
                String::from("RoleEnd"),
                String::from("RoleB<RoleEnd>>"),
            ],
            "RoleB",
            "Role",
        );
    }

    #[test]
    #[should_panic]
    fn change_order_panic_c_none() {
        change_order(
            &[
                String::from("Send<End>"),
                String::from("Recv<End>"),
                String::from("RoleEnd"),
                String::from("RoleB<RoleEnd>>"),
            ],
            "RoleC",
            "Role",
        );
    }

    #[test]
    #[should_panic]
    fn change_order_panic_none_a() {
        change_order(
            &[
                String::from("Send<End>"),
                String::from("Recv<End>"),
                String::from("RoleEnd"),
                String::from("RoleB<RoleEnd>>"),
            ],
            "Role",
            "RoleA",
        );
    }
}
