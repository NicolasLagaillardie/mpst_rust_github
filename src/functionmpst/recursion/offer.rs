/// Offer a choice at A from C between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! offer_mpst_a_to_c {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {
        (move || -> Result<_, _> {
            let (l, s) = mpstthree::functionmpst::recv::recv_mpst_a_to_c($session)?;
            mpstthree::binary::cancel(s);
            match l {
                $(
                    $pat => { $result },
                )*
            }
        })()
    };
}

/// Offer a choice at B from C between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! offer_mpst_b_to_c {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {
        (move || -> Result<_, _> {
            let (l, s) = mpstthree::functionmpst::recv::recv_mpst_b_to_c($session)?;
            mpstthree::binary::cancel(s);
            match l {
                $(
                    $pat => { $result },
                )*
            }
        })()
    };
}

/// Offer a choice at A from B between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! offer_mpst_a_to_b {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {
        (move || -> Result<_, _> {
            let (l, s) = mpstthree::functionmpst::recv::recv_mpst_a_to_b($session)?;
            mpstthree::binary::cancel(s);
            match l {
                $(
                    $pat => { $result },
                )*
            }
        })()
    };
}

/// Offer a choice at B from A between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! offer_mpst_b_to_a {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {
        (move || -> Result<_, _> {
            let (l, s) = mpstthree::functionmpst::recv::recv_mpst_b_to_a($session)?;
            mpstthree::binary::cancel(s);
            match l {
                $(
                    $pat => { $result },
                )*
            }
        })()
    };
}

/// Offer a choice at C from B between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! offer_mpst_c_to_b {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {
        (move || -> Result<_, _> {
            let (l, s) = mpstthree::functionmpst::recv::recv_mpst_c_to_b($session)?;
            mpstthree::binary::cancel(s);
            match l {
                $(
                    $pat => { $result },
                )*
            }
        })()
    };
}

/// Offer a choice at C from A between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! offer_mpst_c_to_a {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {
        (move || -> Result<_, _> {
            let (l, s) = mpstthree::functionmpst::recv::recv_mpst_c_to_a($session)?;
            mpstthree::binary::cancel(s);
            match l {
                $(
                    $pat => { $result },
                )*
            }
        })()
    };
}
