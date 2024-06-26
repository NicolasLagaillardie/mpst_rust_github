use std::marker;

// Trait
pub trait Session: marker::Sized + marker::Send {
    type Dual: Session<Dual = Self>;
}

// Struct
#[derive(Debug)]
pub struct Send<T, S>
where
    T: marker::Send,
    S: Session,
    S::Dual: Session,
{
    t: T,
    s: S,
}

#[derive(Debug)]
pub struct Recv<T, S>
where
    T: marker::Send,
    S: Session,
{
    t: T,
    s: S,
}

#[derive(Debug)]
pub struct End {}

// impl
impl<T: marker::Send, S: Session> Session for Send<T, S> {
    type Dual = Recv<T, S::Dual>;
}

impl<T: marker::Send, S: Session> Session for Recv<T, S> {
    type Dual = Send<T, S::Dual>;
}

impl Session for End {
    type Dual = End;
}

// enum
enum Test0 {
    End(End),
    Branch(Recv<Test0, End>),
}
enum Test1 {
    End(End),
    Branch(Recv<i32, Recv<Test1, End>>),
}
enum Test2 {
    End(End),
    Branch(Send<i32, Recv<Test2, End>>),
}
enum Test3 {
    End(End),
    Branch(Recv<i32, Send<i32, Recv<Test3, End>>>),
}
enum Test4 {
    End(End),
    Branch(Send<i32, Send<i32, Recv<Test4, End>>>),
}
enum Test5 {
    End(End),
    Branch(Recv<i32, Recv<i32, Recv<Test5, End>>>),
}
enum Test6 {
    End(End),
    Branch(Send<i32, Recv<i32, Recv<Test6, End>>>),
}

fn main() {}
