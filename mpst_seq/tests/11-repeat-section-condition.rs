use mpst_seq::seq;

seq!(N in 0..6 ! 3 {
    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Interrupt {
        ~(
            Ira#N,
        )(
            Irb#N,
        )*
        
        #(
            Irc#N,
        )0:0
        
        #(
            Ird#N,
        )1:0
        
        #(
            Ire#N,
        )2:0
        
        #(
            Irf#N,
        )3:0
        
        #(
            Irg#N,
        )4:0
        
        #(
            Irh#N,
        )5:0
    }
});


seq!(N in 1..5 {
    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Interrupt2 {
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
}

fn interrupt_2() {
    let interrupt = Interrupt2::Irs3;

    assert_eq!(interrupt as u8, 10);
    assert_eq!(interrupt, Interrupt2::Irs3);
    
    let interrupt = Interrupt2::Irq2_3;

    assert_eq!(interrupt as u8, 7);
    assert_eq!(interrupt, Interrupt2::Irq2_3);
    
    let interrupt = Interrupt2::Irk5_3;

    assert_eq!(interrupt as u8, 23);
    assert_eq!(interrupt, Interrupt2::Irk5_3);

}

fn main() {
    interrupt_1();
    interrupt_2();
}
