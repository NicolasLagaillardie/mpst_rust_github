mod checking;

#[test]
pub fn kmc_basic() {
    checking::basics::checking_simple::main();
    checking::basics::checking_choice::main();
    checking::basics::checking_recursion::main();
}

#[test]
pub fn kmc_complex() {
    checking::complex::commit_protocol::main();
    checking::complex::async_paper_ext_rev_sync::main();
    checking::complex::two_peers_branchings_sync::main();
    checking::complex::four_players_game_sync::main();
    checking::complex::alternating_bit::main();
    checking::complex::inf_snd_rcv::main();
}

#[test]
pub fn main() {
    assert!(true);
}
