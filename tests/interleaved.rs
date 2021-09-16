mod interleaved_mod;

// Tests for macros with more than two participants
#[test]
fn macro_basics() {
    // Macro multi send-recv
    interleaved_mod::macro_multi_send_recv_meshedchannels::interleaved_main();

    // Macro multi recursion
    interleaved_mod::macro_multi_recursion::interleaved_main();

    // Macro recursive
    interleaved_mod::macro_recursive::interleaved_main();
}
