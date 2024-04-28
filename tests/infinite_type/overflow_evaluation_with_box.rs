use crossbeam_channel::{Receiver, Sender};
use std::marker;
// use std::rc::Rc;

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
    pub channel: Sender<(T, S::Dual)>,
}

#[derive(Debug)]
pub struct Recv<T, S>
where
    T: marker::Send,
    S: Session,
{
    pub channel: Receiver<(T, S)>,
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
    End(Box<End>),
    Branch(Box<Recv<Test0, End>>),
}
enum Test1 {
    End(Box<End>),
    Branch(Box<Recv<i32, Recv<Test1, End>>>),
}
enum Test2 {
    End(Box<End>),
    Branch(Box<Recv<i32, Send<Test2, End>>>),
}
enum Test3 {
    End(Box<End>),
    Branch(Box<Send<i32, Recv<Test3, End>>>),
}
enum Test4 {
    End(Box<End>),
    Branch(Box<Send<i32, Send<Test4, End>>>),
}
enum Test5 {
    End(Box<End>),
    Branch(Box<Recv<i32, Recv<i32, Recv<Test5, End>>>>),
}
enum Test6 {
    End(Box<End>),
    Branch(Box<Recv<i32, Recv<i32, Send<Test6, End>>>>),
}
enum Test7 {
    End(Box<End>),
    Branch(Box<Recv<i32, Send<i32, Recv<Test7, End>>>>),
}
enum Test8 {
    End(Box<End>),
    Branch(Box<Recv<i32, Send<i32, Send<Test8, End>>>>),
}
enum Test9 {
    End(Box<End>),
    Branch(Box<Send<i32, Recv<i32, Recv<Test9, End>>>>),
}
enum Test10 {
    End(Box<End>),
    Branch(Box<Send<i32, Recv<i32, Send<Test10, End>>>>),
}
enum Test11 {
    End(Box<End>),
    Branch(Box<Send<i32, Send<i32, Recv<Test11, End>>>>),
}
enum Test12 {
    End(Box<End>),
    Branch(Box<Send<i32, Send<i32, Send<Test12, End>>>>),
}

fn main() {}
