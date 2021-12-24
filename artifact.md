# Stay Safe under Panic: Affine Rust Programming with Multiparty Session Types (MPST)

---

## Overview

The purpose of this document is to describe in detail the steps
required to assess the artifact associated with our paper.

(!) For better usability, please use the [online](https://gist.github.com/ecoopartifact22/0dd3c058f5599a5e80ed52cb9757e78d) version of this document

The artifact (artifact.tar.gz) contains (1) the source code for the mp-anon tool -- a tool for safe message-passing programming in Rust and (2) all required scripts and examples needed to reproduce the results from the
ECOOP submission #12: ***Stay Safe under Panic: Affine Rust Programming with Multiparty Session Types (MPST)***. The artifact is submitted as a docker image. The artifact claims a functional, reusable and available badge.

## Artifact layout

The artifact (after building the docker image) contains

* The directory `mpst_rust_github` -- a directory containing the source code of the mp-anon tool
  * `mpst_rust_github/examples` -- contains many examples implemented using mp-anon, including all examples reported in Figure 9 and Table 2 in the paper
  * `mpst_rust_github/scripts` -- the scripts for reproducing the results
  * `mpst_rust_github/benches` -- the examples for Figure 9
* The directory `scribble-java` that contains the Scribble source code for generating Rust types from
Scribble protocols
* The directory `kmc` that contains the external kmc tool used to verify that mp-anon types written in Rust are compatible

## Claims about functionality, reusability and availability

1. **Functionality**:  Mp-anon tool can be used for safe communication programming in Rust. In particular, you should be able to verify three claims from the paper:
  
   * Use the mp-anon to write and verify affine protocols using MPST and Scribble as explained in Section 2 in the paper, i.e bottom-up approach.
   __Check the claim  by__: following [Part II: Step 1.1](#Step1.1)

   * Use the mp-anon to write and verify affine protocols using MPST and kmc, i.e top-down approach, as explained in Section 2 in the paper.
   __Check the claim  by__: following [Part II: Step 1.2](#Step1.2)

   * Observe detected errors due to incompatible types, as explained in Section 2 (line 221-225) in the paper.
   __Check the claim  by__: following [Part II: Step 1.3](#Step1.3)

2. **Functionality**: Reproduce the benchmarks in Section 5 (i.e., Table 2 and Figure 9)
  
   2.1 claim expressiveness (Section 5.2 in the paper): examples in Table 2 can be expressed using mp-anon.

   __Check the claim  by__: Table 2 can be reproduced following the instructions in [Part II: Step 2](#Step2)
  
   2.1. claims on compile-time performance (line 886-892):

   * the more participants there are, the higher is the compilation time for MPST
  
   2.2. claims on run-time performance (line 880-885):

   * mp-anon is faster than the BC implementation when there is a large number of interactions and participants (full-mesh protocol)

   * the worst-case scenario for mp-anon is protocols with many participants but no causalities between them which results in a slowdown when compared with BC. (ring protocol)

   * AMPST has a negligible overhead in comparison to MPST
  
   __Check  claims 2.1 and 2.2 by__: Figure 9 can be reproduced following the instructions in [Part II: Step 3](#Step3)
3. **Reusability**: The mp-anon tool can be used to verify your own communication protocols and programs, follow the instructions in [Part III](#PartIII)
4. **Availability**: We agree our artifact to be published under a Creative Commons license on DARTS.

__Note on performance__: the benchmark data in the paper was generated
using a 32-cores AMD Opteron<sup>TM</sup> Processor 6282 SE
machine (the tool makes heavy use of multicore, when available)
with a quota of more than 100.000 files and 100 GB of HDD. In particular, measurements in the paper are taken using AMD Opteron Processor 6282 SE @ 1.30 GHz x 32, 128 GiB memory, 100 GB of HDD, OS: ubuntu 20.04 LTS (64-bit), Rustup: 1.24.3,  Rust cargo compiler: 1.56.0
Depending on your test machine, the absolute values of the measurements produced in Part II: Step 2 and Step 3 will differ from the paper. Nevertheless, the claims stated in the paper should be preserved.

## Prerequisites

To run all benchmarks reported in the paper, the reviewers need:

* a minimum of 16GB RAM and 50 GB of disk space. The library itself is lightweight but the examples and benchmarks pose that requirement.
* to enable localhost access (note that it should be enabled by default unless you disabled it beforehand)

<!-- In addition, the tool needs access to `localhost` for the tests.
 
/!\ To test it on your own computer, it is recommended to have
16 GB of RAM: the library itself is lightweight,
but all the examples and the benchmarks are very heavy and
need more than 16 GB of RAM.
That is why, we commented on the heaviest protocols for a lighter
Docker image and easier compilation.
In the next sections, you will be able to uncomment those files
to test them. -->

---

## Getting started

For the ECOOP'22 artifact evaluation, please use the docker image provided:

0. [Install Docker](https://docs.docker.com/engine/install/) and open the terminal and configure docker setting.
__Important__: By Default docker is limited to use only 2GB-4GB RAM, open docker settings and increase the RAM usage to 16GB.
See instructions for [MacOS](https://docs.docker.com/desktop/mac/) and [Windows](https://docs.docker.com/desktop/windows/)
1. Download the artifact file (assume the filename is `artifact.tar.gz`)
2. Unzip the artifact file.

   ```bash
   gunzip artifact.tar.gz
   ```

3. You should see the tar file `artifact.tar` after the previous operation.
4. Load the docker image

   ```bash
   docker load < artifact.tar
   ```

5. You should see at the end of the output after previous operation:

   ```bash
   Loaded image: mpanon:latest
   ```

6. Run the docker container:

   ```bash
   docker run -it --rm mpanon:latest
   ```

__Note__: You may need to run the above command with sudo

1. The Docker image comes with an installation of `vim` and `neovim` for editing.
  If you wish to install additional software for editing or other purposes, you may obtain sudo access with the password `mpanon`.
2. Thereafter, we assume that you are in the mpst_rust_github directory of the docker file.

## Part I: Quick Start

1. Run the tests to make sure mp-anon is installed and configured correctly

```bash
cargo test --tests --all-features --workspace # Test all tests
```

The above command may take up to 15 min.

2. Run the examples from Table 2:

```bash
cargo test --examples --all-features --workspace # Test all examples
```

The above command may take up to 15 min.

3. Run the benchmarks from Figure 9:

```bash
cargo test --benches --all-features --workspace # Test all benchmarks
```

The above command may take up to 15 min.
If your command results in an error (error: could not compile `mpstthree`; signal: 9, SIGKILL: kill), this indicated that you do not have a sufficient amount of RAM. Make sure that your docker is configured correctly, i.e open docker settings and increase the RAM usage to 16GB.
See instructions for [MacOS](https://docs.docker.com/desktop/mac/) and [Windows](https://docs.docker.com/desktop/windows/).

__Note__:
The commands from steps 1-3 can be run altogether with:

```bash
cargo test --all-targets --all-features --workspace # Test everything in the library
```

## Part II: Step by Step instructions

### STEP 1: Run the main example (VideoStream) of the paper (Section 2)

1. Check and run the running example from the paper using the top-down approach.
<a name="Step1.1"></a>

* execute the following command

```bash
./scripts/top_down.sh
```

2. Check and run the running example from the paper using the bottom-up approach.
<a name="Step1.2"></a>

* execute the following command

```bash
./scripts/bottom_up.sh
```

3. Edit the program and observe the reported errors
<a name="Step1.3"></a>

Next, we highlight how concurrency errors are ruled out by mp-anon (i.e., the ultimate practical purpose of mp-anon).
After each modification, compile the program with `cargo run --example=video_stream_generated --features="baking_checking"` and observe the reported error.

* Open the file [video_stream_generated.rs](examples/video_stream_generated.rs) in the `examples/` folder, containing the _VideoStream_ program, with your favourite text editor.

Suggested modifications:

* swap lines 104 and 105 (this can lead to a deadlock)
* use another communication primitive, replace `let (video, s) = s.recv()?;` on line 106 with `let s = s.send(0)?;` -- compilation errors because type mismatch
* keep the changes from the previous modification and in addition modify the types at line 17, corresponding to line 106, from `Recv` to `Send` -- mismatch because of duality

### STEP 2: Running the examples from Table 2 <a name="Step2"></a>

The purpose of these examples is to demonstrate how the tool works on
existing examples from the literature.

The examples in this table are located in the folder `examples/`
<!-- and duplicated in the `benches/main_all/baking/` folder. -->

The data for these benchmarks can be re-generated using the following script:

```bash
./scripts/examples_literature.sh # Will take up to one hour, progress is displayed in the terminal
```

Each command is run 10 times on each example and the columns display the means in _ms_.

**Results** are outputted in the file `results/benchmarks_main_from_literature_0.csv` where we give in brackets the corresponding names from Table 2 in the paper:

* Column 1: file name (Example/Endpoint),
* Column 2: **check** time in __microseconds__, the result of `cargo check` (Check)
* Column 3: **build** time in __microseconds__, the result of `cargo build` (Comp.)
* Column 4: **build --release** time in __microseconds__, the result of `cargo build --release` (Rel.)
* Column 5: **run** time in __nanoseconds__, the result of running `cargo bench` (Exec time)

<!-- <details>
<summary>
Details on difference between the different cargo commands (optional reading)
</summary>
Columns 2, 3 and 4 gather the time needed for executing the
respective commands `cargo check`, `cargo build` and `cargo build --release`
with the arguments `--example=[name of the file]` and `--features=baking`.
`cargo check` compiles the provided file and all the required dependencies
given by the argument `--features=baking` but does not
build the binaries.
`cargo build` works the same way as `cargo check` and, in addition,
builds the binaries.
`cargo build --release` builds optimised binaries that are
faster for running time and usually used for benchmarks.
Hence, it is faster for checking a `Rust` file
with `cargo check` than the two others,f
and `cargo build --release` is slower than `cargo build`
as it optimises the output binaries.
For higher accuracy and lower variance,
each command is run 10 times on each example
and the columns display the means.
 
The 5th column runs the command `cargo bench` with the arguments
`main` and `--features=baking`.
The first argument is the name of the file containing the files
to be benchmarked and the second argument is the feature used for
compiling the benchmarks.
We use the [criterion](https://crates.io/crates/criterion) `Rust` library
for running the benchmarks.
Each benchmark is run 10.000 times and `criterion` saves the results
(mean, median, confidence interval, ...) in the `target/criterion/` folder.
They can be displayed separately by opening the file `index.html` in the
`target/criterion/report/` folder.
 
Be aware that the scripts adds additional `benchmarks_main_from_literature_*.csv` files
on top of the existing ones.
</details> -->

### STEP 3: Running benchmarks from Figure 9 (ping-pong, mesh and ring protocols) <a name="Step3"></a>

The purpose of this set of benchmarks is to demonstrate the
scalability of the tool on large examples.

#### **Option 1**: Running a small benchmark set

You can run a small set of the benchmarks since the full benchmark set can take about 24 hours.
We have prepared a lighter version that should complete in about an three hours.
The difference is that  `ping_pong` protocols are run up to 50 loops (and not 500),
and `mesh` and `ring` protocols are up to _five_ participants (and not _ten_).
Each benchmark has a significance of 0.1 and a sample size of 100 in this configuration:
each protocol is run 100 times.

These modifications are enough to start observing the performance trends (refer to claims about functionality at the beginning of this document).

To run the lighter benchmark suit:

```bash
./scripts/lightweight_library.sh # Set up
```

then by running the command line

```bash
./scripts/ping_pong_mesh_ring_light.sh # This will take up to 3 hours
```
**Results:** After running the above scripts, the graphs are saved in the [results/](results/) folder in the file graphs_0.pdf,
alongside the raw data for the graphs (.csv files).

To open the graphs_0.pdf file, copy the [results/](results/) folder to a local directory on your machine.

For detailed instructions on how to copy a docker folder to a local folder check [here](https://support.sitecore.com/kb?id=kb_article_view&sysparm_article=KB0383441)
In short, open a terminal, type ```docker ps``` to check the name of the running docker container for mpanon:latest.
The command should return the id of the container, let assume it is c4a9485b3222.
Then given that "Documents/Docker" is a local directory in your system, execute the command:  
```
docker cp c4a9485b3222:"home/mpanon/mpst_rust_github/results" "Documents/Docker"
```
The above will copy the results folder from the docker container to your directory Documents/Docker.
Open the file graphs_0.pdf, it will contain 5 graphs that correspond to the graphs displayed in Figure 9. 

<details>
<summary>
Details on the content of the raw .csv files data (optional reading).
</summary>

The `ping_pong_mesh_ring_light.sh` and `ping_pong_mesh_ring_full.sh` scripts generate 3 files:
`ping_ping_0.csv`, `mesh_0.csv` and `ring_0.csv`
in the folder [results/](results/).

The structure of the `ping_ping_0.csv` file is as follows:

1. Column 1: the type of implementation (`AMPST`, `MPST`, `binary` or `crossbeam`)
2. Column 2: number of loops
3. Column 3: average running time (in nanosecond)
4. Column 4: average compilation time (in microseconds)

The structure of the `mesh_0.csv` and `ring_0.csv`
files are as follows:

1. Column 1: the type of implementation (`AMPST`, `MPST`, `binary` or `crossbeam`)
2. Column 2: number of participants
3. Column 3: average running time (in nanosecond)
4. Column 4: average compilation time (in microseconds)

Be aware that the scripts add additional `*.csv` files
on top of the existing ones.
</details>

#### **Option 2**: Running the entire benchmark set (at least 24 hours)

<!-- Note that this option will fail if you have not enough RAM.
The error displayed in this case will include:

```bash
error: could not compile `mpstthree`

[...]

(signal 9, SIGKILL: kill)
```
-->

To run the same set of benchmarks as in the paper, i.e ping-pong for up to 500 iterations and ring and mesh for 10 participants) execute the following commands:

```bash
./scripts/full_library.sh # set up
```

Then you can run the script:

```bash
./scripts/ping_pong_mesh_ring_full.sh # This will take more than 24 hours
```

Each benchmark has a significance of 0.1 and a sample size of 10000 in this configuration.

__Note__: we have executed this script on a high-performance computing server,
and running the whole script took over 24 hours.
Progress is shown while running each benchmark.

You can also run one of the following scripts
to retrieve results for only one kind of protocol:

```bash
./scripts/ping_pong.sh # For ping-pong protocols
##
./scripts/mesh_full.sh # For mesh protocols with full_library.sh
./scripts/ring_full.sh # For ring protocols with full_library.sh
##
./scripts/mesh_light.sh # For mesh protocols with lightweight_library.sh
./scripts/ring_light.sh # For ring protocols with lightweight_library.sh
```

---

## Part III: A walkthrough tutorial on checking your own protocols with `Mp-anon` <a name="PartIII"></a>

You can write your own examples using
(1) generated types from `Scribble` (top-down approach) or
(2) your own types written with `Mp-anon` and then check them using the kmc tool (bottom-up approach).

### 3.1 Top-down: Generating Types from Scribble

In the `top-down` approach, protocols written in the protocol description language `Scribble` are
used for generating Mp-anon types.

You can use our implementation of a simple recursive protocol that forwards (adds) a number between three participants. The protocol is provided in the `Scribble` repository as a start. The protocol is located
in [scribble-java/scribble-demos/scrib/fib/src/fib/Fib.scr](scribble-java/scribble-demos/scrib/fib/src/fib/Fib.scr)

<details>
<summary>
Follow the steps to implement a simple *adder* example with Scribble and mp-anon
</summary>
 
1️⃣ &nbsp; Generate Rust Types from Scribble

```bash
./scripts/top_down_adder.sh
```

In the above example, we move into the `scribble-java` folder and
run the `Scribble` API for `Rust` on the `Adder` protocol written with `Scribble`.
This command outputs the file `adder_generated.rs` at the root of the `scribble-java` directory.
Then it moves the file `adder_generated.rs` from the `scribble-java` folder to the `examples` subfolder
of the `mpst_rust_github` folder containing `Mpanon`
and auto-format the file with `cargo fmt`.

Now, you can open the `examples/adder_generated.rs` file using your preferred editor program before testing the protocol directly with `Mpanon`.
 
➡️ &nbsp; From this point, we assume that you will remain in the `Mpanon` repository (the mpst_rust_github folder).

<!-- Optional: You can check that the generated types are the same as the one provided in
the [adder](examples/adder.rs) file in the [examples/](examples/) folder,
up to line 73. -->
 
2️⃣ &nbsp; Compile the Rust types

```bash
cargo run --example="adder_generated" --features=baking
```

This command contains four parts:

1. `cargo` which calls the `Rust` compiler
2. `run` for compiling and running one or more `Rust` files
3. `--example="adder_generated` for running the specific *example* `adder_generated`
4. `--features=baking` for compiling only specific parts of `Mpanon` used for the example.

You will have an error and several warnings when running the previous command.
This is because the `Scribble` API only generates `Rust` types
and the `Rust` compiler needs at least a `main` function.

Hereafter, we provide the code for the processes that implement the generated types.
 
3️⃣ &nbsp; Implement the endpoint programs for role `A`, `B` and `C`

```rust
/////////////////////////
 
fn endpoint_a(s: EndpointA48) -> Result<(), Box<dyn Error>> {
   let (_, s) = s.recv()?;
   offer_mpst!(s, {
       Branches0AtoC::Add(s) => {
           recurs_a(s)
       },
       Branches0AtoC::Bye(s) => {
           let (_,s) = s.recv()?;
           s.close()
       },
   })
}
 
fn recurs_a(s: EndpointA23) -> Result<(), Box<dyn Error>> {
   let (_, s) = s.recv()?;
   offer_mpst!(s, {
       Branches0AtoC::Add(s) => {
           recurs_a(s)
       },
       Branches0AtoC::Bye(s) => {
           let (_,s) = s.recv()?;
           s.close()
       },
   })
}
 
/////////////////////////
 
fn endpoint_b(s: EndpointB50) -> Result<(), Box<dyn Error>> {
   offer_mpst!(s, {
       Branches0BtoC::Add(s) => {
           let (_,s) = s.recv()?;
           let s = s.send(0)?;
           endpoint_b(s)
       },
       Branches0BtoC::Bye(s) => {
           let (_,s) = s.recv()?;
           let s = s.send(())?;
           s.close()
       },
   })
}
 
/////////////////////////
 
fn endpoint_c(s: EndpointC13) -> Result<(), Box<dyn Error>> {
   let s = s.send(0)?;
   recurs_c(s, 5)
}
 
fn recurs_c(s: EndpointC10, loops: i32) -> Result<(), Box<dyn Error>> {
   if loops <= 0 {
       let s: EndpointC7 = choose_mpst_c_to_all!(s, Branches0AtoC::Add, Branches0BtoC::Add);
       let s = s.send(0)?;
 
       recurs_c(s, loops - 1)
   } else {
       let s: EndpointC9 = choose_mpst_c_to_all!(s, Branches0AtoC::Bye, Branches0BtoC::Bye);
       let s = s.send(())?;
 
       s.close()
   }
}
 
/////////////////////////
 
fn main() {
   let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);
 
   assert!(thread_a.join().is_ok());
   assert!(thread_b.join().is_ok());
   assert!(thread_c.join().is_ok());
}
```

There are four different parts: the first three ones are for
representing the different roles, `A`, `B` and `C`, involved
in the protocol and the last one (the main function) runs
all processes together.

In the first three parts, we are using the primitives
described in Table 1 of the paper:

1. `send(p)` for sending a payload `p`
2. `recv()` for receiving a payload
3. `offer_mpst!` for receiving a choice
4. `choose_mpst_c_to_all!` for sending a choice

The main function uses `fork_mpst` to fork the different threads.

All those primitives are generated using
the macro `bundle_impl_with_enum_and_cancel!`.

Now, if you run again the file, it should run correctly:

```bash
cargo run --example="adder_generated" --features=baking
```

</details>
<br />

### 3.2 Bottom-up: Write the types in Rust and check them with the kmc tool

</details>
<details>
<summary> Adder example with kmc <a name="adder"></a> </summary>
 We show how to use the bottom-up approach.
 The first step in the bottom-up approach to to write the Rust types for the meshed channels.
 We will use the Adder example from above, since we already have the types and we will only demonstrate here how to check them using the external kmc tool.

 <!--
The `KMC` tool checks that a given system of communicating automata is *correct*, i.e., all messages that are sent are received, and no automaton gets permanently stuck in a receiving state.
We are not going to introduce how to use it but how `Mpanon` takes advantage of its *interactive* mode to check protocols. -->

`Mpanon` uses the macro `checker_concat!` on the types
to rewrite Rust types to communicating finite state machines (CFSM) that the `KMC`checks.

This macro also returns the CFSM (visual) representation for each type using the **dot** format.

<!--
This macro returns two elements within a tuple:

1. the CFSM representation for each type using the **dot** format
2. the minimal **k** checked by the protocol

Our theory only supports protocols that have a bound of **k=1**,
but protocols with higher levels can still be implemented using `Mpanon`. -->

Now, that you have a better idea of the interactions between those
two tools, we will check the types in the `adder_generated` example are correct
using our macro `checker_concat!`.

For this purpose, append the following lines to the `adder_generated.rs` file:

```rust
 
/////////////////////////
 
fn checking() {
   let (graphs, kmc) = mpstthree::checker_concat!(
       "adder_checking",
       EndpointA48,
       EndpointC13,
       EndpointB50
       =>
       [
           EndpointC7,
           Branches0AtoC, Add,
           Branches0BtoC, Add,
       ],
       [
           EndpointC9,
           Branches0AtoC, Bye,
           Branches0BtoC, Bye,
       ]
   )
   .unwrap();
 
   println!("graph A: {:?}", petgraph::dot::Dot::new(&graphs["RoleA"]));
   println!("\n/////////////////////////\n");
   println!("graph B: {:?}", petgraph::dot::Dot::new(&graphs["RoleB"]));
   println!("\n/////////////////////////\n");
   println!("graph C: {:?}", petgraph::dot::Dot::new(&graphs["RoleC"]));
   println!("\n/////////////////////////\n");
   println!("min kMC: {:?}", kmc);
}
```

and update the `main()` function by including `checking();` in it:

```rust
fn main() {
   checking();
 
   let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);
 
   assert!(thread_a.join().is_ok());
   assert!(thread_b.join().is_ok());
   assert!(thread_c.join().is_ok());
}
```

Now, if you run again the file, it should run correctly:

```bash
cargo run --example="adder_generated" --features=baking_checking
```

Notice the different features used for compiling the example: **baking_checking** instead of **baking**.

If you are unsure about either of the above steps,
the `Rust` code is available in the `adder.rs` file
located in the `examples/` folder.

___Optional__: If you want more practice writing types and programs
using mp-anon, and kmc, check the additional examples section at the end of the document:
[A simple example with mp-anon and kmc in the Additional Information section](#example-kmc)

</details>

## ADDITIONAL INFORMATION

<details>
<summary> Benchmark setup in the paper </summary>
All set-up and benchmarks were performed on the following machine:

* AMD Opteron<sup>TM</sup> Processor 6282 SE @ 1.30 GHz x 32, 128 GiB memory, 100 GB of HDD,
OS: ubuntu 20.04 LTS (64-bit), Rustup: 1.24.3,  Rust cargo compiler: 1.56.0

The original benchmarks were generated using:

* Compile and run: `cargo bench --all-targets --all-features --workspace`

<!--
See main instructions
([README.md](README.md))
for more information. -->
</details>

<details>
<summary>
Generating documentation for mp-anon
</summary>
The documentation of `mpanon` can be generated
with the command `cargo doc --all-features`.
The generated documentation will be accessible in the file
[target/doc/mpstthree/index.html](target/doc/mpstthree/index.html).

The source code is included in the root directory.
</details>

<details>
<summary> Rust commands on build, test, compile </summary>

Here is a general description of all commands you can run to check, build and test.
<!-- test `Mpanon` with the following commands: -->

```bash
cd mpst_rust_github # Move to mpanon's repository
cargo check --all-features --lib --workspace # Check only this package's library
cargo check --all-features --bins --workspace # Check all binaries
cargo check --all-features --examples --workspace # Check all examples
cargo check --all-features --tests --workspace # Check all tests
cargo check --all-features --benches --workspace # Check all benchmarks
cargo build --all-features --lib --workspace # Build only this package's library
cargo build --all-features --bins --workspace # Build all binaries
cargo build --all-features --examples --workspace # Build all examples
cargo build --all-features --tests --workspace # Build all tests
cargo build --all-features --benches --workspace # Build all benchmarks
cargo test --all-features --lib --workspace # Test only this package's library
cargo test --all-features --bins --workspace # Test all binaries
cargo test --all-features --examples --workspace # Test all examples
cargo test --all-features --tests --workspace # Test all tests
cargo test --all-features --benches --workspace # Test all benchmarks
 
```

</details>
<details> <summary>  Scribble commands </summary>

Assuming you know how to write `Scribble` protocols,
put your own in the folder `../scribble-java/scribble-demos/scrib/fib/`
and use:

```bash
cd scribble-java/
./scribble-dist/target/scribblec.sh -ip scribble-demos/scrib/fib/src -d scribble-demos/scrib/fib/src scribble-demos/scrib/fib/src/fib/[input file without extension].scr -rustapi [name of the protocol] [output file without extension]
cd ..
mv scribble-java/[input file without extension].rs mpst_rust_github/examples/[output file without extension].rs
cd  mpst_rust_github/
```

</details>

<details>
<summary>
A simple example with mp-anon and kmc <a href="example-kmc"></a>
</summary>

__Need help?__: This example is implemented in `examples/basic.rs`, hence you can use the file as a reference implementation.

1️⃣ &nbsp; First, import the necessary macros from the `Mpanon` library:

```rust
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send}; // The basic types
use mpstthree::bundle_impl_with_enum_and_cancel; // The macro for generating the roles and the MeshedChannels
use mpstthree::role::broadcast::RoleBroadcast; // Optional: used only for protocols with choice/offer
use mpstthree::role::end::RoleEnd; // The final type for the stacks and the names of the roles
use mpstthree::checker_concat; // Used for checking the protocol
use std::error::Error; // Used for functions
```
 
2️⃣ &nbsp;  Then create the **roles** and the **MeshedChannels** data structure:

```rust
bundle_impl_with_enum_and_cancel!(MeshedChannels, A, B); // generates meshed channels for 3 roles
```

<!-- Replace `A, B` with the different names
of the roles you desire.
They must be in alphabetical order,
and a comma at the end is optional. -->
The new generated types will be `MeshedChannels`
and `RoleX` where `X` is the provided name in the macro inputs.
 
2️⃣ &nbsp;  Write the **MeshedChannels** types

A good practice is to write the simplest types first,
and concatenate them into `MeshedChannels`.
That is why we will first write down the types
used for representing the roles:

```rust
// Payload types
struct Request;
struct Response;
struct Stop;
 
// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
```

Then we will write each binary type:

```rust
// Binary types for A
type StartA0 = Recv<Request, Send<Branching0fromAtoB, End>>; // Recv a Request then Send a choice
type OrderingA0 = RoleB<RoleBroadcast>; // Stack for recv then sending a choice

type LoopA0 = Send<Branching0fromAtoB, End>; // Send a choice
type OrderingLoopA0 = RoleBroadcast; // Stack for sending a choice

type MoreA1 = Recv<Response, Send<Branching0fromAtoB, End>>; // Recv Response then send a choice
type OrderingMoreA1 = RoleB<RoleBroadcast>; // Stack for the previous binary type

type DoneA1 = Recv<Stop, End>; // Recv Stop
type OrderingDoneA1 = RoleB<RoleEnd>; // Stack for the previous binary type

// Binary types for B
type StartB0 = Send<Request, Recv<Branching0fromAtoB, End>>; // Send a Request then Recv a choice
type OrderingB0 = RoleA<RoleA<RoleEnd>>; // Stack for send then receiving a choice from A

type LoopB0 = Recv<Branching0fromAtoB, End>; // Recv a choice
type OrderingLoopB0 = RoleA<RoleEnd>; // Stack for recv a choice

type MoreB1 = Send<Response, Recv<Branching0fromAtoB, End>>; // Recv Request then Send Response then receive a choice
type OrderingMoreB1 = RoleA<RoleA<RoleEnd>>; // Stack for the previous binary type

type DoneB1 = Send<Stop, End>; // Send Stop
type OrderingDoneB1 = RoleA<RoleEnd>; // Stack for the previous binary type

enum Branching0fromAtoB {
    // Sum type containing the different paths of the choice
    More(MeshedChannels<MoreB1, OrderingMoreB1, NameB>),
    Done(MeshedChannels<DoneB1, OrderingDoneB1, NameB>),
}
```

This protocol is recursive as you may have noticed
with `MoreB1` both inside the `enum` type `Branching0fromAtoB`
and containing `Recv<Branching0fromAtoB, End>`.
The two paths are `More` and `Done`.

We are now going to concatenate the previous types
into `MeshedChannels`:

```rust
// Creating the endpoints
// A
type EndpointAMore = MeshedChannels<MoreA1, OrderingMoreA1, NameA>;
type EndpointADone = MeshedChannels<DoneA1, OrderingDoneA1, NameA>;
type EndpointALoop = MeshedChannels<LoopA0, OrderingLoopA0, NameA>;
type EndpointA = MeshedChannels<StartA0, OrderingA0, NameA>;
 
// B
type EndpointBLoop = MeshedChannels<LoopB0, OrderingLoopB0, NameB>;
type EndpointB = MeshedChannels<StartB0, OrderingB0, NameB>;
```
 
3️⃣  &nbsp;  Check that the types are correct

We can check that the written types are compatible using
the `checker_concat!` macro which translates the types to Communicating Finite State machines (CFSM) and uses the kmc tool to check for compatibility. Note that, in practice, since this is a binary protocol, we do not need to invoke the kmc tool, since the duality between the types is enough to guarantee correctness.  

```rust
fn main() {
    let (_, kmc) = checker_concat!(
        "basic",
        EndpointA,
        EndpointB
        =>
        [
            EndpointAMore,
            Branching0fromAtoB, More,
        ],
        [
            EndpointADone,
            Branching0fromAtoB, Done,
        ]
    )
    .unwrap();

    println!("min kMC: {:?}", kmc);

    // let (thread_a, thread_b) = fork_mpst(endpoint_a, endpoint_b);

    // assert!(thread_a.join().is_ok());
    // assert!(thread_b.join().is_ok());
}
```

Run the checker_concat! macro to check if the types are correct

```bash
cargo run --example=my_basic --features=baking_checking
```

After running the command above, the terminal should display
the output from the kmc tool, which is the minimal **k** for this protocol. It is **1** for the protocol, as expected.
 
4️⃣ &nbsp;  Implement the endpoint processes for `A`, `B` by adding the following code after the **main** function:

```rust
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    recurs_a(s, 5)
}

fn recurs_a(s: EndpointALoop, loops: i32) -> Result<(), Box<dyn Error>> {
    if loops > 0 {
        let s: EndpointAMore = choose_mpst_a_to_all!(s, Branching0fromAtoB::More);

        let (_, s) = s.recv()?;
        recurs_a(s, loops - 1)
    } else {
        let s: EndpointADone = choose_mpst_a_to_all!(s, Branching0fromAtoB::Done);

        let (_, s) = s.recv()?;
        s.close()
    }
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    let s = s.send(Request {})?;
    recurs_b(s)
}

fn recurs_b(s: EndpointBLoop) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromAtoB::More(s) => {
            let s = s.send(Response {})?;
            recurs_b(s)
        },
        Branching0fromAtoB::Done(s) => {
            let s = s.send(Stop {})?;
            s.close()
        },
    })
}
```

Finally, uncomment the last three lines in the **main** function by removing the `//` at the beginning of each line.  
 
5️⃣ &nbsp; Run the example again:

```bash
cargo run --example=my_basic --features=baking_checking
```

</details>
