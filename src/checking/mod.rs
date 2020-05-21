// use binary::{recv, send, End, Recv, Send, Session};
use binary::Session;
use role::Role;
use sessionmpst::SessionMpst;

use std::any::type_name;
// use std::collections::HashMap;
use std::error::Error;

/// Displays the local endpoints of each roles.
/// It is required that the `SessionMpst` are the root ones, and not a partial part included in a bigger one.
///
/// Returns unit if everything wen right.
pub fn checker<S1, S2, S3, R1, R2, R3>(
    s1: SessionMpst<S1, <S3 as Session>::Dual, R1>,
    s2: SessionMpst<<S1 as Session>::Dual, S2, R2>,
    s3: SessionMpst<S3, <S2 as Session>::Dual, R3>,
    // s1: SessionMpst<S1, S2, R1>,
    // s2: SessionMpst<S3, S4, R2>,
    // s3: SessionMpst<S5, S6, R3>,
) -> Result<(String, String, String), Box<dyn Error>>
where
    S1: Session + 'static,
    S2: Session + 'static,
    S3: Session + 'static,
    // S4: Session + 'static,
    // S5: Session + 'static,
    // S6: Session + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    R3: Role + 'static,
{
    // let mut result = HashMap::new();
    //let mut seen: &HashMap<String, String> = HashMap::new();

    // let (channel_ab, channel_ba) = S1::new();
    // let (channel_ca, channel_ac) = S3::new();
    // let (channel_bc, channel_cb) = S2::new();

    // let (role_a, _) = R1::new();
    // let (role_b, _) = R2::new();
    // let (role_c, _) = R3::new();

    // let a = SessionMpst {
    //     session1: channel_ab,
    //     session2: channel_ac,
    //     stack: role_a,
    // };
    // let b = SessionMpst {
    //     session1: channel_ba,
    //     session2: channel_bc,
    //     stack: role_b,
    // };
    // let c = SessionMpst {
    //     session1: channel_ca,
    //     session2: channel_cb,
    //     stack: role_c,
    // };

    // println!("role_a: {:?}", role_a);
    // println!("role_b: {:?}", role_b);
    // println!("role_c: {:?}", role_c);

    println!("SessionMpst A : {}", parse_type(type_of(&s1)));
    println!("SessionMpst B : {}", parse_type(type_of(&s2)));
    println!("SessionMpst C : {}", parse_type(type_of(&s3)));

    let result_1 = checker_aux(
        &parse_type(type_of(&s1.session1)),
        &parse_type(type_of(&s1.session2)),
        &parse_type(type_of(&s1.stack)),
    )?;
    let result_2 = checker_aux(
        &parse_type(type_of(&s2.session1)),
        &parse_type(type_of(&s2.session2)),
        &parse_type(type_of(&s2.stack)),
    )?;
    let result_3 = checker_aux(
        &parse_type(type_of(&s3.session1)),
        &parse_type(type_of(&s3.session2)),
        &parse_type(type_of(&s3.stack)),
    )?;

    println!("result_1: {}", &result_1);
    println!("result_2: {}", &result_2);
    println!("result_3: {}", &result_3);

    // result.insert(String::from("A"), result_1);
    // result.insert(String::from("B"), result_2);
    // result.insert(String::from("C"), result_3);

    Ok((
        format!("A: {}", &result_1),
        format!("B: {}", &result_2),
        format!("C: {}", &result_3),
    ))
}

fn checker_aux(session1: &str, session2: &str, stack: &str) -> Result<String, Box<dyn Error>> {
    let head_stack = get_head(&stack);
    let head_session1 = get_head(&session1);
    let head_session2 = get_head(&session2);

    println!("Session 1 : {}", &session1);
    println!("Session 2 : {}", &session2);
    println!("Stack : {}", &stack);
    println!("Head 1 : {}", get_head(&session1));
    println!("Head 2 : {}", get_head(&session2));
    println!("Head stack : {}", get_head(&stack));
    println!("Payload 1 : {}", get_head_payload(&session1));
    println!("Payload 2 : {}", get_head_payload(&session2));

    if !head_stack.contains("RoleEnd") {
        let (sender, receiver) = get_name(&head_stack);

        let result = match sender.as_str() {
            "A" => match receiver.as_str() {
                "B" => match head_session1.as_str() {
                    "Send" => format!(
                        "{}!{}.{}",
                        sender,
                        receiver,
                        checker_aux(&get_tail(&session1), session2, &get_tail(&stack))?
                    ),
                    "Recv" => {
                        if get_head_payload(&session1).contains("Either") {
                            let branching: [String; 6] =
                                divide_either(&get_head_payload(&session1));
                            format!(
                                "( {} & {} )",
                                checker_aux(&branching[0], &branching[1], &branching[2])?,
                                checker_aux(&branching[3], &branching[4], &branching[5])?,
                            )
                        } else {
                            format!(
                                "{}?{}.{}",
                                sender,
                                receiver,
                                checker_aux(&get_tail(&session1), session2, &get_tail(&stack))?
                            )
                        }
                    }
                    _ => panic!("Wrong session type, not recognized: {}", head_session1),
                },
                "C" => match head_session2.as_str() {
                    "Send" => format!(
                        "{}!{}.{}",
                        sender,
                        receiver,
                        checker_aux(session1, &get_tail(&session2), &get_tail(&stack))?
                    ),
                    "Recv" => {
                        if get_head_payload(&session2).contains("Either") {
                            let branching: [String; 6] =
                                divide_either(&get_head_payload(&session2));
                            format!(
                                "( {} & {} )",
                                checker_aux(&branching[0], &branching[1], &branching[2])?,
                                checker_aux(&branching[3], &branching[4], &branching[5])?,
                            )
                        } else {
                            format!(
                                "{}?{}.{}",
                                sender,
                                receiver,
                                checker_aux(session1, &get_tail(&session2), &get_tail(&stack))?
                            )
                        }
                    }
                    _ => panic!("Wrong session type, not recognized: {}", head_session2),
                },
                "All" => {
                    if get_head_payload(&session1).contains("Either")
                        && get_head_payload(&session2).contains("Either")
                    {
                        let branching_1: [String; 6] = divide_either(&get_head_payload(&session1));
                        let branching_2: [String; 6] = divide_either(&get_head_payload(&session2));
                        let tails: [String; 2] = get_two_tails(&stack);

                        println!(
                            "Choices: {:?} + {} / {:?} + {}",
                            &branching_1, &tails[0], &branching_2, &tails[1]
                        );

                        format!(
                            "( {} + {} )",
                            checker_aux(
                                &get_dual(&branching_1[0]),
                                &get_dual(&branching_2[0]),
                                &tails[0]
                            )?,
                            checker_aux(
                                &get_dual(&branching_1[3]),
                                &get_dual(&branching_2[3]),
                                &tails[1]
                            )?,
                        )
                    } else if get_head_payload(&session1).contains("Either") {
                        let branching_1: [String; 6] = divide_either(&get_head_payload(&session1));
                        let tails: [String; 2] = get_two_tails(&stack);

                        format!(
                            "( {} + {} )",
                            checker_aux(
                                &get_dual(&branching_1[0]),
                                &get_dual(&session2),
                                &tails[0]
                            )?,
                            checker_aux(
                                &get_dual(&branching_1[3]),
                                &get_dual(&session2),
                                &tails[1]
                            )?,
                        )
                    } else if get_head_payload(&session2).contains("Either") {
                        let branching_2: [String; 6] = divide_either(&get_head_payload(&session2));
                        let tails: [String; 2] = get_two_tails(&stack);

                        format!(
                            "( {} + {} )",
                            checker_aux(
                                &get_dual(&session1),
                                &get_dual(&branching_2[0]),
                                &tails[0]
                            )?,
                            checker_aux(
                                &get_dual(&session1),
                                &get_dual(&branching_2[3]),
                                &tails[1]
                            )?,
                        )
                    } else {
                        panic!(
                            "Wrong payloads, not recognized: {:?} , {:?} and {:?} ( {:?} / {:?} ) for {:?} , {:?} and {:?}",
                            divide_either(&get_head_payload(&session1)),
                            divide_either(&get_head_payload(&session2)),
                            get_two_tails(&stack),
                            &get_head_payload(&session1),
                            &get_head_payload(&session2),
                            &session1,
                            &session2,
                            &stack
                        );
                    }
                }
                _ => panic!("Wrong receiver, not recognized: {}", receiver),
            },
            "B" => match receiver.as_str() {
                "A" => match head_session1.as_str() {
                    "Send" => format!(
                        "{}!{}.{}",
                        sender,
                        receiver,
                        checker_aux(&get_tail(&session1), session2, &get_tail(&stack))?
                    ),
                    "Recv" => {
                        if get_head_payload(&session1).contains("Either") {
                            let branching: [String; 6] =
                                divide_either(&get_head_payload(&session1));
                            format!(
                                "( {} & {} )",
                                checker_aux(&branching[0], &branching[1], &branching[2])?,
                                checker_aux(&branching[3], &branching[4], &branching[5])?,
                            )
                        } else {
                            format!(
                                "{}?{}.{}",
                                sender,
                                receiver,
                                checker_aux(&get_tail(&session1), session2, &get_tail(&stack))?
                            )
                        }
                    }
                    _ => panic!("Wrong session type, not recognized: {}", head_session1),
                },
                "C" => match head_session2.as_str() {
                    "Send" => format!(
                        "{}!{}.{}",
                        sender,
                        receiver,
                        checker_aux(session1, &get_tail(&session2), &get_tail(&stack))?
                    ),
                    "Recv" => {
                        if get_head_payload(&session2).contains("Either") {
                            let branching: [String; 6] =
                                divide_either(&get_head_payload(&session2));
                            format!(
                                "( {} & {} )",
                                checker_aux(&branching[0], &branching[1], &branching[2])?,
                                checker_aux(&branching[3], &branching[4], &branching[5])?,
                            )
                        } else {
                            format!(
                                "{}?{}.{}",
                                sender,
                                receiver,
                                checker_aux(session1, &get_tail(&session2), &get_tail(&stack))?
                            )
                        }
                    }
                    _ => panic!("Wrong session type, not recognized: {}", head_session2),
                },
                "All" => {
                    if get_head_payload(&session1).contains("Either")
                        && get_head_payload(&session2).contains("Either")
                    {
                        let branching_1: [String; 6] = divide_either(&get_head_payload(&session1));
                        let branching_2: [String; 6] = divide_either(&get_head_payload(&session2));
                        let tails: [String; 2] = get_two_tails(&stack);

                        format!(
                            "( {} + {} )",
                            checker_aux(
                                &get_dual(&branching_1[0]),
                                &get_dual(&branching_2[1]),
                                &tails[0]
                            )?,
                            checker_aux(
                                &get_dual(&branching_1[3]),
                                &get_dual(&branching_2[4]),
                                &tails[1]
                            )?,
                        )
                    } else if get_head_payload(&session1).contains("Either") {
                        let branching_1: [String; 6] = divide_either(&get_head_payload(&session1));
                        let tails: [String; 2] = get_two_tails(&stack);

                        format!(
                            "( {} + {} )",
                            checker_aux(
                                &get_dual(&branching_1[0]),
                                &get_dual(&session2),
                                &tails[0]
                            )?,
                            checker_aux(
                                &get_dual(&branching_1[3]),
                                &get_dual(&session2),
                                &tails[1]
                            )?,
                        )
                    } else if get_head_payload(&session2).contains("Either") {
                        let branching_2: [String; 6] = divide_either(&get_head_payload(&session2));
                        let tails: [String; 2] = get_two_tails(&stack);

                        format!(
                            "( {} + {} )",
                            checker_aux(
                                &get_dual(&session1),
                                &get_dual(&branching_2[1]),
                                &tails[0]
                            )?,
                            checker_aux(
                                &get_dual(&session1),
                                &get_dual(&branching_2[4]),
                                &tails[1]
                            )?,
                        )
                    } else {
                        panic!(
                            "Wrong payloads, not recognized: {:?} , {:?} and {:?} ( {:?} / {:?} ) for {:?} , {:?} and {:?}",
                            divide_either(&get_head_payload(&session1)),
                            divide_either(&get_head_payload(&session2)),
                            get_two_tails(&stack),
                            &get_head_payload(&session1),
                            &get_head_payload(&session2),
                            &session1,
                            &session2,
                            &stack
                        );
                    }
                }
                _ => panic!("Wrong receiver, not recognized: {}", receiver),
            },
            "C" => match receiver.as_str() {
                "A" => match head_session1.as_str() {
                    "Send" => format!(
                        "{}!{}.{}",
                        sender,
                        receiver,
                        checker_aux(&get_tail(&session1), session2, &get_tail(&stack))?
                    ),
                    "Recv" => {
                        if get_head_payload(&session1).contains("Either") {
                            let branching: [String; 6] =
                                divide_either(&get_head_payload(&session1));
                            format!(
                                "( {} & {} )",
                                checker_aux(&branching[0], &branching[1], &branching[2])?,
                                checker_aux(&branching[3], &branching[4], &branching[5])?,
                            )
                        } else {
                            format!(
                                "{}?{}.{}",
                                sender,
                                receiver,
                                checker_aux(&get_tail(&session1), session2, &get_tail(&stack))?
                            )
                        }
                    }
                    _ => panic!("Wrong session type, not recognized: {}", head_session1),
                },
                "B" => match head_session2.as_str() {
                    "Send" => format!(
                        "{}!{}.{}",
                        sender,
                        receiver,
                        checker_aux(session1, &get_tail(&session2), &get_tail(&stack))?
                    ),
                    "Recv" => {
                        if get_head_payload(&session2).contains("Either") {
                            let branching: [String; 6] =
                                divide_either(&get_head_payload(&session2));
                            format!(
                                "( {} & {} )",
                                checker_aux(&branching[0], &branching[1], &branching[2])?,
                                checker_aux(&branching[3], &branching[4], &branching[5])?,
                            )
                        } else {
                            format!(
                                "{}?{}.{}",
                                sender,
                                receiver,
                                checker_aux(session1, &get_tail(&session2), &get_tail(&stack))?
                            )
                        }
                    }
                    _ => panic!("Wrong session type, not recognized: {}", head_session2),
                },
                "All" => {
                    if get_head_payload(&session1).contains("Either")
                        && get_head_payload(&session2).contains("Either")
                    {
                        let branching_1: [String; 6] = divide_either(&get_head_payload(&session1));
                        let branching_2: [String; 6] = divide_either(&get_head_payload(&session2));
                        let tails: [String; 2] = get_two_tails(&stack);

                        format!(
                            "( {} + {} )",
                            checker_aux(
                                &get_dual(&branching_1[1]),
                                &get_dual(&branching_2[1]),
                                &tails[0]
                            )?,
                            checker_aux(
                                &get_dual(&branching_1[4]),
                                &get_dual(&branching_2[4]),
                                &tails[1]
                            )?,
                        )
                    } else if get_head_payload(&session1).contains("Either") {
                        let branching_1: [String; 6] = divide_either(&get_head_payload(&session1));
                        let tails: [String; 2] = get_two_tails(&stack);

                        format!(
                            "( {} + {} )",
                            checker_aux(
                                &get_dual(&branching_1[1]),
                                &get_dual(&session2),
                                &tails[0]
                            )?,
                            checker_aux(
                                &get_dual(&branching_1[4]),
                                &get_dual(&session2),
                                &tails[1]
                            )?,
                        )
                    } else if get_head_payload(&session2).contains("Either") {
                        let branching_2: [String; 6] = divide_either(&get_head_payload(&session2));
                        let tails: [String; 2] = get_two_tails(&stack);

                        format!(
                            "( {} + {} )",
                            checker_aux(
                                &get_dual(&session1),
                                &get_dual(&branching_2[1]),
                                &tails[0]
                            )?,
                            checker_aux(
                                &get_dual(&session1),
                                &get_dual(&branching_2[4]),
                                &tails[1]
                            )?,
                        )
                    } else {
                        panic!(
                            "Wrong payloads, not recognized: {:?} , {:?} and {:?} ( {:?} / {:?} ) for {:?} , {:?} and {:?}",
                            divide_either(&get_head_payload(&session1)),
                            divide_either(&get_head_payload(&session2)),
                            get_two_tails(&stack),
                            &get_head_payload(&session1),
                            &get_head_payload(&session2),
                            &session1,
                            &session2,
                            &stack
                        );
                    }
                }
                _ => panic!("Wrong receiver, not recognized: {}", receiver),
            },
            _ => panic!("Wrong sender, not recognized: {}", sender),
        };
        Ok(result)
    } else {
        Ok(String::from("0"))
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn parse_type(s: &str) -> String {
    let result = &s.replace("&", "");
    let result = &result.replace("mpstthree::", "");
    let result = &result.replace("sessionmpst::", "");
    let result = &result.replace("binary::", "");
    let result = &result.replace("role::a_to_b::", "");
    let result = &result.replace("role::a_to_c::", "");
    let result = &result.replace("role::b_to_a::", "");
    let result = &result.replace("role::b_to_c::", "");
    let result = &result.replace("role::c_to_a::", "");
    let result = &result.replace("role::c_to_b::", "");
    let result = &result.replace("role::a_to_all::", "");
    let result = &result.replace("role::b_to_all::", "");
    let result = &result.replace("role::c_to_all::", "");
    let result = &result.replace("role::all_to_a::", "");
    let result = &result.replace("role::all_to_b::", "");
    let result = &result.replace("role::all_to_c::", "");
    let result = &result.replace("role::end::", "");
    let result = &result.replace("either::", "");
    let result = &result.replace("checker::", "");
    result.chars().filter(|c| !c.is_whitespace()).collect()
}

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

fn get_head_payload(s: &str) -> String {
    String::from(&get_two_tails(s)[0])
}

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
    println!("Result get_two_tails: {:?} / {}", result, s);
    result
}

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
    println!("Result divide_either: {:?} / {}", result, s);
    result
}

fn get_tail(s: &str) -> String {
    let result: Vec<&str> = s.split('<').collect();
    result[1..].join("<")
}

fn get_dual(s: &str) -> String {
    // Switch Send / Recv
    let result = &s.replace("Send<", "Revc<");
    let result = &result.replace("Recv<", "Send<");
    let result = &result.replace("Revc<", "Recv<");
    // Switch RoleAtoB / RoleBtoA
    let result = &result.replace("RoleAtoB", "RoleBtoX");
    let result = &result.replace("RoleBtoA", "RoleAtoB");
    let result = &result.replace("RoleBtoX", "RoleBtoA");
    // Switch RoleAtoC / RoleCtoA
    let result = &result.replace("RoleAtoC", "RoleCtoX");
    let result = &result.replace("RoleCtoA", "RoleAtoC");
    let result = &result.replace("RoleCtoX", "RoleCtoA");
    // Switch RoleBtoC / RoleCtoB
    let result = &result.replace("RoleCtoB", "RoleBtoX");
    let result = &result.replace("RoleBtoC", "RoleCtoB");
    let result = &result.replace("RoleBtoX", "RoleBtoC");
    // Switch RoleAtoAll / RoleAlltoA
    let result = &result.replace("RoleAtoAll", "RoleAlltoX");
    let result = &result.replace("RoleAlltoA", "RoleAtoAll");
    let result = &result.replace("RoleAlltoX", "RoleAlltoA");
    // Switch RoleBtoAll / RoleAlltoB
    let result = &result.replace("RoleBtoAll", "RoleAlltoX");
    let result = &result.replace("RoleAlltoB", "RoleBtoAll");
    let result = &result.replace("RoleAlltoX", "RoleAlltoB");
    // Switch RoleCtoAll / RoleAlltoC
    let result = &result.replace("RoleCtoAll", "RoleAlltoX");
    let result = &result.replace("RoleAlltoC", "RoleCtoAll");
    let result = &result.replace("RoleAlltoX", "RoleAlltoC");
    String::from(result)
}
