////////////////////////////////////////////
/// CHOICE

/// Cancels a session
#[macro_export]
macro_rules! send_cancel {
    ($func_name:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal, $msg:expr) => {
        mpst_seq::seq!(N in 1..$nsessions ! 1 {
            fn $func_name<%(S#N:0,)()0* R>(
                s: $sessionmpst_name<
                    %(
                        S#N:0,
                    )(
                        End,
                    )0*
                    R,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> std::result::Result<(), Box<dyn std::error::Error>>
            where
                %(
                    S#N:0: mpstthree::binary::struct_trait::Session,
                )(
                )0*
                R: mpstthree::role::Role,
            {
                s.session1.sender.send(mpstthree::binary::struct_trait::Signal::Cancel).unwrap();;
                mpstthree::binary::cancel::cancel(s);
                panic!("{:?}", $msg);
            }
        });
    };
}

/// Broadcast a session from the first participant to
/// others. Creates the function that will be direcly sent
#[macro_export]
macro_rules! broadcast_cancel {
    ($session:expr, $name:ident, $sessionmpst_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
                #(
                    let mut bool_session#N:0 = true;
                )0:0

                let mut s = $session;

                while #(bool_session#N:0 ||)0:0 false {
                    #(
                        match s.session#N:0.receiver.try_recv() {
                            Ok(mpstthree::binary::struct_trait::Signal::Cancel) => {
                                #(
                                    s.session#N:0.sender.send(mpstthree::binary::struct_trait::Signal::Cancel).unwrap_or(());
                                )0:0
                                mpstthree::binary::cancel::cancel(s);
                                panic!("Error");
                            }
                            Ok(mpstthree::binary::struct_trait::Signal::Stop) => match bool_session#N:0 {
                                true => {
                                    s.session#N:0.sender.send(mpstthree::binary::struct_trait::Signal::Stop).unwrap_or(());
                                    bool_session#N:0 = false;
                                }
                                false => panic!("Close already sent"),
                            }
                            Ok(mpstthree::binary::struct_trait::Signal::Offer(channel)) => {
                                s.session#N:0 = channel;
                            }
                            _ => {}
                        };
                    )0:0
                }
        });
    }
}
