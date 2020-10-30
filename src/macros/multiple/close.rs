////////////////////////////////////////////
/// CLOSE

#[macro_export]
macro_rules! close_mpst {
    ($func_name:ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            fn $func_name<R>(s: $struct_name<#(mpstthree::binary::End,)0:0 mpstthree::role::end::RoleEnd, R>) -> Result<(), Box<dyn Error>>
            where
                R: mpstthree::role::Role,
            {
                #(
                    s.session#N:0.sender.send(()).unwrap_or(());
                )0:0

                #(
                    s.session#N:0.receiver.recv()?;
                )0:0

                Ok(())
            }
        });
    }
}
