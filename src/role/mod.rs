pub mod a_to_all;
pub mod a_to_b;
pub mod a_to_c;
pub mod all_to_a;
pub mod all_to_b;
pub mod all_to_c;
pub mod b_to_a;
pub mod b_to_all;
pub mod b_to_c;
pub mod c_to_a;
pub mod c_to_all;
pub mod c_to_b;
pub mod recurs;
pub mod end;
use std::marker;

pub trait Role: marker::Sized + marker::Send {
    type Dual: Role<Dual = Self>;

    fn new() -> (Self, Self::Dual);
}

