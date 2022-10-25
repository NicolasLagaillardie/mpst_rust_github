#![allow(dead_code)]

use anyhow::{anyhow, Result};
use crossbeam_channel::{select, unbounded, Receiver, Sender};
use std::{marker::PhantomData, thread::spawn};

struct Channel<To> {
    pub rx: Receiver<u32>,
    pub tx: Sender<u32>,
    _ph: PhantomData<To>,
}

fn mk_chan<A, B>() -> (Channel<B>, Channel<A>) {
    let (tx_to, rx_to) = unbounded();
    let (tx_fro, rx_fro) = unbounded();
    (
        Channel {
            rx: rx_fro,
            tx: tx_to,
            _ph: PhantomData,
        },
        Channel {
            rx: rx_to,
            tx: tx_fro,
            _ph: PhantomData,
        },
    )
}

struct Tx<Role, Cont> {
    cont: Cont,
    _ph: PhantomData<Role>,
}

impl<Role, Cont: Default> Default for Tx<Role, Cont> {
    fn default() -> Self {
        Self {
            cont: Cont::default(),
            _ph: PhantomData,
        }
    }
}

struct Rx<Role, Cont> {
    cont: Cont,
    _ph: PhantomData<Role>,
}

impl<Role, Cont: Default> Default for Rx<Role, Cont> {
    fn default() -> Self {
        Self {
            cont: Cont::default(),
            _ph: PhantomData,
        }
    }
}

#[derive(Default)]
struct End;

impl<Role, Cont> Tx<Role, Cont> {
    pub fn send(self, channel: &Channel<Role>, value: u32) -> Result<Cont> {
        channel.tx.send(value)?;
        Ok(self.cont)
    }
}
fn tx<R, C>(_role: R, cont: C) -> Tx<R, C> {
    Tx {
        cont,
        _ph: PhantomData,
    }
}

impl<Role, Cont> Rx<Role, Cont> {
    pub fn recv(self, channel: &Channel<Role>) -> Result<(u32, Cont)> {
        let ret = channel.rx.recv()?;
        Ok((ret, self.cont))
    }
}
fn rx<R, C>(_role: R, cont: C) -> Rx<R, C> {
    Rx {
        cont,
        _ph: PhantomData,
    }
}

// The local code can use either left or right, which will
// in any case consume the Choice.
struct MyChoice<C1, C2> {
    left: C1,
    right: C2,
}

// This node allows receiving from either R1 or R2,
// depending on an external choice
struct TheirChoice<R1, C1, R2, C2> {
    left: C1,
    right: C2,
    _ph: PhantomData<(R1, R2)>,
}

enum ChoiceResult<C1, C2> {
    Left(u32, C1),
    Right(u32, C2),
}

impl<R1, C1, R2, C2> TheirChoice<R1, C1, R2, C2> {
    pub fn recv(self, left: Channel<R1>, right: Channel<R2>) -> Result<ChoiceResult<C1, C2>> {
        select! {
            recv(left.rx) -> msg => Ok(ChoiceResult::Left(msg?, self.left)),
            recv(right.rx) -> msg => Ok(ChoiceResult::Right(msg?, self.right)),
        }
    }
}

macro_rules! rec {
    ($x: ident, $t: ty) => {{
        #[derive(Default)]
        struct $x;
        impl $x {
            pub fn rec(self) -> $t {
                Default::default()
            }
        }
        $x::default().rec()
    }};
}

type RecRoleB<Rec> = Rx<RoleB, fn() -> Rec>;

fn rec_test(ch_a: Channel<RoleA>, ch_b: Channel<RoleB>) -> Result<()> {
    let mut p = rec!(X, Tx<RoleA, Rx<RoleB, X>>);
    // the above produces the same code as below, but
    // rust-analyzer cannot use the resulting types, so it
    // is useless
    let mut _p = {
        struct Rec {
            pub rec: Tx<RoleA, RecRoleB<Rec>>,
        }
        fn new() -> Rec {
            Rec {
                rec: tx(RoleA, rx(RoleB, new)),
            }
        }
        new().rec
    };
    loop {
        let p2 = p.send(&ch_a, 42)?;
        let (v, p2) = p2.recv(&ch_b)?;
        println!("received {v:?}");
        p = p2.rec();
    }
}

struct RoleA;
struct RoleB;
struct RoleC;

fn fancy_building_block<Cont>(
    to_a: &Channel<RoleA>,
    to_b: &Channel<RoleB>,
    p: Tx<RoleA, Rx<RoleB, Cont>>,
) -> Result<Cont> {
    Ok(p.send(to_a, 55)?.recv(to_b)?.1)
}

fn main() -> Result<()> {
    let prot_a = tx(RoleB, rx(RoleC, End));
    let prot_b = rx(RoleA, tx(RoleC, End));
    let prot_c = rx(RoleB, tx(RoleA, End));

    let (ch_ab, ch_ba) = mk_chan::<RoleA, RoleB>();
    let (ch_ac, ch_ca) = mk_chan::<RoleA, RoleC>();
    let (ch_bc, ch_cb) = mk_chan::<RoleB, RoleC>();

    let thread_a = spawn(move || prot_a.send(&ch_ab, 42)?.recv(&ch_ac));
    let thread_b = spawn(move || {
        let (value, p) = prot_b.recv(&ch_ba)?;
        p.send(&ch_bc, value)
    });
    let thread_c = spawn(move || {
        let (value, p) = prot_c.recv(&ch_cb)?;
        p.send(&ch_ca, value)
    });

    let (x, _end): (u32, End) = thread_a.join().map_err(|x| anyhow!("{:?}", x))??;
    let _end: End = thread_b.join().map_err(|x| anyhow!("{:?}", x))??;
    let _end: End = thread_c.join().map_err(|x| anyhow!("{:?}", x))??;

    println!("result: {x:?}");

    Ok(())
}
