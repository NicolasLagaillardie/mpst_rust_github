#!/bin/bash

set -e

cargo bench --bench main -- --verbose

# cargo criterion --message-format=json

# cargo flamegraph --example long_simple_three_binary -o flamegraphs/flamegraph_long_simple_three_binary.svg
# cargo flamegraph --example long_simple_three_mpst -o flamegraphs/flamegraph_long_simple_three_mpst.svg
# cargo flamegraph --example long_simple_four_binary -o flamegraphs/flamegraph_long_simple_four_binary.svg
# cargo flamegraph --example long_simple_four_mpst -o flamegraphs/flamegraph_long_simple_four_mpst.svg
# cargo flamegraph --example long_simple_five_binary -o flamegraphs/flamegraph_long_simple_five_binary.svg
# cargo flamegraph --example long_simple_five_mpst -o flamegraphs/flamegraph_long_simple_five_mpst.svg
# cargo flamegraph --example long_simple_six_binary -o flamegraphs/flamegraph_long_simple_six_binary.svg
# cargo flamegraph --example long_simple_six_mpst -o flamegraphs/flamegraph_long_simple_six_mpst.svg
# cargo flamegraph --example long_simple_seven_binary -o flamegraphs/flamegraph_long_simple_seven_binary.svg
# cargo flamegraph --example long_simple_seven_mpst -o flamegraphs/flamegraph_long_simple_seven_mpst.svg
# cargo flamegraph --example long_simple_eight_binary -o flamegraphs/flamegraph_long_simple_eight_binary.svg
# cargo flamegraph --example long_simple_eight_mpst -o flamegraphs/flamegraph_long_simple_eight_mpst.svg
# cargo flamegraph --example long_simple_nine_binary -o flamegraphs/flamegraph_long_simple_nine_binary.svg
# cargo flamegraph --example long_simple_nine_mpst -o flamegraphs/flamegraph_long_simple_nine_mpst.svg
# cargo flamegraph --example long_simple_ten_binary -o flamegraphs/flamegraph_long_simple_ten_binary.svg
# cargo flamegraph --example long_simple_ten_mpst -o flamegraphs/flamegraph_long_simple_ten_mpst.svg
# cargo flamegraph --example long_simple_eleven_binary -o flamegraphs/flamegraph_long_simple_eleven_binary.svg
# cargo flamegraph --example long_simple_eleven_mpst -o flamegraphs/flamegraph_long_simple_eleven_mpst.svg

# # ./scripts/compile.sh long_simple_three_binary 
# ./scripts/compile.sh long_simple_three_mpst
# # ./scripts/compile.sh long_simple_four_binary
# ./scripts/compile.sh long_simple_four_mpst 
# # ./scripts/compile.sh long_simple_five_binary
# ./scripts/compile.sh long_simple_five_mpst
# # ./scripts/compile.sh long_simple_six_binary
# ./scripts/compile.sh long_simple_six_mpst 
# # ./scripts/compile.sh long_simple_seven_binary 
# ./scripts/compile.sh long_simple_seven_mpst
# # ./scripts/compile.sh long_simple_eight_binary 
# ./scripts/compile.sh long_simple_eight_mpst 
# # ./scripts/compile.sh long_simple_nine_binary 
# ./scripts/compile.sh long_simple_nine_mpst 
# # ./scripts/compile.sh long_simple_ten_binary 
# ./scripts/compile.sh long_simple_ten_mpst 
# # ./scripts/compile.sh long_simple_eleven_binary 
# ./scripts/compile.sh long_simple_eleven_mpst 
# # ./scripts/compile.sh long_simple_twenty_binary 
# ./scripts/compile.sh long_simple_twenty_mpst
# ./scripts/compile.sh long_simple_three_crossbeam
# ./scripts/compile.sh long_simple_four_crossbeam
# ./scripts/compile.sh long_simple_five_crossbeam
# ./scripts/compile.sh long_simple_six_crossbeam
# ./scripts/compile.sh long_simple_seven_crossbeam
# ./scripts/compile.sh long_simple_eight_crossbeam
# ./scripts/compile.sh long_simple_nine_crossbeam
# ./scripts/compile.sh long_simple_ten_crossbeam
# ./scripts/compile.sh long_simple_eleven_crossbeam
# ./scripts/compile.sh long_simple_twelve_crossbeam
./scripts/compile.sh o_auth
./scripts/compile.sh fib
./scripts/compile.sh simple_voting_three
./scripts/compile.sh three_buyer
./scripts/compile.sh travel_three
./scripts/compile.sh actyx_os_1
./scripts/compile.sh actyx_os_2
./scripts/compile.sh smtp
./scripts/compile.sh distributed_calc
./scripts/compile.sh ping_pong 
./scripts/compile.sh usecase
./scripts/compile.sh online_wallet

# python scripts/create_graph_compile.py 

# python scripts/create_graph_bench.py

# cargo expand --example long_simple_three_mpst > expand/long_simple_three_mpst_expand.txt
# cargo expand --example long_simple_four_mpst > expand/long_simple_four_mpst_expand.txt
# cargo expand --example long_simple_five_mpst > expand/long_simple_five_mpst_expand.txt
# cargo expand --example long_simple_six_mpst > expand/long_simple_six_mpst_expand.txt
# cargo expand --example long_simple_seven_mpst > expand/long_simple_seven_mpst_expand.txt
# cargo expand --example long_simple_eight_mpst > expand/long_simple_eight_mpst_expand.txt
# cargo expand --example long_simple_nine_mpst > expand/long_simple_nine_mpst_expand.txt
# cargo expand --example long_simple_ten_mpst > expand/long_simple_ten_mpst_expand.txt
# cargo expand --example long_simple_eleven_mpst > expand/long_simple_eleven_mpst_expand.txt
# cargo expand --example long_simple_twenty_mpst > expand/long_simple_twenty_mpst_expand.txt

# cargo expand --example long_simple_three_binary > expand/long_simple_three_binary_expand.txt
# cargo expand --example long_simple_four_binary > expand/long_simple_four_binary_expand.txt
# cargo expand --example long_simple_five_binary > expand/long_simple_five_binary_expand.txt
# cargo expand --example long_simple_six_binary > expand/long_simple_six_binary_expand.txt
# cargo expand --example long_simple_seven_binary > expand/long_simple_seven_binary_expand.txt
# cargo expand --example long_simple_eight_binary > expand/long_simple_eight_binary_expand.txt
# cargo expand --example long_simple_nine_binary > expand/long_simple_nine_binary_expand.txt
# cargo expand --example long_simple_ten_binary > expand/long_simple_ten_binary_expand.txt
# cargo expand --example long_simple_eleven_binary > expand/long_simple_eleven_binary_expand.txt
# cargo expand --example long_simple_twenty_binary > expand/long_simple_twenty_binary_expand.txt

# cargo expand --example long_simple_three_crossbeam > expand/long_simple_three_crossbeam_expand.txt
# cargo expand --example long_simple_four_crossbeam > expand/long_simple_four_crossbeam_expand.txt
# cargo expand --example long_simple_five_crossbeam > expand/long_simple_five_crossbeam_expand.txt
# cargo expand --example long_simple_six_crossbeam > expand/long_simple_six_crossbeam_expand.txt
# cargo expand --example long_simple_seven_crossbeam > expand/long_simple_seven_crossbeam_expand.txt
# cargo expand --example long_simple_eight_crossbeam > expand/long_simple_eight_crossbeam_expand.txt
# cargo expand --example long_simple_nine_crossbeam > expand/long_simple_nine_crossbeam_expand.txt
# cargo expand --example long_simple_ten_crossbeam > expand/long_simple_ten_crossbeam_expand.txt
# cargo expand --example long_simple_eleven_crossbeam > expand/long_simple_eleven_crossbeam_expand.txt
# cargo expand --example long_simple_twenty_crossbeam > expand/long_simple_twenty_crossbeam_expand.txt
