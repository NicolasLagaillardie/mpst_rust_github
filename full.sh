#!/bin/sh

set -e

cargo bench --bench main -- --verbose

cargo flamegraph --example long_simple_three_binary -o flamegraphs/flamegraph_long_simple_three_binary.svg
cargo flamegraph --example long_simple_three_mpst -o flamegraphs/flamegraph_long_simple_three_mpst.svg
cargo flamegraph --example long_simple_four_binary -o flamegraphs/flamegraph_long_simple_four_binary.svg
cargo flamegraph --example long_simple_four_mpst -o flamegraphs/flamegraph_long_simple_four_mpst.svg
cargo flamegraph --example long_simple_five_binary -o flamegraphs/flamegraph_long_simple_five_binary.svg
cargo flamegraph --example long_simple_five_mpst -o flamegraphs/flamegraph_long_simple_five_mpst.svg
cargo flamegraph --example long_simple_six_binary -o flamegraphs/flamegraph_long_simple_six_binary.svg
cargo flamegraph --example long_simple_six_mpst -o flamegraphs/flamegraph_long_simple_six_mpst.svg
cargo flamegraph --example long_simple_seven_binary -o flamegraphs/flamegraph_long_simple_seven_binary.svg
cargo flamegraph --example long_simple_seven_mpst -o flamegraphs/flamegraph_long_simple_seven_mpst.svg
cargo flamegraph --example long_simple_eight_binary -o flamegraphs/flamegraph_long_simple_eight_binary.svg
cargo flamegraph --example long_simple_eight_mpst -o flamegraphs/flamegraph_long_simple_eight_mpst.svg
cargo flamegraph --example long_simple_nine_binary -o flamegraphs/flamegraph_long_simple_nine_binary.svg
cargo flamegraph --example long_simple_nine_mpst -o flamegraphs/flamegraph_long_simple_nine_mpst.svg
cargo flamegraph --example long_simple_ten_binary -o flamegraphs/flamegraph_long_simple_ten_binary.svg
cargo flamegraph --example long_simple_ten_mpst -o flamegraphs/flamegraph_long_simple_ten_mpst.svg
cargo flamegraph --example long_simple_eleven_binary -o flamegraphs/flamegraph_long_simple_eleven_binary.svg
cargo flamegraph --example long_simple_eleven_mpst -o flamegraphs/flamegraph_long_simple_eleven_mpst.svg

./compile.sh long_simple_three_binary 
./compile.sh long_simple_three_mpst
./compile.sh long_simple_four_binary
./compile.sh long_simple_four_mpst 
./compile.sh long_simple_five_binary
./compile.sh long_simple_five_mpst
./compile.sh long_simple_six_binary
./compile.sh long_simple_six_mpst 
./compile.sh long_simple_seven_binary 
./compile.sh long_simple_seven_mpst
./compile.sh long_simple_eight_binary 
./compile.sh long_simple_eight_mpst 
./compile.sh long_simple_nine_binary 
./compile.sh long_simple_nine_mpst 
./compile.sh long_simple_ten_binary 
./compile.sh long_simple_ten_mpst 
./compile.sh long_simple_eleven_binary 
./compile.sh long_simple_eleven_mpst 

python create_graph_compile.py 

python create_graph_bench.py
