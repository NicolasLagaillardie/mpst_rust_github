// This test case should hopefully be a freebie if all of the previous ones are
// passing. This test demonstrates using the seq macro to construct a const
// array literal.
//
// The generated code would be:
//
//     [Proc::new(0), Proc::new(1), ..., Proc::new(255),]

use mpst_seq::eseq;

const PROCS: [Proc; 256] = {
    eseq!(N in 0..256 {
        [
            #(
                Proc::new(N),
            )0:0
        ]
    })
};

struct Proc {
    id: usize,
}

impl Proc {
    const fn new(id: usize) -> Self {
        Proc { id }
    }
}

fn main() {
    assert_eq!(PROCS[32].id, 32);
}
