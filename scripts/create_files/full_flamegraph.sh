#!/bin/bash

# Create the flamegraphs of the examples

set -e

sleep 5

cargo flamegraph --example o_auth -o flamegraphs/flamegraph_o_auth.svg
cargo flamegraph --example o_auth_2 -o flamegraphs/flamegraph_o_auth_2.svg
cargo flamegraph --example fib -o flamegraphs/flamegraph_fib.svg
cargo flamegraph --example simple_voting_three -o flamegraphs/flamegraph_simple_voting_three.svg
cargo flamegraph --example three_buyer -o flamegraphs/flamegraph_three_buyer.svg
cargo flamegraph --example travel_three -o flamegraphs/flamegraph_travel_three.svg
cargo flamegraph --example actyx_os_logging -o flamegraphs/flamegraph_actyx_os_logging.svg
cargo flamegraph --example actyx_os_api -o flamegraphs/flamegraph_actyx_os_api.svg
cargo flamegraph --example smtp -o flamegraphs/flamegraph_smtp.svg
cargo flamegraph --example distributed_calc -o flamegraphs/flamegraph_distributed_calc.svg
cargo flamegraph --example video_stream -o flamegraphs/flamegraph_video_stream.svg
cargo flamegraph --example online_wallet -o flamegraphs/flamegraph_online_wallet.svg
cargo flamegraph --example dns_fowler -o flamegraphs/flamegraph_dns_fowler.svg
cargo flamegraph --example dns_imai -o flamegraphs/flamegraph_dns_imai.svg