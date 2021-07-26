//! This module is a beta feature that only works with
//! RoleA, RoleB and RoleC. It also has various
//! prerequisites that are shown in tests 06.

mod aux_checker;
pub mod main;
pub mod new_test;

use self::aux_checker::{checker_aux, parse_type_of};

use crate::binary::struct_trait::session::Session;
use crate::meshedchannels::MeshedChannels;
use crate::role::Role;

use std::collections::HashMap;
use std::error::Error;

/// Displays the local endpoints of each roles.
/// It is required that the `MeshedChannels` are the root ones,
/// and not a partial part included in a bigger one. It is
/// useful for checking whether the implemented local
/// endpoints are the expected ones.
///
/// Returns the 3 strings if everything went right.
/// TODO: Adapt checker for RoleBroadcast
/// Useful???
pub fn checker<
    S0,
    S1,
    S2,
    R1,
    R2,
    R3,
    N1,
    N2,
    N3,
    BH1: ::std::hash::BuildHasher,
    BH2: ::std::hash::BuildHasher,
>(
    s1: MeshedChannels<S0, <S2 as Session>::Dual, R1, N1>,
    s2: MeshedChannels<<S0 as Session>::Dual, S1, R2, N2>,
    s3: MeshedChannels<S2, <S1 as Session>::Dual, R3, N3>,
    branches_receivers: &HashMap<String, &Vec<String>, BH1>,
    branches_sender: &HashMap<String, &Vec<String>, BH2>,
) -> Result<(String, String, String), Box<dyn Error>>
where
    S0: Session + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    R3: Role + 'static,
    N1: Role + 'static,
    N2: Role + 'static,
    N3: Role + 'static,
{
    let result_1 = checker_aux(
        [
            &parse_type_of(&s1.session1),
            &parse_type_of(&s1.session2),
            &parse_type_of(&s1.stack),
            &parse_type_of(&s1.name),
        ],
        "A",
        (branches_receivers, branches_sender),
        &mut vec![],
    )?;
    // println!("result A: {}", &result_1);
    let result_2 = checker_aux(
        [
            &parse_type_of(&s2.session1),
            &parse_type_of(&s2.session2),
            &parse_type_of(&s2.stack),
            &parse_type_of(&s2.name),
        ],
        "B",
        (branches_receivers, branches_sender),
        &mut vec![],
    )?;
    // println!("result B: {}", &result_2);
    let result_3 = checker_aux(
        [
            &parse_type_of(&s3.session1),
            &parse_type_of(&s3.session2),
            &parse_type_of(&s3.stack),
            &parse_type_of(&s3.name),
        ],
        "C",
        (branches_receivers, branches_sender),
        &mut vec![],
    )?;
    // println!("result C: {}", &result_3);

    Ok((
        format!("A: {}", &result_1),
        format!("B: {}", &result_2),
        format!("C: {}", &result_3),
    ))
}

/// macro to create hashmap function, necessary for
/// recursion. Need to sort out the path
#[macro_export]
#[doc(hidden)]
macro_rules! checker_hashmaps {
    // ($( $branch: ty, $func: ident, $branch_type: expr, { $( $pat: path, $branch_name: expr, $label: path, )* }, )*) => {
        ({ $( $branch: path, $func: ident, { $( $pat: path, )* }, )* }) => {

        let mut hm: HashMap<String, &Vec<String>> = HashMap::new();

        fn type_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        $(
            impl<N: marker::Send> fmt::Display for $branch<N> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    match self {
                        $(
                            $pat(s) => write!(f, stringify!($pat), type_of(&s)),
                        )*
                    }
                }
            }

            fn $func() -> Vec<String> {
                let vec = Vec::new();

                $(
                    let (s, _) = <_ as Session>::new();

                    vec.push((&$pat::<i32>(s).to_string()));
                )*

                vec
            }

            hm.insert(String::from(stringify!($branch)), &$func());
        )*

        hm
    };
}
