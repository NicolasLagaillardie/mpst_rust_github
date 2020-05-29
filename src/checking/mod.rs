mod aux_checker;
mod aux_dot;

use self::aux_checker::{checker_aux, parse_type_of};

use binary::Session;
use role::Role;
use sessionmpst::SessionMpst;

use std::collections::HashMap;
use std::error::Error;

/// Displays the local endpoints of each roles.
/// It is required that the `SessionMpst` are the root ones, and not a partial part included in a bigger one.
///
/// Returns the 3 strings if everything went right.
pub fn checker<S0, S1, S2, R1, R2, R3>(
    s1: SessionMpst<S0, <S2 as Session>::Dual, R1>,
    s2: SessionMpst<<S0 as Session>::Dual, S1, R2>,
    s3: SessionMpst<S2, <S1 as Session>::Dual, R3>,
    // s1: SessionMpst<S0, S1, R1>,
    // s2: SessionMpst<S2, S3, R2>,
    // s3: SessionMpst<S4, S5, R3>,
    hm: &HashMap<String, &Vec<String>>,
) -> Result<(String, String, String), Box<dyn Error>>
where
    S0: Session + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    // S3: Session + 'static,
    // S4: Session + 'static,
    // S5: Session + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    R3: Role + 'static,
{
    // println!("Type of s1: {}", parse_type_of(&s1));
    // println!("Type of s2: {}", parse_type_of(&s2));
    // println!("Type of s3: {}", parse_type_of(&s3));

    let result_1 = checker_aux(
        [
            &parse_type_of(&s1.session1),
            &parse_type_of(&s1.session2),
            &parse_type_of(&s1.stack),
        ],
        "A",
        &hm,
        &mut vec![],
    )?;
    let result_2 = checker_aux(
        [
            &parse_type_of(&s2.session1),
            &parse_type_of(&s2.session2),
            &parse_type_of(&s2.stack),
        ],
        "B",
        &hm,
        &mut vec![],
    )?;
    let result_3 = checker_aux(
        [
            &parse_type_of(&s3.session1),
            &parse_type_of(&s3.session2),
            &parse_type_of(&s3.stack),
        ],
        "C",
        &hm,
        &mut vec![],
    )?;

    Ok((
        format!("A: {}", &result_1),
        format!("B: {}", &result_2),
        format!("C: {}", &result_3),
    ))
}
