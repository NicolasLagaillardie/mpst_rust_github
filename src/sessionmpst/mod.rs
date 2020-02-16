pub mod a_to_b;
pub mod a_to_c;
pub mod b_to_a;
pub mod b_to_c;
pub mod c_to_a;
pub mod c_to_b;
pub mod end;

use role::Role;
use binary::Session;

pub struct SessionMpst<S1: Session, S2: Session, R: Role> {
    pub session1: S1,
    pub session2: S2,
    pub queue: R,
}
