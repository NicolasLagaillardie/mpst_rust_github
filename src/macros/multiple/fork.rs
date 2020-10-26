////////////////////////////////////////////
/// FORK

#[macro_export]
macro_rules! fork_simple_multi {
    ($func_name: ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(K in 1..$nsessions {
            fn $func_name<#(S#K:0,)0:0 R, N, P>(p: P, s: $struct_name<#(S#K:0,)0:0 R, N>) -> std::thread::JoinHandle<()>
            where
                #(
                    S#K:0: mpstthree::binary::Session + 'static,
                )0:0
                R: mpstthree::role::Role + 'static,
                N: mpstthree::role::Role + 'static,
                P: FnOnce($struct_name<#(S#K:0,)0:0 R, N>) -> Result<(), Box<dyn std::error::Error>> + std::marker::Send + 'static,
            {
                std::thread::spawn(move || {
                    std::panic::set_hook(Box::new(|_info| {
                        // do nothing
                    }));
                    match p(s) {
                        Ok(()) => (),
                        Err(e) => panic!("{:?}", e),
                    }
                })
            }
        });
    }
}

/// TODO
#[macro_export]
macro_rules! fork_mpst_multi {
    ($func_name: ident, $fork_function: ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(K in 1..=$nsessions {
            fn $func_name<#(S#K:0,)14:0 #(R#K:0,)0:0 #(N#K:0,)0:0 #(F#K:0,)0:0>(
                #(
                    f#K:0: F#K:0,
                )0:0
            ) -> (
                #(
                    Result<(), Box<(dyn std::any::Any + std::marker::Send + 'static)>>,
                )0:0
            )
            where
                #(
                    S#K:0: mpstthree::binary::Session + 'static,
                    R#K:0: mpstthree::role::Role + 'static,
                    N#K:0: mpstthree::role::Role + 'static,
                )0:0

                #( // i in 1..K
                    F#K:0: FnOnce($struct_name<
                        ~( // j in 0..K
                            S~K:6, // S(i + j) (with Dual if needed)
                        )(
                            <S~K:6 as mpstthree::binary::Session>::Dual,
                        )4*
                        R#K:0, N#K:0>) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
                )0:0
            {
                #( // i in 1..(diff * (diff + 1))
                    let (channel_#K:12, channel_#K:13) = S#K:0::new(); // channel_(get from matrix), channel_(opposite get from matrix) = S(i)
                )14:0

                #(
                    let (role_#K:0, _) = R#K:0::new();
                    let (name_#K:0, _) = N#K:0::new();
                )0:0

                #(
                    let sessionmpst_#K:0 = $struct_name {
                        ~(
                            session#K:1 : channel_~K:5,
                        )(
                            session#K:1 : channel_~K:5,
                        )4*
                        stack: role_#K:0,
                        name: name_#K:0,
                    };
                )0:0

                #(
                    let thread_#K:0 = $fork_function(f#K:0, sessionmpst_#K:0);
                )0:0

                (
                    #(
                        thread_#K:0.join(),
                    )0:0
                )
            }
        });
    }
}
