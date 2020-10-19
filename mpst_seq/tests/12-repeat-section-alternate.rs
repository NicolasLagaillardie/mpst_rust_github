use mpst_seq::seq;

seq!(N in 1..5 {
    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Interrupt {
        ^(
            Irs#N,
            ^(
                Irq~N,
            )(
                Irk^N,
            )*
        )(
            Irt#N,
            ^(
                Irq^N,
            )(
                Irk^N,
            )*
        )~
    }
});

fn main() {
    let interrupt = Interrupt::Irs3;

    assert_eq!(interrupt as u8, 10);
    assert_eq!(interrupt, Interrupt::Irs3);
    
    let interrupt = Interrupt::Irq2_3;

    assert_eq!(interrupt as u8, 7);
    assert_eq!(interrupt, Interrupt::Irq2_3);
    
    let interrupt = Interrupt::Irk5_3;

    assert_eq!(interrupt as u8, 23);
    assert_eq!(interrupt, Interrupt::Irk5_3);
}
