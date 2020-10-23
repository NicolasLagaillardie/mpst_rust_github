////////////////////////////////////////////
/// FORK

#[macro_export]
macro_rules! fork_simple_multi {
    ($func_name: ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(K in 1..$nsessions {
            fn fork_simple_multi<#(S#K:0,)0:0 R, N, P>(p: P, s: $struct_name<#(S#K:0,)0:0 R, N>) -> std::thread::JoinHandle<()>
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
            fn $func_name<#(S#K:0,)0:0 #(R#K:0,)0:0 #(N#K:0,)0:0 #(F#K:0,)0:0>(
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

                // ^(
                //     F#N:0: FnOnce($struct_name<^(S+N)(<S+N as mpstthree::binary::Session>::Dual)* R1, N1>) -> Result<(), Box<dyn std::error::Error>>
                //     + std::marker::Send
                //     + 'static,
                // )(
                //     F#N:0: FnOnce($struct_name<^(S+N)(<S+N as mpstthree::binary::Session>::Dual)* R1, N1>) -> Result<(), Box<dyn std::error::Error>>
                //     + std::marker::Send
                //     + 'static,
                // )~

                F1: FnOnce($struct_name<S1, S2, R1, N1>) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
                F2: FnOnce($struct_name<<S1 as mpstthree::binary::Session>::Dual, S3, R2, N2>) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
                F3: FnOnce($struct_name<<S2 as mpstthree::binary::Session>::Dual, <S3 as mpstthree::binary::Session>::Dual, R3, N3>) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
            {

                // #(
                //     let ( channel|N, channel%N ) = S#N:0::new();
                // )0:0

                let (channel1_2, channel2_1) = S1::new();
                let (channel1_3, channel3_1) = S2::new();
                let (channel2_3, channel3_2) = S3::new();

                #(
                    let (role_#K:0, _) = R#K:0::new();
                    let (name_#K:0, _) = N#K:0::new();
                )0:0

                let sessionmpst_1 = $struct_name {
                    session1: channel1_2,
                    session2: channel1_3,
                    stack: role_1,
                    name: name_1,
                };
                let sessionmpst_2 = $struct_name {
                    session1: channel2_1,
                    session2: channel2_3,
                    stack: role_2,
                    name: name_2,
                };
                let sessionmpst_3 = $struct_name {
                    session1: channel3_1,
                    session2: channel3_2,
                    stack: role_3,
                    name: name_3,
                };

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
