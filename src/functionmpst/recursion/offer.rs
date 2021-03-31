/// Offer a choice to A from C between many different
/// sessions wrapped in an `enum`
///
/// # Arguments
///
/// * The session to be used
/// * Each path, which are each variant of the enum which contains the new branches
/// * The block of code to process each new session
///
/// # Example
///
/// ```ignore
/// offer_mpst_a_to_c!(s, {
///    CBranchesAtoC::End(s) => {
///        close_mpst(s)
///    },
///    CBranchesAtoC::Video(s) => {
///        let (request, s) = recv_mpst_a_from_c(s)?;
///        let s = send_mpst_a_to_b(request + 1, s);
///        let (video, s) = recv_mpst_a_from_b(s)?;
///        let s = send_mpst_a_to_c(video + 1, s);
///        authenticator_recurs(s)
///    },
/// })?;
/// ```
#[macro_export]
macro_rules! offer_mpst_a_to_c {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {{

        use mpstthree::functionmpst::recv::recv_mpst_a_from_c;

        mpstthree::offer_mpst!($session, recv_mpst_a_from_c, { $($pat => $result,)* })
    }};
}

/// Offer a choice to B from C between many different
/// sessions wrapped in an `enum`
///
/// # Arguments
///
/// * The session to be used
/// * Each path, which are each variant of the enum which contains the new branches
/// * The block of code to process each new session
///
/// # Example
///
/// ```ignore
/// offer_mpst_b_to_c!(s, {
///    CBranchesBtoC::End(s) => {
///        close_mpst(s)
///    },
///    CBranchesBtoC::Video(s) => {
///        let (request, s) = recv_mpst_b_from_c(s)?;
///        let s = send_mpst_b_to_a(request + 1, s);
///        let (video, s) = recv_mpst_b_from_a(s)?;
///        let s = send_mpst_b_to_c(video + 1, s);
///        authenticator_recurs(s)
///    },
/// })?;
/// ```
#[macro_export]
macro_rules! offer_mpst_b_to_c {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {{

        use mpstthree::functionmpst::recv::recv_mpst_b_from_c;

        mpstthree::offer_mpst!($session, recv_mpst_b_from_c, { $($pat => $result,)* })
    }};
}

/// Offer a choice to A from B between many different
/// sessions wrapped in an `enum`
///
/// # Arguments
///
/// * The session to be used
/// * Each path, which are each variant of the enum which contains the new branches
/// * The block of code to process each new session
///
/// # Example
///
/// ```ignore
/// offer_mpst_a_to_b!(s, {
///    CBranchesAtoB::End(s) => {
///        close_mpst(s)
///    },
///    CBranchesAtoB::Video(s) => {
///        let (request, s) = recv_mpst_a_from_c(s)?;
///        let s = send_mpst_a_to_b(request + 1, s);
///        let (video, s) = recv_mpst_a_from_b(s)?;
///        let s = send_mpst_a_to_c(video + 1, s);
///        authenticator_recurs(s)
///    },
/// })?;
/// ```
#[macro_export]
macro_rules! offer_mpst_a_to_b {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {{

        use mpstthree::functionmpst::recv::recv_mpst_a_from_b;

        mpstthree::offer_mpst!($session, recv_mpst_a_from_b, { $($pat => $result,)* })

        // (move || -> Result<_, Box<dyn std::error::Error>> {
        //     let (l, s) = mpstthree::functionmpst::recv::recv_mpst_a_from_b($session)?;
        //     mpstthree::binary::cancel(s);
        //     match l {
        //         $(
        //             $pat => { $result },
        //         )*
        //     }
        // })()
    }};
}

/// Offer a choice to B from A between many different
/// sessions wrapped in an `enum`
///
/// # Arguments
///
/// * The session to be used
/// * Each path, which are each variant of the enum which contains the new branches
/// * The block of code to process each new session
///
/// # Example
///
/// ```ignore
/// offer_mpst_b_to_a!(s, {
///    CBranchesBtoA::End(s) => {
///        close_mpst(s)
///    },
///    CBranchesBtoA::Video(s) => {
///        let (request, s) = recv_mpst_b_from_c(s)?;
///        let s = send_mpst_b_to_a(request + 1, s);
///        let (video, s) = recv_mpst_b_from_a(s)?;
///        let s = send_mpst_b_to_c(video + 1, s);
///        authenticator_recurs(s)
///    },
/// })?;
/// ```
#[macro_export]
macro_rules! offer_mpst_b_to_a {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {{

        use mpstthree::functionmpst::recv::recv_mpst_b_from_a;

        mpstthree::offer_mpst!($session, recv_mpst_b_from_a, { $($pat => $result,)* })
    }};
}

/// Offer a choice to C from B between many different
/// sessions wrapped in an `enum`
///
/// # Arguments
///
/// * The session to be used
/// * Each path, which are each variant of the enum which contains the new branches
/// * The block of code to process each new session
///
/// # Example
///
/// ```ignore
/// offer_mpst_c_to_b!(s, {
///    CBranchesCtoB::End(s) => {
///        close_mpst(s)
///    },
///    CBranchesCtoB::Video(s) => {
///        let (request, s) = recv_mpst_c_from_b(s)?;
///        let s = send_mpst_c_to_a(request + 1, s);
///        let (video, s) = recv_mpst_c_from_a(s)?;
///        let s = send_mpst_c_to_b(video + 1, s);
///        authenticator_recurs(s)
///    },
/// })?;
/// ```
#[macro_export]
macro_rules! offer_mpst_c_to_b {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {{

        use mpstthree::functionmpst::recv::recv_mpst_c_from_b;

        mpstthree::offer_mpst!($session, recv_mpst_c_from_b, { $($pat => $result,)* })
    }};
}

/// Offer a choice to C from A between many different
/// sessions wrapped in an `enum`
///
/// # Arguments
///
/// * The session to be used
/// * Each path, which are each variant of the enum which contains the new branches
/// * The block of code to process each new session
///
/// # Example
///
/// ```ignore
/// offer_mpst_c_to_a!(s, {
///    CBranchesCtoA::End(s) => {
///        close_mpst(s)
///    },
///    CBranchesCtoA::Video(s) => {
///        let (request, s) = recv_mpst_c_from_a(s)?;
///        let s = send_mpst_c_to_a(request + 1, s);
///        let (video, s) = recv_mpst_c_from_a(s)?;
///        let s = send_mpst_c_to_a(video + 1, s);
///        authenticator_recurs(s)
///    },
/// })?;
/// ```
#[macro_export]
macro_rules! offer_mpst_c_to_a {
    ($session:expr, { $($pat:pat => $result:block,)* }) => {{

        use mpstthree::functionmpst::recv::recv_mpst_c_from_a;

        mpstthree::offer_mpst!($session, recv_mpst_c_from_a, { $($pat => $result,)* })
    }};
}
