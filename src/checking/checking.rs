use binary::Session;
use role::Role;
use sessionmpst::SessionMpst;

use std::collections::HashMap;
use std::error::Error;

/// Displays the local endpoints of each roles.
/// It is required that the `SessionMpst` are the root ones, and not a partial part included in a bigger one.
///
/// Returns unit if everything wen right.
pub fn checker<S1, S2, S3, S4, S5, R1, R2, R3, R4>(
    s1: SessionMpst<S1, <S3 as Session>::Dual, R1>,
    s2: SessionMpst<<S1 as Session>::Dual, S2, R2>,
    s3: SessionMpst<S3, <S2 as Session>::Dual, R3>,
) -> Result<(), Box<dyn Error>>
where
    S1: Session + 'static,
    S2: Session + 'static,
    S3: Session + 'static,
    S4: Session + 'static,
    S5: Session + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    R3: Role + 'static,
    R4: Role + 'static,
{
    let mut result = HashMap::new();

    let result_1 = checker_aux(s1)?;
    let result_2 = checker_aux(s2)?;
    let result_3 = checker_aux(s3)?;

    result.insert("A".to_string(), result_1);
    result.insert("B".to_string(), result_2);
    result.insert("C".to_string(), result_3);

    Ok(())
}

fn checker_aux<S1, S2, R, T>(s: SessionMpst<S1, S2, R>) -> Result<(), Box<dyn Error>>
where
    S1: Session + 'static,
    S2: Session + 'static,
    R: Role + 'static,
{
    let head = R::head();

    println!("Head: {:?}", head);

    Ok(())
}