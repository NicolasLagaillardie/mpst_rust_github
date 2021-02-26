////////////////////////////////////////////
/// CHOICE

/// Cancels a session
#[macro_export]
macro_rules! send_cancel {
    ($func_name:ident, $name:ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! 1 {
            fn $func_name<%(S#N:0,)()0* R>(
                s: $struct_name<
                    %(
                        S#N:0,
                    )(
                        End,
                    )0*
                    R,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            )
            where
                %(
                    S#N:0: mpstthree::binary::Session,
                )(
                )0*
                R: mpstthree::role::Role,
            {
                s.session1.sender.send(mpstthree::binary::Signal::Cancel).unwrap();;
                std::mem::drop(s);
            }
        });
    }
}

/// Broadcast a session from the first participant to
/// others. Creates the function that will be direcly sent
#[macro_export]
macro_rules! broadcast_cancel {
    ($session:expr, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
                #(
                    let mut session#N:0 = true;
                )0:0

                while #(session#N:0 ||)0:0 false {
                    #(
                        match $session.session#N:0.receiver.try_recv() {
                            Ok(mpstthree::binary::Signal::Cancel) => {
                                #(
                                    $session.session#N:0.sender.send(mpstthree::binary::Signal::Cancel).unwrap_or(());
                                )0:0
                                panic!("Error");
                            }
                            Ok(mpstthree::binary::Signal::Stop) => match session#N:0 {
                                true => {
                                    $session.session#N:0.sender.send(mpstthree::binary::Signal::Stop).unwrap_or(());
                                    session#N:0 = false;
                                }
                                false => panic!("Close already sent"),
                            },
                            _ => {}
                        };
                    )0:0
                }
        });
    }
}
