use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::recv::Recv;
use mpstthree::binary::struct_trait::send::Send;

use std::marker;

// Bug with the constraint checker.
enum CalcOp2<N: marker::Send> {
    More(Send<i64, Recv<i64, NiceCalcServer2<N>>>),
    More2(Recv<i64, Send<i64, NiceCalcServer2<N>>>),
    Stop(Send<i64, End>),
}

type NiceCalcServer2<N> = Recv<CalcOp2<N>, End>;

fn main() {
    let _ = NiceCalcServer2<i32>::new();
}
