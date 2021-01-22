#!/bin/sh

# ./compile.sh long_simple_three_binary
# ./compile.sh long_simple_three_mpst
# ./compile.sh long_simple_four_binary
# ./compile.sh long_simple_four_mpst
# ./compile.sh long_simple_five_binary
# ./compile.sh long_simple_five_mpst
# ./compile.sh long_simple_six_binary
# ./compile.sh long_simple_six_mpst
# ./compile.sh long_simple_seven_binary
# ./compile.sh long_simple_seven_mpst
# ./compile.sh long_simple_eight_binary
# ./compile.sh long_simple_eight_mpst
# ./compile.sh long_simple_nine_binary
# ./compile.sh long_simple_nine_mpst
# ./compile.sh long_simple_ten_binary
# ./compile.sh long_simple_ten_mpst
# ./compile.sh long_simple_eleven_binary
# ./compile.sh long_simple_eleven_mpst

cargo expand --example long_simple_three_mpst > expand/long_simple_three_mpst_expand.txt
cargo expand --example long_simple_four_mpst > expand/long_simple_four_mpst_expand.txt
cargo expand --example long_simple_five_mpst > expand/long_simple_five_mpst_expand.txt
cargo expand --example long_simple_six_mpst > expand/long_simple_six_mpst_expand.txt
cargo expand --example long_simple_seven_mpst > expand/long_simple_seven_mpst_expand.txt
cargo expand --example long_simple_eight_mpst > expand/long_simple_eight_mpst_expand.txt
cargo expand --example long_simple_nine_mpst > expand/long_simple_nine_mpst_expand.txt
cargo expand --example long_simple_ten_mpst > expand/long_simple_ten_mpst_expand.txt
cargo expand --example long_simple_eleven_mpst > expand/long_simple_eleven_mpst_expand.txt

cargo expand --example long_simple_three_binary > expand/long_simple_three_binary_expand.txt
cargo expand --example long_simple_four_binary > expand/long_simple_four_binary_expand.txt
cargo expand --example long_simple_five_binary > expand/long_simple_five_binary_expand.txt
cargo expand --example long_simple_six_binary > expand/long_simple_six_binary_expand.txt
cargo expand --example long_simple_seven_binary > expand/long_simple_seven_binary_expand.txt
cargo expand --example long_simple_eight_binary > expand/long_simple_eight_binary_expand.txt
cargo expand --example long_simple_nine_binary > expand/long_simple_nine_binary_expand.txt
cargo expand --example long_simple_ten_binary > expand/long_simple_ten_binary_expand.txt
cargo expand --example long_simple_eleven_binary > expand/long_simple_eleven_binary_expand.txt
