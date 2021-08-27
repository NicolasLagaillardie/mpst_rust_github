mod checking_mod;
mod graph_mod;

#[test]
fn graph_mod() {
    graph_mod::simple::simple_triple_endpoints();
}

#[test]
pub fn kmc_basic() {
    checking_mod::basics::checking_simple::main();
    checking_mod::basics::checking_choice::main();
    checking_mod::basics::checking_recursion::main();
}

#[test]
pub fn kmc_complex() {
    checking_mod::complex::commit_protocol::main();
    checking_mod::complex::async_paper_ext_rev_sync::main();
    checking_mod::complex::two_peers_branchings_sync::main();
    checking_mod::complex::four_players_game_sync::main();
    checking_mod::complex::alternating_bit::main();
    checking_mod::complex::inf_snd_rcv::main();
}
