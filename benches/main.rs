use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    // //////////
    // benchmarks::mesh::mesh_three_short::mesh_three_short,
    // //////////
    // benchmarks::mesh::mesh_three_empty::mesh_three_empty,
    // benchmarks::mesh::mesh_three::mesh_three,
    // benchmarks::mesh::mesh_cancel_three::mesh_three,
    // benchmarks::mesh::mesh_four_empty::mesh_four_empty,
    // benchmarks::mesh::mesh_four::mesh_four,
    // benchmarks::mesh::mesh_cancel_four::mesh_four,
    // benchmarks::mesh::mesh_five_empty::mesh_five_empty,
    // benchmarks::mesh::mesh_five::mesh_five,
    // benchmarks::mesh::mesh_cancel_five::mesh_five,
    // benchmarks::mesh::mesh_six_empty::mesh_six_empty,
    // benchmarks::mesh::mesh_six::mesh_six,
    // benchmarks::mesh::mesh_cancel_six::mesh_six,
    // benchmarks::mesh::mesh_seven_empty::mesh_seven_empty,
    // benchmarks::mesh::mesh_seven::mesh_seven,
    // benchmarks::mesh::mesh_cancel_seven::mesh_seven,
    // benchmarks::mesh::mesh_eight_empty::mesh_eight_empty,
    // benchmarks::mesh::mesh_eight::mesh_eight,
    // benchmarks::mesh::mesh_cancel_eight::mesh_eight,
    // benchmarks::mesh::mesh_nine_empty::mesh_nine_empty,
    // benchmarks::mesh::mesh_nine::mesh_nine,
    // benchmarks::mesh::mesh_cancel_nine::mesh_nine,
    // benchmarks::mesh::mesh_ten_empty::mesh_ten_empty,
    // benchmarks::mesh::mesh_ten::mesh_ten,
    // benchmarks::mesh::mesh_cancel_ten::mesh_ten,
    // benchmarks::mesh::mesh_eleven_empty::mesh_eleven_empty,
    // benchmarks::mesh::mesh_eleven::mesh_eleven,
    // benchmarks::mesh::mesh_cancel_eleven::mesh_eleven,
    // benchmarks::mesh::mesh_twenty_empty::mesh_twenty_empty,
    // benchmarks::mesh::mesh_twenty::mesh_twenty,
    // benchmarks::mesh::mesh_cancel_twenty::mesh_twenty,
    // //////////
    // benchmarks::basic::choose_five::choose_five,
    benchmarks::basic::long_five::long_five_protocol,
    // benchmarks::basic::actyx_os_1::actyx_os_1,
    // benchmarks::basic::actyx_os_2::actyx_os_2,
    // benchmarks::basic::distributed_calc::distributed_calc,
    // benchmarks::basic::fib::fib,
    // benchmarks::basic::o_auth::o_auth,
    // benchmarks::basic::online_wallet::online_wallet,
    // benchmarks::basic::ping_pong::ping_pong,
    // benchmarks::basic::simple_voting_three::simple_voting,
    // benchmarks::basic::smtp::smtp,
    // benchmarks::basic::three_buyer::three_buyer,
    // benchmarks::basic::travel_three::travel_three,
    // benchmarks::basic::video_stream::video_stream,
    // //////////
    // benchmarks::ring::ring_three::ring_three,
    // benchmarks::ring::ring_three_empty::ring_three,
    // benchmarks::ring::ring_four::ring_four,
    // benchmarks::ring::ring_four_empty::ring_four,
    // benchmarks::ring::ring_five::ring_five,
    // benchmarks::ring::ring_five_empty::ring_five,
    // benchmarks::ring::ring_six::ring_six,
    // benchmarks::ring::ring_six_empty::ring_six,
    // benchmarks::ring::ring_seven::ring_seven,
    // benchmarks::ring::ring_seven_empty::ring_seven,
    // benchmarks::ring::ring_eight::ring_eight,
    // benchmarks::ring::ring_eight_empty::ring_eight,
    // benchmarks::ring::ring_nine::ring_nine,
    // benchmarks::ring::ring_nine_empty::ring_nine,
    // benchmarks::ring::ring_ten::ring_ten,
    // benchmarks::ring::ring_ten_empty::ring_ten,
    // benchmarks::ring::ring_eleven::ring_eleven,
    // benchmarks::ring::ring_eleven_empty::ring_eleven,
    // benchmarks::ring::ring_twenty::ring_twenty,
    // benchmarks::ring::ring_twenty_empty::ring_twenty,
}
