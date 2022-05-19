#![allow(clippy::type_complexity)]

use ntest::timeout;

mod binary_timed_mod;

#[test]
fn tests_binary_timed() {
    binary_timed_mod::main::head_str();
    binary_timed_mod::main::tail_str();
    binary_timed_mod::main::self_head_str();
    binary_timed_mod::main::self_tail_str();
    binary_timed_mod::main::constraint_start_excluded();
    binary_timed_mod::main::constraint_start_included();
    binary_timed_mod::main::constraint_end_excluded();
    binary_timed_mod::main::constraint_end_included();
    binary_timed_mod::main::constraint_start_excluded_end_excluded();
    binary_timed_mod::main::constraint_start_excluded_end_included();
    binary_timed_mod::main::constraint_start_included_end_excluded();
    binary_timed_mod::main::constraint_start_included_end_included();
    binary_timed_mod::main::simple_calc_works();
}

///////////////////////// send wrong order

#[test]
#[should_panic]
fn send_both_positive_both_included_wrong_order_panics() {
    binary_timed_mod::panicking::send::send_both_positive_both_included_wrong_order_panics();
}

///////////////////////// send both negative

#[test]
#[should_panic]
fn send_both_negative_both_included_panics() {
    binary_timed_mod::panicking::send::send_both_negative_both_included_panics();
}

///////////////////////// send / both positive / both included

#[test]
#[should_panic]
fn send_both_positive_both_included_upper_timeout_panics() {
    binary_timed_mod::panicking::send::send_both_positive_both_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn send_both_positive_both_included_lower_timeout_panics() {
    binary_timed_mod::panicking::send::send_both_positive_both_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn send_both_positive_both_included_reset_clock_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_both_positive_both_included_reset_clock_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_both_positive_both_included_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_both_positive_both_included_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_both_positive_both_included_wrong_reset_panics() {
    binary_timed_mod::panicking::send::send_both_positive_both_included_wrong_reset_panics();
}

///////////////////////// send / both positive / upper included

#[test]
#[should_panic]
fn send_both_positive_upper_included_upper_timeout_panics() {
    binary_timed_mod::panicking::send::send_both_positive_upper_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn send_both_positive_upper_included_lower_timeout_panics() {
    binary_timed_mod::panicking::send::send_both_positive_upper_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn send_both_positive_upper_included_reset_clock_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_both_positive_upper_included_reset_clock_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_both_positive_upper_included_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_both_positive_upper_included_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_both_positive_upper_included_wrong_reset_panics() {
    binary_timed_mod::panicking::send::send_both_positive_upper_included_wrong_reset_panics();
}

///////////////////////// send / both positive / lower included

#[test]
#[should_panic]
fn send_both_positive_lower_included_upper_timeout_panics() {
    binary_timed_mod::panicking::send::send_both_positive_lower_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn send_both_positive_lower_included_lower_timeout_panics() {
    binary_timed_mod::panicking::send::send_both_positive_lower_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn send_both_positive_lower_included_lower_reset_clock_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_both_positive_lower_included_lower_reset_clock_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_both_positive_lower_included_lower_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_both_positive_lower_included_lower_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_both_positive_lower_included_wrong_reset_panics() {
    binary_timed_mod::panicking::send::send_both_positive_lower_included_wrong_reset_panics();
}

///////////////////////// send / both positive / none included

#[test]
#[should_panic]
fn send_both_positive_none_included_upper_timeout_panics() {
    binary_timed_mod::panicking::send::send_both_positive_none_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn send_both_positive_none_included_lower_timeout_panics() {
    binary_timed_mod::panicking::send::send_both_positive_none_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn send_both_positive_none_included_reset_clock_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_both_positive_none_included_reset_clock_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_both_positive_none_included_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_both_positive_none_included_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_both_positive_none_included_wrong_reset_panics() {
    binary_timed_mod::panicking::send::send_both_positive_none_included_wrong_reset_panics();
}

///////////////////////// send / upper positive / both included

#[test]
#[should_panic]
fn send_upper_positive_both_included_upper_timeout_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_both_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_both_included_lower_timeout_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_both_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_both_included_reset_clock_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_both_included_reset_clock_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_both_included_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_both_included_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_both_included_wrong_reset_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_both_included_wrong_reset_panics();
}

///////////////////////// send / both positive / upper included

#[test]
#[should_panic]
fn send_upper_positive_upper_included_upper_timeout_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_upper_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_upper_included_lower_timeout_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_upper_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_upper_included_reset_clock_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_upper_included_reset_clock_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_upper_included_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_upper_included_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_upper_included_wrong_reset_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_upper_included_wrong_reset_panics();
}

///////////////////////// send / both positive / lower included

#[test]
#[should_panic]
fn send_upper_positive_lower_included_upper_timeout_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_lower_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_lower_included_lower_timeout_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_lower_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_lower_included_lower_reset_clock_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_lower_included_lower_reset_clock_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_lower_included_lower_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_lower_included_lower_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_lower_included_wrong_reset_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_lower_included_wrong_reset_panics();
}

///////////////////////// send / both positive / none included

#[test]
#[should_panic]
fn send_upper_positive_none_included_upper_timeout_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_none_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_none_included_lower_timeout_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_none_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_none_included_reset_clock_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_none_included_reset_clock_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_none_included_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_none_included_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_upper_positive_none_included_wrong_reset_panics() {
    binary_timed_mod::panicking::send::send_upper_positive_none_included_wrong_reset_panics();
}

///////////////////////// send / lower positive / both included

#[test]
#[should_panic]
fn send_lower_positive_both_included_upper_timeout_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_both_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_both_included_lower_timeout_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_both_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_both_included_reset_clock_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_both_included_reset_clock_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_both_included_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_both_included_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_both_included_wrong_reset_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_both_included_wrong_reset_panics();
}

///////////////////////// send / both positive / upper included

#[test]
#[should_panic]
fn send_lower_positive_upper_included_upper_timeout_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_upper_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_upper_included_lower_timeout_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_upper_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_upper_included_reset_clock_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_upper_included_reset_clock_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_upper_included_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_upper_included_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_upper_included_wrong_reset_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_upper_included_wrong_reset_panics();
}

///////////////////////// send / both positive / lower included

#[test]
#[should_panic]
fn send_lower_positive_lower_included_upper_timeout_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_lower_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_lower_included_lower_timeout_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_lower_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_lower_included_lower_reset_clock_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_lower_included_lower_reset_clock_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_lower_included_lower_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_lower_included_lower_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_lower_included_wrong_reset_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_lower_included_wrong_reset_panics();
}

///////////////////////// send / both positive / none included

#[test]
#[should_panic]
fn send_lower_positive_none_included_upper_timeout_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_none_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_none_included_lower_timeout_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_none_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_none_included_reset_clock_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_none_included_reset_clock_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_none_included_receive_missing_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_none_included_receive_missing_panics();
}

#[test]
#[should_panic]
fn send_lower_positive_none_included_wrong_reset_panics() {
    binary_timed_mod::panicking::send::send_lower_positive_none_included_wrong_reset_panics();
}

///////////////////////// recv wrong order

#[test]
#[should_panic]
fn recv_both_positive_both_included_wrong_order_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_both_included_wrong_order_panics();
}

///////////////////////// recv both negative

#[test]
#[should_panic]
fn recv_both_negative_both_included_panics() {
    binary_timed_mod::panicking::recv::recv_both_negative_both_included_panics();
}

///////////////////////// recv / both positive / both included

#[test]
#[should_panic]
fn recv_both_positive_both_included_upper_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_both_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn recv_both_positive_both_included_lower_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_both_included_lower_timeout_panics();
}

#[test]
fn recv_both_positive_both_included_reset_clock_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_both_included_reset_clock_send_missing_panics();
}

#[test]
fn recv_both_positive_both_included_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_both_included_send_missing_panics();
}

#[test]
#[should_panic]
fn recv_both_positive_both_included_wrong_reset_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_both_included_wrong_reset_panics();
}

///////////////////////// recv / both positive / upper included

#[test]
#[should_panic]
fn recv_both_positive_upper_included_upper_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_upper_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn recv_both_positive_upper_included_lower_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_upper_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn recv_both_positive_upper_included_reset_clock_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_upper_included_reset_clock_send_missing_panics();
}

#[test]
#[should_panic]
fn recv_both_positive_upper_included_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_upper_included_send_missing_panics();
}

#[test]
#[should_panic]
fn recv_both_positive_upper_included_wrong_reset_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_upper_included_wrong_reset_panics();
}

///////////////////////// recv / both positive / lower included

#[test]
#[should_panic]
fn recv_both_positive_lower_included_upper_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_lower_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn recv_both_positive_lower_included_lower_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_lower_included_lower_timeout_panics();
}

#[test]
fn recv_both_positive_lower_included_lower_reset_clock_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_lower_included_lower_reset_clock_send_missing_panics();
}

#[test]
fn recv_both_positive_lower_included_lower_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_lower_included_lower_send_missing_panics(
    );
}

#[test]
#[should_panic]
fn recv_both_positive_lower_included_wrong_reset_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_lower_included_wrong_reset_panics();
}

///////////////////////// recv / both positive / none included

#[test]
#[should_panic]
fn recv_both_positive_none_included_upper_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_none_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn recv_both_positive_none_included_lower_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_none_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn recv_both_positive_none_included_reset_clock_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_none_included_reset_clock_send_missing_panics();
}

#[test]
#[should_panic]
fn recv_both_positive_none_included_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_none_included_send_missing_panics();
}

#[test]
#[should_panic]
fn recv_both_positive_none_included_wrong_reset_panics() {
    binary_timed_mod::panicking::recv::recv_both_positive_none_included_wrong_reset_panics();
}

///////////////////////// recv / upper positive / both included

#[test]
#[should_panic]
fn recv_upper_positive_both_included_upper_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_both_included_upper_timeout_panics();
}

#[test]
fn recv_upper_positive_both_included_lower_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_both_included_lower_timeout_panics();
}

#[test]
fn recv_upper_positive_both_included_reset_clock_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_both_included_reset_clock_send_missing_panics();
}

#[test]
fn recv_upper_positive_both_included_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_both_included_send_missing_panics();
}

#[test]
#[should_panic]
fn recv_upper_positive_both_included_wrong_reset_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_both_included_wrong_reset_panics();
}

///////////////////////// recv / both positive / upper included

#[test]
#[should_panic]
fn recv_upper_positive_upper_included_upper_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_upper_included_upper_timeout_panics();
}

#[test]
fn recv_upper_positive_upper_included_lower_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_upper_included_lower_timeout_panics();
}

#[test]
fn recv_upper_positive_upper_included_reset_clock_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_upper_included_reset_clock_send_missing_panics();
}

#[test]
fn recv_upper_positive_upper_included_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_upper_included_send_missing_panics();
}

#[test]
#[should_panic]
fn recv_upper_positive_upper_included_wrong_reset_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_upper_included_wrong_reset_panics();
}

///////////////////////// recv / both positive / lower included

#[test]
#[should_panic]
fn recv_upper_positive_lower_included_upper_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_lower_included_upper_timeout_panics();
}

#[test]
fn recv_upper_positive_lower_included_lower_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_lower_included_lower_timeout_panics();
}

#[test]
fn recv_upper_positive_lower_included_lower_reset_clock_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_lower_included_lower_reset_clock_send_missing_panics();
}

#[test]
fn recv_upper_positive_lower_included_lower_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_lower_included_lower_send_missing_panics(
    );
}

#[test]
#[should_panic]
fn recv_upper_positive_lower_included_wrong_reset_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_lower_included_wrong_reset_panics();
}

///////////////////////// recv / both positive / none included

#[test]
#[should_panic]
fn recv_upper_positive_none_included_upper_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_none_included_upper_timeout_panics();
}

#[test]
fn recv_upper_positive_none_included_lower_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_none_included_lower_timeout_panics();
}

#[test]
fn recv_upper_positive_none_included_reset_clock_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_none_included_reset_clock_send_missing_panics();
}

#[test]
fn recv_upper_positive_none_included_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_none_included_send_missing_panics();
}

#[test]
#[should_panic]
fn recv_upper_positive_none_included_wrong_reset_panics() {
    binary_timed_mod::panicking::recv::recv_upper_positive_none_included_wrong_reset_panics();
}

///////////////////////// recv / lower positive / both included

#[test]
#[should_panic]
#[timeout(5000)]
fn recv_lower_positive_both_included_upper_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_both_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn recv_lower_positive_both_included_lower_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_both_included_lower_timeout_panics();
}

#[test]
fn recv_lower_positive_both_included_reset_clock_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_both_included_reset_clock_send_missing_panics();
}

#[test]
fn recv_lower_positive_both_included_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_both_included_send_missing_panics();
}

#[test]
#[should_panic]
fn recv_lower_positive_both_included_wrong_reset_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_both_included_wrong_reset_panics();
}

///////////////////////// recv / both positive / upper included

#[test]
#[should_panic]
#[timeout(5000)]
fn recv_lower_positive_upper_included_upper_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_upper_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn recv_lower_positive_upper_included_lower_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_upper_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn recv_lower_positive_upper_included_reset_clock_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_upper_included_reset_clock_send_missing_panics();
}

#[test]
#[should_panic]
fn recv_lower_positive_upper_included_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_upper_included_send_missing_panics();
}

#[test]
#[should_panic]
fn recv_lower_positive_upper_included_wrong_reset_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_upper_included_wrong_reset_panics();
}

///////////////////////// recv / both positive / lower included

#[test]
#[should_panic]
#[timeout(5000)]
fn recv_lower_positive_lower_included_upper_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_lower_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn recv_lower_positive_lower_included_lower_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_lower_included_lower_timeout_panics();
}

#[test]
fn recv_lower_positive_lower_included_lower_reset_clock_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_lower_included_lower_reset_clock_send_missing_panics();
}

#[test]
fn recv_lower_positive_lower_included_lower_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_lower_included_lower_send_missing_panics(
    );
}

#[test]
#[should_panic]
fn recv_lower_positive_lower_included_wrong_reset_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_lower_included_wrong_reset_panics();
}

///////////////////////// recv / both positive / none included

#[test]
#[should_panic]
#[timeout(5000)]
fn recv_lower_positive_none_included_upper_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_none_included_upper_timeout_panics();
}

#[test]
#[should_panic]
fn recv_lower_positive_none_included_lower_timeout_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_none_included_lower_timeout_panics();
}

#[test]
#[should_panic]
fn recv_lower_positive_none_included_reset_clock_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_none_included_reset_clock_send_missing_panics();
}

#[test]
#[should_panic]
fn recv_lower_positive_none_included_send_missing_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_none_included_send_missing_panics();
}

#[test]
#[should_panic]
fn recv_lower_positive_none_included_wrong_reset_panics() {
    binary_timed_mod::panicking::recv::recv_lower_positive_none_included_wrong_reset_panics();
}

///////////////////////// choose left

#[test]
#[should_panic]
fn choose_left_upper_timeout_panics() {
    binary_timed_mod::panicking::choose::choose_left_upper_timeout_panics();
}

#[test]
#[should_panic]
fn choose_left_lower_timeout_panics() {
    binary_timed_mod::panicking::choose::choose_left_lower_timeout_panics();
}

///////////////////////// choose right

#[test]
#[should_panic]
fn choose_right_upper_timeout_panics() {
    binary_timed_mod::panicking::choose::choose_right_upper_timeout_panics();
}

#[test]
#[should_panic]
fn choose_right_lower_timeout_panics() {
    binary_timed_mod::panicking::choose::choose_right_lower_timeout_panics();
}

///////////////////////// offer

#[test]
#[should_panic]
fn offer_upper_timeout_panics() {
    binary_timed_mod::panicking::offer::offer_upper_timeout_panics();
}

#[test]
#[should_panic]
fn offer_lower_timeout_panics() {
    binary_timed_mod::panicking::offer::offer_lower_timeout_panics();
}
