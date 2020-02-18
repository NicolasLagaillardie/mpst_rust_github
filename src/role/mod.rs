pub mod a_receives_from_b;
pub mod b_receives_from_a;
pub mod a_receives_from_c;
pub mod c_receives_from_a;
pub mod c_receives_from_b;
pub mod b_receives_from_c;

pub mod a_sends_to_b;
pub mod b_sends_to_a;
pub mod a_sends_to_c;
pub mod c_sends_to_a;
pub mod c_sends_to_b;
pub mod b_sends_to_c;

pub mod end;
use std::marker;

pub trait Role: marker::Sized + marker::Send {
    type Dual: Role<Dual = Self>;

    fn new() -> (Self, Self::Dual);
}
