// use binary::{recv, send, End, Recv, Send, Session};
use binary::Session;
use role::Role;
use sessionmpst::SessionMpst;

use std::any::type_name;
use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::marker;

// use role::a_to_b::{next_a_to_b, RoleAtoB};
// use role::a_to_c::{next_a_to_c, RoleAtoC};
// use role::b_to_a::{next_b_to_a, RoleBtoA};
// use role::b_to_c::{next_b_to_c, RoleBtoC};
// use role::c_to_a::{next_c_to_a, RoleCtoA};
// use role::c_to_b::{next_c_to_b, RoleCtoB};
// use role::end::RoleEnd;

// use functionmpst::recv::recv_mpst_a_to_c;
// use functionmpst::recv::recv_mpst_b_to_a;
// use functionmpst::recv::recv_mpst_c_to_b;

// use functionmpst::send::send_mpst_a_to_b;
// use functionmpst::send::send_mpst_b_to_c;
// use functionmpst::send::send_mpst_c_to_a;

/// Displays the local endpoints of each roles.
/// It is required that the `SessionMpst` are the root ones, and not a partial part included in a bigger one.
///
/// Returns unit if everything wen right.
pub fn checker<S1, S2, S3, R1, R2, R3>(
    // s1: *mut SessionMpst<S1, <S3 as Session>::Dual, R1>,
    // s2: *mut SessionMpst<<S1 as Session>::Dual, S2, R2>,
    // s3: *mut SessionMpst<S3, <S2 as Session>::Dual, R3>,
    s1: SessionMpst<S1, <S3 as Session>::Dual, R1>,
    s2: SessionMpst<<S1 as Session>::Dual, S2, R2>,
    s3: SessionMpst<S3, <S2 as Session>::Dual, R3>,
) -> Result<(), Box<dyn Error>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    R1: Role,
    R2: Role,
    R3: Role,
{
    let mut result = HashMap::new();
    //let mut seen: &HashMap<String, String> = HashMap::new();

    let result_1 = checker_aux(s1)?;
    let result_2 = checker_aux(s2)?;
    let result_3 = checker_aux(s3)?;

    println!("result_1: {}", result_1);
    println!("result_2: {}", result_2);
    println!("result_3: {}", result_3);

    result.insert(String::from("A"), result_1);
    result.insert(String::from("B"), result_2);
    result.insert(String::from("C"), result_3);

    Ok(())
}

fn checker_aux<S1, S2, R>(s: SessionMpst<S1, S2, R>) -> Result<String, Box<dyn Error>>
where
    S1: Session,
    S2: Session,
    R: Role,
{
    let head = R::head();

    let _parsed = parse_type(type_of(&s));

    if head.contains("toAll") {
        let (_sender, _receiver) = get_name(head);

        Ok(String::from(""))
    } else {
        if head.as_str() != "RoleEnd" {
            let (sender, receiver) = get_name(head);

            let result = match sender.as_str() {
                "A" => match receiver.as_str() {
                    "B" => match S1::head().as_str() {
                        // "Send" => String::from("Send"),
                        "Send" =>
                        {

                            //let s = try!(send_mpst_a_to_b(0, s));

                            // let test: Send<_, _> = s.session1;

                            // let new_session = send(0, s.session1);
                            // let new_queue = next_a_to_b(s.queue);

                            format!("{}!{}.{}", sender, receiver, checker_aux(s)?)
                        },
                        "Recv" => String::from("Recv"),
                        "End" => String::from("End"),
                        _ => panic!("Wrong session, not recognized"),
                    },
                    "C" => match S2::head().as_str() {
                        "Send" => String::from("Send"),
                        "Recv" => String::from("Recv"),
                        "End" => String::from("End"),
                        _ => panic!("Wrong session, not recognized"),
                    },
                    _ => panic!("Wrong receiver, not recognized"),
                },
                "B" => match receiver.as_str() {
                    "A" => match S1::head().as_str() {
                        "Send" => String::from("Send"),
                        "Recv" => String::from("Recv"),
                        "End" => String::from("End"),
                        _ => panic!("Wrong session, not recognized"),
                    },
                    "C" => match S2::head().as_str() {
                        "Send" => String::from("Send"),
                        "Recv" => String::from("Recv"),
                        "End" => String::from("End"),
                        _ => panic!("Wrong session, not recognized"),
                    },
                    _ => panic!("Wrong receiver, not recognized"),
                },
                "C" => match receiver.as_str() {
                    "A" => match S1::head().as_str() {
                        "Send" => String::from("Send"),
                        "Recv" => String::from("Recv"),
                        "End" => String::from("End"),
                        _ => panic!("Wrong session, not recognized"),
                    },
                    "B" => match S2::head().as_str() {
                        "Send" => String::from("Send"),
                        "Recv" => String::from("Recv"),
                        "End" => String::from("End"),
                        _ => panic!("Wrong session, not recognized"),
                    },
                    _ => panic!("Wrong receiver, not recognized"),
                },
                _ => panic!("Wrong sender, not recognized"),
            };

            Ok(result)
        } else {
            Ok(String::from(""))
        }
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn parse_type(s: &str) -> String {
    let result = &s.replace("&", "");
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

    String::from(result)
}

fn get_name(head: String) -> (String, String) {
    let (sender, receiver) = match head.as_str() {
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
        _ => panic!("Wrong head, not recognized"),
    };

    (sender, receiver)
}
