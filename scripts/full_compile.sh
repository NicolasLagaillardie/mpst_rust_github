#!/bin/bash

set -e

sleep 5

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

### Basic examples
./scripts/compile.sh o_auth
./scripts/compile.sh o_auth_2
./scripts/compile.sh fib
./scripts/compile.sh simple_voting_three
./scripts/compile.sh three_buyer
./scripts/compile.sh travel_three
./scripts/compile.sh actyx_os_logging
./scripts/compile.sh actyx_os_api
./scripts/compile.sh smtp
./scripts/compile.sh distributed_calc
./scripts/compile.sh usecase
./scripts/compile.sh online_wallet
./scripts/compile.sh dns_fowler
./scripts/compile.sh dns_imai

# ### Ping-pong
# ./scripts/compile.sh ping_pong_binary
# ./scripts/compile.sh ping_pong_cancel
# ./scripts/compile.sh ping_pong_broadcast_cancel
# ./scripts/compile.sh ping_pong_crossbeam
# ./scripts/compile.sh ping_pong_mpst

# ### Mesh
# # Three
# ./scripts/compile.sh mesh_three_binary
# ./scripts/compile.sh mesh_three_cancel
# ./scripts/compile.sh mesh_three_broadcast_cancel
# ./scripts/compile.sh mesh_three_crossbeam
# ./scripts/compile.sh mesh_three_mpst

# # Four
# ./scripts/compile.sh mesh_four_binary
# ./scripts/compile.sh mesh_four_cancel
# ./scripts/compile.sh mesh_four_broadcast_cancel
# ./scripts/compile.sh mesh_four_crossbeam
# ./scripts/compile.sh mesh_four_mpst

# # Five
# ./scripts/compile.sh mesh_five_binary
# ./scripts/compile.sh mesh_five_cancel
# ./scripts/compile.sh mesh_five_broadcast_cancel
# ./scripts/compile.sh mesh_five_crossbeam
# ./scripts/compile.sh mesh_five_mpst

# # Six
# ./scripts/compile.sh mesh_six_binary
# ./scripts/compile.sh mesh_six_cancel
# ./scripts/compile.sh mesh_six_broadcast_cancel
# ./scripts/compile.sh mesh_six_crossbeam
# ./scripts/compile.sh mesh_six_mpst

# # Seven
# ./scripts/compile.sh mesh_seven_binary
# ./scripts/compile.sh mesh_seven_cancel
# ./scripts/compile.sh mesh_seven_broadcast_cancel
# ./scripts/compile.sh mesh_seven_crossbeam
# ./scripts/compile.sh mesh_seven_mpst

# # Eight
# ./scripts/compile.sh mesh_eight_binary
# ./scripts/compile.sh mesh_eight_cancel
# ./scripts/compile.sh mesh_eight_broadcast_cancel
# ./scripts/compile.sh mesh_eight_crossbeam
# ./scripts/compile.sh mesh_eight_mpst

# # Nine
# ./scripts/compile.sh mesh_nine_binary
# ./scripts/compile.sh mesh_nine_cancel
# ./scripts/compile.sh mesh_nine_broadcast_cancel
# ./scripts/compile.sh mesh_nine_crossbeam
# ./scripts/compile.sh mesh_nine_mpst

# # Ten
# ./scripts/compile.sh mesh_ten_binary
# ./scripts/compile.sh mesh_ten_cancel
# ./scripts/compile.sh mesh_ten_broadcast_cancel
# ./scripts/compile.sh mesh_ten_crossbeam
# ./scripts/compile.sh mesh_ten_mpst

# # Eleven
# ./scripts/compile.sh mesh_eleven_binary
# ./scripts/compile.sh mesh_eleven_cancel
# ./scripts/compile.sh mesh_eleven_broadcast_cancel
# ./scripts/compile.sh mesh_eleven_crossbeam
# ./scripts/compile.sh mesh_eleven_mpst

# # Twenty
# ./scripts/compile.sh mesh_twenty_binary
# ./scripts/compile.sh mesh_twenty_cancel
# ./scripts/compile.sh mesh_twenty_broadcast_cancel
# ./scripts/compile.sh mesh_twenty_crossbeam
# ./scripts/compile.sh mesh_twenty_mpst

# ### Ring
# # Three
# ./scripts/compile.sh ring_three_binary
# ./scripts/compile.sh ring_three_cancel
# ./scripts/compile.sh ring_three_broadcast_cancel
# ./scripts/compile.sh ring_three_crossbeam
# ./scripts/compile.sh ring_three_mpst

# # Four
# ./scripts/compile.sh ring_four_binary
# ./scripts/compile.sh ring_four_cancel
# ./scripts/compile.sh ring_four_broadcast_cancel
# ./scripts/compile.sh ring_four_crossbeam
# ./scripts/compile.sh ring_four_mpst

# # Five
# ./scripts/compile.sh ring_five_binary
# ./scripts/compile.sh ring_five_cancel
# ./scripts/compile.sh ring_five_broadcast_cancel
# ./scripts/compile.sh ring_five_crossbeam
# ./scripts/compile.sh ring_five_mpst

# # Six
# ./scripts/compile.sh ring_six_binary
# ./scripts/compile.sh ring_six_cancel
# ./scripts/compile.sh ring_six_broadcast_cancel
# ./scripts/compile.sh ring_six_crossbeam
# ./scripts/compile.sh ring_six_mpst

# # Seven
# ./scripts/compile.sh ring_seven_binary
# ./scripts/compile.sh ring_seven_cancel
# ./scripts/compile.sh ring_seven_broadcast_cancel
# ./scripts/compile.sh ring_seven_crossbeam
# ./scripts/compile.sh ring_seven_mpst

# # Eight
# ./scripts/compile.sh ring_eight_binary
# ./scripts/compile.sh ring_eight_cancel
# ./scripts/compile.sh ring_eight_broadcast_cancel
# ./scripts/compile.sh ring_eight_crossbeam
# ./scripts/compile.sh ring_eight_mpst

# # Nine
# ./scripts/compile.sh ring_nine_binary
# ./scripts/compile.sh ring_nine_cancel
# ./scripts/compile.sh ring_nine_broadcast_cancel
# ./scripts/compile.sh ring_nine_crossbeam
# ./scripts/compile.sh ring_nine_mpst

# # Ten
# ./scripts/compile.sh ring_ten_binary
# ./scripts/compile.sh ring_ten_cancel
# ./scripts/compile.sh ring_ten_broadcast_cancel
# ./scripts/compile.sh ring_ten_crossbeam
# ./scripts/compile.sh ring_ten_mpst

# # Eleven
# ./scripts/compile.sh ring_eleven_binary
# ./scripts/compile.sh ring_eleven_cancel
# ./scripts/compile.sh ring_eleven_broadcast_cancel
# ./scripts/compile.sh ring_eleven_crossbeam
# ./scripts/compile.sh ring_eleven_mpst

# # Twenty
# ./scripts/compile.sh ring_twenty_binary
# ./scripts/compile.sh ring_twenty_cancel
# ./scripts/compile.sh ring_twenty_broadcast_cancel
# ./scripts/compile.sh ring_twenty_crossbeam
# ./scripts/compile.sh ring_twenty_mpst

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

cargo bench --bench main -- --verbose
# cargo bench --bench ping_pong -- --verbose
