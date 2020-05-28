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
pub fn checker<S1, S2, S3, R1, R2, R3>(
    s1: SessionMpst<S1, <S3 as Session>::Dual, R1>,
    s2: SessionMpst<<S1 as Session>::Dual, S2, R2>,
    s3: SessionMpst<S3, <S2 as Session>::Dual, R3>,
    hm: &HashMap<String, &Vec<String>>,
) -> Result<(String, String, String), Box<dyn Error>>
where
    S1: Session + 'static,
    S2: Session + 'static,
    S3: Session + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    R3: Role + 'static,
{
    let result_1 = checker_aux(
        &parse_type_of(&s1.session1),
        &parse_type_of(&s1.session2),
        &parse_type_of(&s1.stack),
        &hm,
        &mut vec![],
    )?;
    let result_2 = checker_aux(
        &parse_type_of(&s2.session1),
        &parse_type_of(&s2.session2),
        &parse_type_of(&s2.stack),
        &hm,
        &mut vec![],
    )?;
    let result_3 = checker_aux(
        &parse_type_of(&s3.session1),
        &parse_type_of(&s3.session2),
        &parse_type_of(&s3.stack),
        &hm,
        &mut vec![],
    )?;

    Ok((
        format!("A: {}", &result_1),
        format!("B: {}", &result_2),
        format!("C: {}", &result_3),
    ))
}
