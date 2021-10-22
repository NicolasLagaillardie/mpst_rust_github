use mpstthree::binary::cancel::cancel;
use mpstthree::binary::choose::*;
use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::offer::*;
use mpstthree::binary::recv::recv;
use mpstthree::binary::select::select_mut;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::end::*;
use mpstthree::binary::struct_trait::recv::*;
use mpstthree::binary::struct_trait::send::*;
use mpstthree::binary::struct_trait::session::*;
use mpstthree::choose;
use mpstthree::offer;

use rand::{thread_rng, Rng};
use std::boxed::Box;
use std::error::Error;
use std::marker;
use std::mem;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

// Bug with the constraint checker.
#[allow(dead_code)]
enum CalcOp2<N: marker::Send> {
    More(Send<i64, Recv<i64, NiceCalcServer2<N>>>),
    More2(Recv<i64, Send<i64, NiceCalcServer2<N>>>),
    Stop(Send<i64, End>),
}

#[allow(dead_code)]
type NiceCalcServer2<N> = Recv<CalcOp2<N>, End>;