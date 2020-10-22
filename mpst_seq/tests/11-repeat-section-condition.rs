use mpst_seq::seq;

seq!(N in 0..6 ! 3 {
    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Interrupt {
        %(
            Ira#N:0,
        )(
            Irb#N:0,
        )0*

        #(
            Irc#N:0,
        )0:0

        #(
            Ird#N:0,
        )1:0

        #(
            Ire#N:0,
        )2:0

        #(
            Irf#N:0,
        )3:0

        #(
            Irg#N:0,
        )4:0

        #(
            Irh#N:0,
        )5:0

        #(
            Iri#N:0,
        )6:0

        #(
            Irj#N:0,
        )7:0
    }
});

seq!(N in 1..5 {
    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Interrupt2 {
        %(
            Ir2a#N:0,
            ~(
                Ir2b~N:0,
            )(
                Ir2c~N:1,
            )0*
        )(
            Ir2d#N:0,
            ~(
                Ir2e~N:0,
            )(
                Ir2f~N:1,
            )0*
        )1*
    }
});

fn interrupt_1() {
    let interrupt = Interrupt::Ira2;

    assert_eq!(interrupt as u8, 2);
    assert_eq!(interrupt, Interrupt::Ira2);

    let interrupt = Interrupt::Irb3;

    assert_eq!(interrupt as u8, 3);
    assert_eq!(interrupt, Interrupt::Irb3);

    let interrupt = Interrupt::Irc2;

    assert_eq!(interrupt as u8, 8);
    assert_eq!(interrupt, Interrupt::Irc2);

    let interrupt = Interrupt::Ird2;

    assert_eq!(interrupt as u8, 14);
    assert_eq!(interrupt, Interrupt::Ird2);

    let interrupt = Interrupt::Ire5;

    assert_eq!(interrupt as u8, 32);
    assert_eq!(interrupt, Interrupt::Ire5);

    let interrupt = Interrupt::Irf6;

    assert_eq!(interrupt as u8, 38);
    assert_eq!(interrupt, Interrupt::Irf6);

    let interrupt = Interrupt::Irg6;

    assert_eq!(interrupt as u8, 50);
    assert_eq!(interrupt, Interrupt::Irg6);

    let interrupt = Interrupt::Irh6;

    assert_eq!(interrupt as u8, 92);
    assert_eq!(interrupt, Interrupt::Irh6);

    let interrupt = Interrupt::Iri2;

    assert_eq!(interrupt as u8, 102);
    assert_eq!(interrupt, Interrupt::Iri2);

    let interrupt = Interrupt::Irj1;

    assert_eq!(interrupt as u8, 104);
    assert_eq!(interrupt, Interrupt::Irj1);
}

fn interrupt_2() {
    let interrupt = Interrupt2::Ir2a3;

    assert_eq!(interrupt as u8, 10);
    assert_eq!(interrupt, Interrupt2::Ir2a3);

    let interrupt = Interrupt2::Ir2b2_3;

    assert_eq!(interrupt as u8, 7);
    assert_eq!(interrupt, Interrupt2::Ir2b2_3);

    let interrupt = Interrupt2::Ir2c5_3;

    assert_eq!(interrupt as u8, 23);
    assert_eq!(interrupt, Interrupt2::Ir2c5_3);
}

fn main() {
    interrupt_1();
    interrupt_2();
}
