# Fearless Asynchronous Communications with Timed Session Types in Rust

---

## Overview

The purpose of this document is to describe in details the steps
required to assess the artifact associated with our paper.

<!-- (!) For better usability, please use the [online](https://gist.github.com/ecoopartifact22/0dd3c058f5599a5e80ed52cb9757e78d) version of this document -->

The artifact (`artifact.tar.gz`) contains (1) the source code for the **Anon** tool -- a tool for safe message-passing programming in Rust and (2) all required scripts and examples needed to reproduce the results from the paper:
**Fearless Asynchronous Communications with Timed Session Types in Rust**.
The artifact is submitted as a docker image.
The artifact claims a functional,
reusable and available badge.

## Artifact layout

The artifact (after building the docker image) contains

* The directory `mpst_rust_github` -- a directory containing the source code of the **Anon** tool
  * `mpst_rust_github/examples` -- contains many examples implemented using **Anon**, including all examples reported in Figures 11 and 12 in the paper
  * `mpst_rust_github/scripts` -- the scripts for reproducing the results
  * `mpst_rust_github/benches` -- the examples for runtime benchmarks in Figures 11 and 12
* The directory `nuscr` that contains the `Nuscr` source code for generating Rust types from
`Nuscr` protocols

## Claims about functionality, reusability and availability

1. **Functionality**: **Anon** tool can be used for safe communication programming in Rust. In particular, you should be able to verify three claims from the paper:

   * Use **Anon** to write and verify affine timed protocols using MPST and `Nuscr` as explained in Section 2 in the paper.
   __Check the claim by__: following [Part II: Step 1.1](#Step1.1)

   * Observe detected errors due to incompatible types, as explained in Section 5.4 in the paper.
   __Check the claim by__: following [Part II: Step 1.2](#Step1.2)

2. **Functionality**: Reproduce the benchmarks in Section 5 (i.e., Table 2 and Figure 9)

   2.1 claim expressiveness (Section 5.2 in the paper): examples in Table 2 can be expressed using **Anon**.

   __Check the claim by__: Table 2 can be reproduced following the instructions in [Part II: Step 2](#Step2)

   2.1. claims on compile-time performance (line 886-892):

   * the more participants there are, the higher is the compilation time for MPST

   2.2. claims on run-time performance (line 880-885):

   * **Anon** is faster than the BC implementation when there are numerous interactions and participants (full-mesh protocol)

   * the worst-case scenario for **Anon** is protocols with many participants but no causalities between them which results in a slowdown when compared with BC. (ring protocol)

   * AMPST has a negligible overhead in comparison to MPST

   __Check claims 2.1 and 2.2 by__: Figure 9 can be reproduced following the instructions in [Part II: Step 3](#Step3)
3. **Reusability**: The **Anon** tool can be used to verify your own communication protocols and programs, follow the instructions in [Part III](#PartIII)
4. **Availability**: We agree our artifact to be published under a Creative Commons licence on DARTS.

__Note on performance__: the benchmark data in the paper was generated
using an i7-7700K @ 8 x 4.20 GHz workstation
(the tool makes heavy use of multicore, when available)
with 1 TB of SDD,
32 GB memory,
OS:
Manjaro 23.0.0 LTS (64-bit),
Rustup: 1.26,
Rust cargo compiler: 1.70.
Depending on your test machine,
the absolute values of the measurements produced in Part II: Step 2 will differ from the paper. Nevertheless,
the claims stated in the paper should be preserved.

## Prerequisites

To run all benchmarks reported in the paper,
the reviewers need:

* a minimum of 8 GB RAM and 50 GB of disk space. The library itself is lightweight but the examples and benchmarks pose that requirement.
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

For the artifact evaluation, please use the docker image provided:

0. [Install Docker](https://docs.docker.com/engine/install/) and open the terminal and configure docker setting.
__Important__: By Default docker is limited to use all the available RAM on Linux-based systems, and only 2GB-4GB RAM on other platforms, open docker settings and increase the RAM usage to 16 GB.
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
   Loaded image: anon:latest
   ```

6. Run the docker container:

   ```bash
   docker run -it --rm anon:latest
   ```

__Note__: You may need to run the above command with `sudo`. You also may have

1. The Docker image comes with an installation of `vim` and `neovim` for editing.
  If you wish to install additional software for editing or other purposes, you may obtain sudo access with the password `admin`.
2. Thereafter, we assume that you are in the _mpst\_rust\_github_ directory of the docker file.

## Part I: Quick Start

1. Run the tests to make sure **Anon** is installed and configured correctly

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
If your command results in an error (error: could not compile `mpstthree`; signal: 9, SIGKILL: kill), this indicated that you do not have a sufficient amount of RAM. Make sure that your docker is configured correctly, i.e, open docker settings and increase the RAM usage to 16 GB.
See instructions for [MacOS](https://docs.docker.com/desktop/mac/) and [Windows](https://docs.docker.com/desktop/windows/).

__Note__:
The commands from steps 1-3 can be run altogether with:

```bash
cargo test --all-targets --all-features --workspace # Test everything in the library
```

## Part II: Step by Step instructions

### STEP 1: Run the main example (Servo) of the paper (Section 2)

1. Check and run the running example from the paper using the top-down approach.
<a name="Step1.1"></a>

* execute the following command

```bash
./scripts/top_down.sh
```

2. Edit the program and observe the reported errors
<a name="Step1.2"></a>

Next,
we highlight how concurrency errors are ruled out by **Anon**
(i.e., the ultimate practical purpose of **Anon**).
After each modification,
compile and run the program with `cargo run --example=servo_generated --features="baking_timed"` and observe the reported error(s).

* Open the file [servo_generated.rs](examples/servo_generated.rs) in the `examples/` folder,
containing the Servo program,
with your favourite text editor.

Suggested modifications:

* swap lines 104 and 105 (this can lead to a deadlock)
* use another communication primitive, replace `let (video, s) = s.recv()?;` on line 106 with `let s = s.send(0)?;` -- compilation errors because type mismatch
* keep the changes from the previous modification and in addition modify the types at line 17, corresponding to line 106, from `Recv` to `Send` -- mismatch because of duality

### STEP 2: Running the examples from Figure 12<a name="Step2"></a>

The purpose of these examples is to demonstrate how the tool works on
existing examples from the literature.

The examples in this table are located in the folder `examples/`.
<!-- and duplicated in the `benches/main/baking/` folder. -->

The data for these benchmarks can be re-generated using the following script:

```bash
./scripts/examples_affine_timed_literature_extra.sh # Will take up to one hour, progress is displayed in the terminal
python scripts/create_graphs/examples_literature_affine_timed_check_build_release.py
python scripts/create_graphs/examples_extra_affine_timed_check_build_release.py
```

Each example is compiled 10 times and is run 10,000 times.

**Results** are outputted in _pdf_ files named `graphs_bench/examples/extra_build_0.pdf`, `graphs_bench/examples/literature_build_0.pdf`, `graphs_bench/examples/extra_run_0.pdf` and `graphs_bench/examples/literature_run_0.pdf`.

To open the files,
copy them to a local directory on your machine.
For detailed instructions on how to copy a docker folder to a local folder check
[here](https://support.sitecore.com/kb?id=kb_article_view&sysparm_article=KB0383441).
In short, open a terminal,
type ```docker ps``` to check the name of the running docker container for `anon:latest`.
The command should return the _id_ of the container.
Let assume this _id_ is c4a9485b3222,
then given that "Documents/Docker" is a local directory in your system, execute the command:

```
docker cp c4a9485b3222:"home/anon/mpst_rust_github/graphs_bench/" "Documents/Docker"
```

The above will copy the _graphs\_bench_ folder from the docker container to your directory Documents/Docker.

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
We have prepared a lighter version that should complete in about three hours.
The difference is that `mesh` and `ring` protocols are up to _four_ participants (and not _eight_).
Each benchmark has a significance of 0.05 and each protocol is compiled 10 times and run 10,000 times.

These modifications are enough to start observing the performance trends (refer to claims about functionality at the beginning of this document).

To run the lighter benchmark suit:

```bash
cat scripts/toml/light_cargo.toml > Cargo.toml # Set up
cargo bench --bench="ring_*" --all-features -- --verbose # Run the benchmarks for the ring protocols
cargo bench --bench="mesh_*" --all-features -- --verbose # Run the benchmarks for the mesh protocols
##
python scripts/create_graphs/mesh_bench.py # Create graph for the runtime benchmarks for the mesh protocols
python scripts/create_graphs/mesh_compile.py # Create graph for the compile benchmarks for the mesh protocols
python scripts/create_graphs/ring_bench.py # Create graph for the runtime benchmarks for the ring protocols
python scripts/create_graphs/ring_compile.py # Create graph for the compile benchmarks for the ring protocols
```

**Results:** After running the above scripts,
the graphs are saved in the [graphs/mesh/](graphs/mesh/)
and [graphs/ring/](graphs/ring/) folders.

To open the _pdf_ files,
copy the [graphs/](graphs/) folder to a local directory on your machine.

The files named `compile_time_[n].pdf` display the mean of the compilation time
benchmarks for the related protocols,
and the files named `runtime_[n].pdf` display the mean of the runtime
benchmarks for the related protocols,
where `[n]` is an integer.

#### **Option 2**: Running the entire benchmark set (at least 24 hours)

<!-- Note that this option will fail if you have not enough RAM.
The error displayed in this case will include:

```bash
error: could not compile `mpstthree`

[...]

(signal 9, SIGKILL: kill)
```
-->

To run the same set of benchmarks as in the paper, i.e, ping-pong for up to 500 iterations and ring and mesh for 10 participants execute the following commands:

```bash
cat scripts/toml/full_cargo.toml > Cargo.toml # Set up
```

Then you can run the script:

```bash
./scripts/full_benches_compile_running.sh # This will take more than 24 hours
```

Each benchmark has a significance of 0.1 and a sample size of 10000 in this configuration.

__Note__: we have executed this script on a workstation,
and running the whole script took over 24 hours.
Progress is shown while running each benchmark.

You can also run one of the following scripts
to retrieve results for only one kind of protocol:

```bash
./scripts/benches_runtime_mesh_ring.sh # For runtime for mesh and ring protocols
./scripts/full_benches_compile_mesh_ring.sh # For compile time for mesh and ring protocols
##
python scripts/create_graphs/mesh_bench.py # Create graph for the runtime benchmarks for the mesh protocols
python scripts/create_graphs/mesh_compile.py # Create graph for the compile benchmarks for the mesh protocols
python scripts/create_graphs/ring_bench.py # Create graph for the runtime benchmarks for the ring protocols
python scripts/create_graphs/ring_compile.py # Create graph for the compile benchmarks for the ring protocols
```

---

## Part III: A walkthrough tutorial on checking your own protocols with `**Anon**` <a name="PartIII"></a>

You can write your own examples using generated types
from `Nuscr` (top-down approach)

### 3.1 Top-down: Generating Types from `Nuscr`

In the `top-down` approach, protocols written in the protocol description language `Nuscr` are
used for generating **Anon** types.

You can use our implementation of a simple recursive protocol
that forwards (adds) a number between three participants.
The protocol is provided in the `Nuscr` repository as a start.
The protocol is located in [Nuscr-java/Nuscr-demos/scrib/fib/src/fib/Fib.scr](Nuscr-java/Nuscr-demos/scrib/fib/src/fib/Fib.scr)

<details>
<summary>
Follow the steps to implement a simple *adder* example with Nuscr and **Anon**
</summary>

1️⃣ &nbsp; Generate Rust Types from `Nuscr`

```bash
./scripts/top_down_adder.sh
```

In the above example,
we move into the `Nuscr-java` folder and
run the `Nuscr` API for `Rust` on the `Adder` protocol written with `Nuscr`.
This command outputs the file `adder_generated.rs`
at the root of the `Nuscr-java` directory.
Then it moves the file `adder_generated.rs`
from the `Nuscr-java` folder to the `examples` subfolder
of the `mpst_rust_github` folder
and auto-format the file with `cargo fmt`.

Now,
you can open the `examples/adder_generated.rs` file
using your preferred editor program before testing the protocol directly with **Anon**.

➡️ &nbsp; From this point, we assume that you will remain in the `mpst_rust_github` folder.

<!-- Optional: You can check that the generated types are the same as the one provided in
the [adder](examples/adder.rs) file in the [examples/](examples/) folder,
up to line 73. -->

2️⃣ &nbsp; Compile the `Rust` types

```bash
cargo run --example adder_generated  --features baking_timed
```

This command contains four parts:

1. `cargo` which calls the `Rust` compiler
2. `run` for compiling and running one or more `Rust` files
3. `--example adder_generated` for running the specific *example* `adder_generated`
4. `--features baking_timed` for compiling only specific parts of **Anon** used for the example.

You will have an error and several warnings when running the previous command.
This is because the `Nuscr` API only generates `Rust` types,
not the functions,
and the `Rust` compiler needs at least a `main` function.

Hereafter,
we provide the code for the processes that implement the generated types.

3️⃣ &nbsp; Implement the endpoint programs for role `A`, `B` and `C`

```rust
/////////////////////////

fn endpoint_a(s: EndpointA48, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_, s) = s.recv()?;
    offer_mpst!(s, all_clocks, {
        Branches0AtoC::Add(s) => {
            recurs_a(s, all_clocks)
        },
        Branches0AtoC::Bye(s) => {
            let (_,s) = s.recv(all_clocks)?;
            s.close()
        },
    })
}

fn recurs_a(s: EndpointA23, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv(all_clocks)?;
    offer_mpst!(s, all_clocks, {
        Branches0AtoC::Add(s) => {
            recurs_a(s, all_clocks)
        },
        Branches0AtoC::Bye(s) => {
            let (_,s) = s.recv(all_clocks)?;
            s.close()
        },
    })
}

/////////////////////////

fn endpoint_b(s: EndpointB50, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    offer_mpst!(s, {
        Branches0BtoC::Add(s) => {
            let (_,s) = s.recv(all_clocks)?;
            let s = s.send(0, all_clocks)?;
            endpoint_b(s, all_clocks)
        },
        Branches0BtoC::Bye(s) => {
            let (_,s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            s.close()
        },
    })
}

/////////////////////////

fn endpoint_c(s: EndpointC13, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let s = s.send(0, all_clocks)?;
    recurs_c(s, 5, all_clocks)
}

fn recurs_c(s: EndpointC10, loops: i32, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    if loops <= 0 {
        let s: EndpointC7 = choose_mpst_c_to_all!(s, all_clocks, Branches0AtoC::Add, Branches0BtoC::Add);
        let s = s.send(0, all_clocks)?;

        recurs_c(s, loops - 1, all_clocks)
    } else {
        let s: EndpointC9 = choose_mpst_c_to_all!(s, all_clocks, Branches0AtoC::Bye, Branches0BtoC::Bye);
        let s = s.send((), all_clocks)?;

        s.close()
    }
}

/////////////////////////

fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}
```

There are four different parts:
the first three ones are for
representing the different roles, `A`, `B` and `C` involved
in the protocol,
and the last one (the `main` function) runs
all processes together.

In the first three parts, we are using the primitives
described in Table 1 of the paper:

1. `send(p, all_clocks)` for sending a payload `p` if the clock in `all_clocks` is within the time bounds
2. `recv(all_clocks)` for receiving a payload if the clock in `all_clocks` is within the time bounds
3. `offer_mpst!` for receiving a choice if the clock in `all_clocks` is within the time bounds
4. `choose_mpst_c_to_all!` for sending a choice if the clock in `all_clocks` is within the time bounds

The main function uses `fork_mpst` to fork the different threads.

All those primitives are generated using
the macro `baker!`.

Now, if you run again the file, it should run correctly:

```bash
cargo run --example adder_generated --features baking
```

</details>
<br />

## ADDITIONAL INFORMATION

<details>
<summary> Benchmark setup in the paper </summary>
All set-up and benchmarks were performed on the following machine:

* i7-7700K @ 8 x 4.20 GHz, 32 GB memory, 1 TB of SSD, OS: Manjaro 23.0.0 LTS (64-bit), Rustup: 1.26, Rust cargo compiler: 1.70

The original benchmarks were generated using:

* Compile and run: `cargo bench --all-targets --all-features --workspace`

<!--
See main instructions
([README.md](README.md))
for more information. -->
</details>

<details>
<summary>
Generating documentation for **Anon**
</summary>
The documentation of **Anon** can be generated
with the command `cargo doc --all-features`.
The generated documentation will be accessible in the file
[target/doc/mpstthree/index.html](target/doc/mpstthree/index.html).

The source code is included in the root directory.
</details>

<details>
<summary> Rust commands on build, test, compile </summary>

Here is a general description of all commands you can run to check, build and test.
<!-- test **Anon** with the following commands: -->

```bash
cd mpst_rust_github # Move to anon's repository
cargo check --all-features --lib --workspace # Check only this package's library
cargo check --all-features --bins --workspace # Check all binaries
cargo check --all-features --examples --workspace # Check all examples
cargo check --all-features --tests --workspace # Check all tests
cargo check --all-features --benches --workspace # Check all benchmarks
##
cargo build --all-features --lib --workspace # Build only this package's library
cargo build --all-features --bins --workspace # Build all binaries
cargo build --all-features --examples --workspace # Build all examples
cargo build --all-features --tests --workspace # Build all tests
cargo build --all-features --benches --workspace # Build all benchmarks
##
cargo test --all-features --lib --workspace # Test only this package's library
cargo test --all-features --bins --workspace # Test all binaries
cargo test --all-features --examples --workspace # Test all examples
cargo test --all-features --tests --workspace # Test all tests
cargo test --all-features --benches --workspace # Test all benchmarks

```

</details>
<details> <summary>Nuscr commands</summary>

Assuming you know how to write `Nuscr` protocols,
put your own in the folder `../Nuscr-java/Nuscr-demos/scrib/fib/`
and use:

```bash
cd Nuscr-java/
./Nuscr-dist/target/Nuscrc.sh -ip Nuscr-demos/scrib/fib/src -d Nuscr-demos/scrib/fib/src Nuscr-demos/scrib/fib/src/fib/[input file without extension].scr -rustapi [name of the protocol] [output file without extension]
cd ..
mv Nuscr-java/[input file without extension].rs mpst_rust_github/examples/[output file without extension].rs
cd mpst_rust_github/
```

__Need help?__: This example is implemented in `examples/basic.rs`,
hence you can use the file as a reference implementation.

1️⃣ &nbsp; First, import the necessary macros from the **Anon** library:

```rust
use mpstthree::binary::struct_trait::end::End; // The basic End types
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed}; // The basic timed types
use mpstthree::generate_timed; // The macro for generating the roles and the MeshedChannels
use mpstthree::role::broadcast::RoleBroadcast; // Optional: used only for protocols with choice/offer
use mpstthree::role::end::RoleEnd; // The final type for the stacks and the names of the roles

use std::collections::HashMap; // Used for storing clocks
use std::error::Error; // Used for functions returning _affine_ types
use std::time::Instant; // Used for clocks
```

2️⃣ &nbsp; Then create the **roles** and the **MeshedChannels** data structure:

```rust
generate_timed!(MeshedChannels, A, B); // generates meshed channels for 3 roles
```

<!-- Replace `A, B` with the different names
of the roles you desire.
They must be in alphabetical order,
and a comma at the end is optional. -->
The new generated types will be `MeshedChannels`
and `NameX` where `X` is the provided name in the macro inputs.

2️⃣ &nbsp; Write the **MeshedChannels** types

A good practice is to write the simplest types first,
and concatenate them into `MeshedChannels`.
That is why we will first write down the types
used for representing the roles:

```rust
// Payload types
struct Request;
struct Response;
struct Stop;
```

Then we will write each binary type:

```rust
// Binary types for A
type StartA0 = RecvTimed<Request, 'a', 0, true, 10, true, ' ', SendTimed<Branching0fromAtoB, 'a', 0, true, 10, true, ' ', End>>; // RecvTimed a Request then SendTimed a choice
type OrderingA0 = RoleB<RoleBroadcast>; // Stack for recv then sending a choice

type LoopA0 = SendTimed<Branching0fromAtoB, 'a', 0, true, 10, true, ' ', End>; // SendTimed a choice
type OrderingLoopA0 = RoleBroadcast; // Stack for sending a choice

type MoreA1 = RecvTimed<Response, 'a', 0, true, 10, true, ' ', SendTimed<Branching0fromAtoB, 'a', 0, true, 10, true, ' ', End>>; // RecvTimed Response then send a choice
type OrderingMoreA1 = RoleB<RoleBroadcast>; // Stack for the previous binary type

type DoneA1 = RecvTimed<Stop, 'a', 0, true, 10, true, ' ', End>; // RecvTimed Stop
type OrderingDoneA1 = RoleB<RoleEnd>; // Stack for the previous binary type

// Binary types for B
type StartB0 = SendTimed<Request, 'a', 0, true, 10, true, ' ', RecvTimed<Branching0fromAtoB, 'a', 0, true, 10, true, ' ', End>>; // SendTimed a Request then RecvTimed a choice
type OrderingB0 = RoleA<RoleA<RoleEnd>>; // Stack for send then receiving a choice from A

type LoopB0 = RecvTimed<Branching0fromAtoB, 'a', 0, true, 10, true, ' ', End>; // RecvTimed a choice
type OrderingLoopB0 = RoleA<RoleEnd>; // Stack for recv a choice

type MoreB1 = SendTimed<Response, 'a', 0, true, 10, true, ' ', RecvTimed<Branching0fromAtoB, 'a', 0, true, 10, true, ' ', End>>; // RecvTimed Request then SendTimed Response then receive a choice
type OrderingMoreB1 = RoleA<RoleA<RoleEnd>>; // Stack for the previous binary type

type DoneB1 = SendTimed<Stop, 'a', 0, true, 10, true, ' ', End>; // SendTimed Stop
type OrderingDoneB1 = RoleA<RoleEnd>; // Stack for the previous binary type

enum Branching0fromAtoB {
    // Sum type containing the different paths of the choice
    More(MeshedChannels<MoreB1, OrderingMoreB1, NameB>),
    Done(MeshedChannels<DoneB1, OrderingDoneB1, NameB>),
}
```

This protocol is recursive as you may have noticed
with `MoreB1` both inside the `enum` type `Branching0fromAtoB`
and containing `RecvTimed<Branching0fromAtoB, 'a', 0, true, 10, true, ' ', End>`.
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

3️⃣ &nbsp; Check that the types are correct with:

```bash
cargo check --example my_basic --features baking_timed
```

No errors should be displayed,
only warnings of unused types.

4️⃣ &nbsp; Implement the endpoint processes for `A`, `B` by removing the line `fn main() {}`
and appending the following code:

```rust
fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_, s) = s.recv(all_clocks)?;
    recurs_a(s, 5, all_clocks)
}

fn recurs_a(s: EndpointALoop, loops: i32, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    if loops > 0 {
        let s: EndpointAMore = choose_mpst_a_to_all!(s, all_clocks, Branching0fromAtoB::More);

        let (_, s) = s.recv(all_clocks)?;
        recurs_a(s, loops - 1, all_clocks)
    } else {
        let s: EndpointADone = choose_mpst_a_to_all!(s, all_clocks, Branching0fromAtoB::Done);

        let (_, s) = s.recv(all_clocks)?;
        s.close()
    }
}

fn endpoint_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let s = s.send(Request {}, all_clocks)?;
    recurs_b(s, all_clocks)
}

fn recurs_b(s: EndpointBLoop, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching0fromAtoB::More(s) => {
            let s = s.send(Response {}, all_clocks)?;
            recurs_b(s, all_clocks)
        },
        Branching0fromAtoB::Done(s) => {
            let s = s.send(Stop {}, all_clocks)?;
            s.close()
        },
    })
}

fn main() {
    let (thread_a, thread_b) = fork_mpst(endpoint_a, endpoint_b);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}
```

5️⃣ &nbsp; Run the example again:

```bash
cargo run --example my_basic --features baking_timed
```

</details>

## Acknowledgements

The authors wish to thank Martin Vassor for their comments and testing the artifact,
and the anonymous reviewers for their comments and suggestions.

## Funding

Funding The work is partially supported by VeTSS, EPSRC EP/K011715/1, EP/K034413/1,
EP/L00058X/1, EP/N027833/1, EP/N028201/1, EP/T006544/1 and EP/T014709/1.
