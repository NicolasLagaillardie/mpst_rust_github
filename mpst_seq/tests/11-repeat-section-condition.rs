use mpst_seq::seq;

seq!(N in 0..16 ! 14 {
    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Interrupt {
        ~(
            Irq#N,
        )(
            Irk#N,
        )*
    }
});

fn main() {
    let interrupt = Interrupt::Irq8;

    assert_eq!(interrupt as u8, 8);
    assert_eq!(interrupt, Interrupt::Irq8);
    
    let interrupt = Interrupt::Irk14;

    assert_eq!(interrupt as u8, 14);
}
