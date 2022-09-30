#!/bin/bash

# Create the flamegraphs of the examples

set -e

sleep 5

cargo flamegraph --example o_auth -o flamegraphs/flamegraph_o_auth.svg
cargo flamegraph --example o_auth_transport -o flamegraphs/flamegraph_o_auth_transport.svg
cargo flamegraph --example fib -o flamegraphs/flamegraph_fib.svg
cargo flamegraph --example simple_voting -o flamegraphs/flamegraph_simple_voting.svg
cargo flamegraph --example three_buyers -o flamegraphs/flamegraph_three_buyer.svg
cargo flamegraph --example three_travel -o flamegraphs/flamegraph_three_travel.svg
cargo flamegraph --example logging -o flamegraphs/flamegraph_logging.svg
cargo flamegraph --example circuit_breaker -o flamegraphs/flamegraph_circuit_breaker.svg
cargo flamegraph --example logging_baking -o flamegraphs/flamegraph_logging_baking.svg
cargo flamegraph --example circuit_breaker_baking -o flamegraphs/flamegraph_circuit_breaker_baking.svg
cargo flamegraph --example logging_interleaved -o flamegraphs/flamegraph_logging_interleaved.svg
cargo flamegraph --example circuit_breaker_logging_interleaved -o flamegraphs/flamegraph_circuit_breaker_logging_interleaved.svg
cargo flamegraph --example smtp -o flamegraphs/flamegraph_smtp.svg
cargo flamegraph --example distributed_calc -o flamegraphs/flamegraph_distributed_calc.svg
cargo flamegraph --example video_stream -o flamegraphs/flamegraph_video_stream.svg
cargo flamegraph --example online_wallet -o flamegraphs/flamegraph_online_wallet.svg
cargo flamegraph --example dns_fowler -o flamegraphs/flamegraph_dns_fowler.svg
cargo flamegraph --example dns_imai -o flamegraphs/flamegraph_dns_imai.svg