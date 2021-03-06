use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::mesh::normal::mesh_three_short::mesh_three_short,
    // //////////
    // benchmarks::mesh::empty::mesh_two::mesh_two,
    // benchmarks::mesh::empty::mesh_three::mesh_three,
    // benchmarks::mesh::empty::mesh_four::mesh_four,
    // benchmarks::mesh::empty::mesh_five::mesh_five,
    // benchmarks::mesh::empty::mesh_six::mesh_six,
    // benchmarks::mesh::empty::mesh_seven::mesh_seven,
    // benchmarks::mesh::empty::mesh_eight::mesh_eight,
    // benchmarks::mesh::empty::mesh_nine::mesh_nine,
    // benchmarks::mesh::empty::mesh_ten::mesh_ten,
    // benchmarks::mesh::empty::mesh_eleven::mesh_eleven,
    // benchmarks::mesh::empty::mesh_twenty::mesh_twenty,
    // //////////
    // benchmarks::mesh::normal::mesh_two::mesh_two,
    // benchmarks::mesh::normal::mesh_three::mesh_three,
    // benchmarks::mesh::normal::mesh_four::mesh_four,
    // benchmarks::mesh::normal::mesh_five::mesh_five,
    // benchmarks::mesh::normal::mesh_six::mesh_six,
    // benchmarks::mesh::normal::mesh_seven::mesh_seven,
    // benchmarks::mesh::normal::mesh_eight::mesh_eight,
    // benchmarks::mesh::normal::mesh_nine::mesh_nine,
    // benchmarks::mesh::normal::mesh_ten::mesh_ten,
    // benchmarks::mesh::normal::mesh_eleven::mesh_eleven,
    // benchmarks::mesh::normal::mesh_twenty::mesh_twenty,
    // //////////
    // benchmarks::mesh::cancel_broadcast::mesh_three::mesh_three,
    // benchmarks::mesh::cancel_broadcast::mesh_four::mesh_four,
    // benchmarks::mesh::cancel_broadcast::mesh_five::mesh_five,
    // benchmarks::mesh::cancel_broadcast::mesh_six::mesh_six,
    // benchmarks::mesh::cancel_broadcast::mesh_seven::mesh_seven,
    // benchmarks::mesh::cancel_broadcast::mesh_eight::mesh_eight,
    // benchmarks::mesh::cancel_broadcast::mesh_nine::mesh_nine,
    // benchmarks::mesh::cancel_broadcast::mesh_ten::mesh_ten,
    // benchmarks::mesh::cancel_broadcast::mesh_eleven::mesh_eleven,
    // benchmarks::mesh::cancel_broadcast::mesh_twenty::mesh_twenty,
    // //////////
    // benchmarks::mesh::cancel::mesh_two::mesh_two,
    // benchmarks::mesh::cancel::mesh_three::mesh_three,
    // benchmarks::mesh::cancel::mesh_four::mesh_four,
    // benchmarks::mesh::cancel::mesh_five::mesh_five,
    // benchmarks::mesh::cancel::mesh_six::mesh_six,
    // benchmarks::mesh::cancel::mesh_seven::mesh_seven,
    // benchmarks::mesh::cancel::mesh_eight::mesh_eight,
    // benchmarks::mesh::cancel::mesh_nine::mesh_nine,
    // benchmarks::mesh::cancel::mesh_ten::mesh_ten,
    // benchmarks::mesh::cancel::mesh_eleven::mesh_eleven,
    // benchmarks::mesh::cancel::mesh_twenty::mesh_twenty,
    // //////////
    // benchmarks::basic::actyx_os_logging::actyx_os_logging,
    // benchmarks::basic::actyx_os_api::actyx_os_api,
    // benchmarks::basic::distributed_calc::distributed_calc,
    // benchmarks::basic::fib::fib,
    // benchmarks::basic::o_auth::o_auth,
    // benchmarks::basic::online_wallet::online_wallet,
    // benchmarks::basic::simple_voting_three::simple_voting,
    // benchmarks::basic::smtp::smtp,
    // benchmarks::basic::three_buyer::three_buyer,
    // benchmarks::basic::travel_three::travel_three,
    // benchmarks::basic::video_stream::video_stream,
    // benchmarks::basic::dns_fowler::dns_fowler,
    // benchmarks::basic::dns_imai::dns_imai,
    // //////////
    // benchmarks::ring::empty::ring_two::ring_two,
    // benchmarks::ring::empty::ring_three::ring_three,
    // benchmarks::ring::empty::ring_four::ring_four,
    // benchmarks::ring::empty::ring_five::ring_five,
    // benchmarks::ring::empty::ring_six::ring_six,
    // benchmarks::ring::empty::ring_seven::ring_seven,
    // benchmarks::ring::empty::ring_eight::ring_eight,
    // benchmarks::ring::empty::ring_nine::ring_nine,
    // benchmarks::ring::empty::ring_ten::ring_ten,
    // benchmarks::ring::empty::ring_eleven::ring_eleven,
    // benchmarks::ring::empty::ring_twenty::ring_twenty,
    // //////////
    // benchmarks::ring::normal::ring_two::ring_two,
    // benchmarks::ring::normal::ring_three::ring_three,
    // benchmarks::ring::normal::ring_four::ring_four,
    // benchmarks::ring::normal::ring_five::ring_five,
    // benchmarks::ring::normal::ring_six::ring_six,
    // benchmarks::ring::normal::ring_seven::ring_seven,
    // benchmarks::ring::normal::ring_eight::ring_eight,
    // benchmarks::ring::normal::ring_nine::ring_nine,
    // benchmarks::ring::normal::ring_ten::ring_ten,
    // benchmarks::ring::normal::ring_eleven::ring_eleven,
    // benchmarks::ring::normal::ring_twenty::ring_twenty,
    // //////////
    // benchmarks::ring::cancel_broadcast::ring_three::ring_three,
    // benchmarks::ring::cancel_broadcast::ring_four::ring_four,
    // benchmarks::ring::cancel_broadcast::ring_five::ring_five,
    // benchmarks::ring::cancel_broadcast::ring_six::ring_six,
    // benchmarks::ring::cancel_broadcast::ring_seven::ring_seven,
    // benchmarks::ring::cancel_broadcast::ring_eight::ring_eight,
    // benchmarks::ring::cancel_broadcast::ring_nine::ring_nine,
    // benchmarks::ring::cancel_broadcast::ring_ten::ring_ten,
    // benchmarks::ring::cancel_broadcast::ring_eleven::ring_eleven,
    // benchmarks::ring::cancel_broadcast::ring_twenty::ring_twenty,
    // //////////
    // benchmarks::ring::cancel::ring_two::ring_two,
    // benchmarks::ring::cancel::ring_three::ring_three,
    // benchmarks::ring::cancel::ring_four::ring_four,
    // benchmarks::ring::cancel::ring_five::ring_five,
    // benchmarks::ring::cancel::ring_six::ring_six,
    // benchmarks::ring::cancel::ring_seven::ring_seven,
    // benchmarks::ring::cancel::ring_eight::ring_eight,
    // benchmarks::ring::cancel::ring_nine::ring_nine,
    // benchmarks::ring::cancel::ring_ten::ring_ten,
    // benchmarks::ring::cancel::ring_eleven::ring_eleven,
    // benchmarks::ring::cancel::ring_twenty::ring_twenty,
}
