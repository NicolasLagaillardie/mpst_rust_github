# Stay Safe under Panic: Affine Rust Programming with Multiparty Session Types (MPST)

---

## Overview

The purpose of this document is to describe in details the steps
required to assess the artifact associated to our paper.

This artifact (artifact.tar.gz) contains (1) the source code for the mp-anon tool -- a tool for safe message-passing programming in Rust and (2) all requires scripts and example needed to reproduce the results from the
ECOOP submission #12: ***Stay Safe under Panic: Affine Rust Programming with Multiparty Session Types (MPST)***. The artifact is submitted as a docker image. The artifact claims functional, reusable and available badge.

## Artifact layout

The artifact (after building the docker image) contains

* The directory `most-rust-github`-- a directory containing the source code of the mp-anon tool
  * `most-rust-github/examples` -- contains many examples implemented using mp-anon, including all examples reported in Fig. 9 and Table 2 in the paper
  * `most-rust-github/scripts` - the scripts for reproducing the results
  * `most-rust-github/benches` --- the examples for Fig. 9
* The directory `kmc` that contains the kmc tool used to verify that mp-anon types written in Rust are compatible
* The directory `scribble-java` that contains the Scribble source code for generating Rust types from
Scribble protocols

## Claims about functionality,reusability and availability

1. **Functionality**:  Mp-anon tool can be used for safe communication programming in Rust. In particular, you should be able to verify three claims from the paper: 
    
    -   Use the mp-anan to write and verify affine protocols using MPST and Scribble as explained in Section 2 in the paper, i.e bottom-upapproach. __Check the claim  by__: following [Part II: Step 1.1](#Step1.1)
    
    -  Use the mp-anan to write and verify affine protocols using MPST and kmc, i.e top-down approach, as explained in Section 2 in the paper. __Check the claim  by__: following [Part II: Step 1.2](#Step1.2)
    
    -  Observe detected errors due to incompatible types, as explained in Section 2 in the paper. 
    __Check the claim  by__: following [Part II: Step 1.3](#Step1.3)

2. **Functionality**: Reproduce the benchmarks in Section 5 (i.e., Table 2 and Figure 9)
    
    2.1 claims expressiveness (Section 5.2 in the paper): examples in Table 2 can be expressed using mp-anon. 

    __Check the claim  by__: Table 2 can be reproduces following the instructions in [Part II: Step 2](#Step2)
    
    2.1. claims on compile-time performance (line 886-892):: 

    - the more participants there are, the higher is the compilation time for MPST
    
    2.2. claims on run-time performance (line 880-885): 

    - mp-anon is faster than the BC implementation when there is a large number of interactions and participants (mesh protocol)

    - the worst-case scenario for mp-anon is protocols with many participants but no causalities between them which results in a slowdown when compared with BC. (ping-pong and ring protocol)

    - AMPST can a negligible overhead in comparison to MPST 
    
    __Check  claims 2.1 and 2.2 by__: Figure 9 can be reproduces following the instructions in [Part II: Step 3](#Step3)
3. **Reusability**: The mp-anon tool can be used to verify your own communication protocols, follow
the instructions in [Part III](#PartIII)
4. **Availability**: We agree our artifact to be  published under a Creative Commons license on DARTS. 

## Prerequisites

To run all benchmarks reported in the paper, the reviewers need:

* a minimum of 16GB RAM and 50 GB of disk space. The library itself is lightweight but the examples and benchmarks pose that requirement.
* to enable localhost access (note that it should be enabled by default, unless you disabled it beforehand)

Note: The benchmark data in the paper was generated
using an 32-cores AMD Opteron<sup>TM</sup> Processor 6282 SE
machine (the tool makes heavy use of multicore, when available)
with a quota of more than 100.000 files and 100 GB of HDD.

<!-- In addition, the tool needs an access to `localhost` for the tests.

/!\ To test it on your own computer, it is recommended to have
16 GB of RAM: the library itself is lightweight,
but all the examples and the benchmarks are very heavy and
need more than 16 GB of RAM.
That is why, we commented the heaviest protocols for a lighter
Docker image and easier compilation.
In the next sections, you will be able to uncomment those files
to test them. -->

---

## Getting started

For the ECOOP'22 artifact evaluation, please use the docker image provided:

0. [Install docker](https://docs.docker.com/engine/install/) and open the terminal.
1. Download the artifact file (assume the filename is `artifact.tar.gz`)
2. Unzip the artifact file.

    ```bash
    gunzip artifact.tar.gz
    ```

3. You should see the tar file `artifact.tar` after last operation.
4. Load the docker image

    ```bash
    docker load < artifact.tar
    ```

5. You should see in the end of the output after last operation:

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
2. Thereafter, we assume that you are in the main directory of the docker file.

<!-- For running the docker file on your own machine,
assuming you downloaded it and you have Docker installed on your machine: -->

<!-- 1. open a terminal
2. move to the folder containing your docker file with `cd`
3. run the command `docker run -it mpanon`. You may need to `sudo` this command.
Note: This may take around 30 min to complete
The password and user in this docker image are both `mpanon`.-->
<!-- During the compilation of the docker file,
tests are ran for the different tools used in this artifact,
hence it may take some time to compile. -->
<!--
The rest of the document is organised as follows: 
* Quick Start lets you test that all required components are installed correctly 
* Mp-anon in 5 minutes walks you through writing your first program with mp-anon and demonstrates both the bottom-up and top-down approach. 
* Repo
Thereafter, we assume that you are in the main directory of the docker file. -->

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

__Note__:
The commands from steps 3-5 can be ran all together with:

```bash
cargo test --all-targets --all-features --workspace # Test everything in the library
```

## Part II: Step by Step instructions

### STEP 1: Run the main example (VideoStream) of the paper (Section 2) 

1. Check and run the running example from the paper using the top-down approach, VideoStreaming. 
<a name="Step1.1"></a>

* execute the following command

```bash
./scripts/top_down.sh
```

2. Check and run the running example from the paper using the bottom-up approach, VideoStreaming.
<a name="Step1.2"></a>

* execute the following command

```bash
./scripts/bottom_up.sh
```

3. Edit the program and observe the reported errors
<a name="Step1.3"></a>

After each modification, compile the program with `cargo run --example=video_stream_full --features="baking_checking` and observe the reported error.

* Open the file [video_stream_full.rs](examples/video_stream_full.rs) in the `examples/` folder, containing the _VideoStream_ program, with your favourite text editor.
Next we highlight how concurrency errors are ruled out by mp-anon (i.e., the ultimate practical purpose of mp-anon). Suggested modifications:
  * swapping lines 104 and 105 (which will lead to a deadlock)
  * using another communication primitive, replace `let (video, s) = s.recv()?;` on line 106 with `let s = s.send(0)?;` -- compilation errors because type mismatch
  * modify the types at line 17, corresponding to line 106, from `Recv` to `Send` -- mismatch because of duality

### STEP 2: Running the examples from in Table 2 (examples from the literature) <a name="Step2"></a>

The purpose of these examples is to demonstrate how the tool works on
existing examples from the literature.

The examples in this table are located in the folder `examples/`
<!-- and duplicated in the `benches/main_all/baking/` folder. -->

The data for these benchmarks can be re-generated using the following script:

```bash
./scripts/examples_literature.sh # Will take up to one hour, progress is displayed in the terminal
```

Each command is ran 10 times on each example and the columns display the means

**Results** are outputted in the file `results/benchmarks_main_from_literature_0.csv` where we give in brackets the corresponding names from Table 2 in the paper:

* Column 1: file name (Example/Endpoint),
* Column 2: **check** time, the result of `cargo check` (Check)
* Column 3: **build** time, the result of `cargo build` (Comp.)
* Column 4: **build --release** time, the result of `cargo build --release` (Rel.)
* Column 5: **run** time, the result of running `cargo bench` (Exec time)

<!-- <details>
<summary>
Details on difference between the different cargo commands (optional reading)
</summary>
The columns 2, 3 and 4 gather the time needed for executing the
respective commands `cargo check`, `cargo build` and `cargo build --release`
with the arguments `--example=[name of the file]` and `--features=baking`.
`cargo check` compiles the provided file and all the required dependencies
given by the argument `--features=baking` but does not
build the binaries.
`cargo build` works the same way than `cargo check` and, in addition,
builds the binaries.
`cargo build --release` builds optimised binaries which are
faster for running time and usually used for benchmarks.
Hence, it is faster for checking a `Rust` file
with `cargo check` than the two others,f
and `cargo build --release` is slower than `cargo build`
as it optimises the output binaries.
For higher accuracy and lower variance,
each command is ran 10 times on each example
and the columns display the means.

The 5th column runs the command `cargo bench` with the arguments
`main` and `--features=baking`.
The first argument is the name of the file containing the files
to be benchmarked and the second argument is the feature used for
compiling the benchmarks.
We use the [criterion](https://crates.io/crates/criterion) `Rust` library
for running the benchmarks.
Each benchmark is ran 10.000 times and `criterion` saves the results
(mean, median, confidence interval, ...) in the `target/criterion/` folder.
They can be displayed separately by opening the file `index.html` in the
`target/criterion/report/` folder. 

Be aware that the scripts adds additional `benchmarks_main_from_literature_*.csv` files
on top of the existing ones.
</details> -->

### STEP 3: Running benchmarks from Figure 9 (ping-pong, mesh and ring protocols) <a name="Step3"></a>

The purpose of these set of benchmarks is to demonstrate the
scalability of the tool on large examples.

#### **Option 1**: Running a small benchmark set

You can run a small set of benchmarks:

```bash
./lightweight_library.sh # Set up
```

then by running the command line

```bash
./scripts/ping_pong_mesh_ring.sh # This will take around 1 hour
```

The above benchmarks run a set of the benchmarks from Figure 9.
In particular, `ping_pong` protocols are up to 200 loops,
and `mesh` and `ring` protocols are up to _five_ participants.

**Results:** After running the above scripts,
5 graphs will be displays corresponding to Figure 9.

The graphs are displayed using `Python matplotlib` and the row data for the graphs (.csv files) is in the [results/](results/) folder.

<details>
<summary>
Details on the content of the raw .csv files data (optional reading).
</summary>

The `ping_pong_mesh_ring.sh` scripts generate 3 files:
`ping_ping_0.csv`, `mesh_0.csv` and `ring_0.csv`
in the folder [results/](results/).

The structure of the `ping_ping_0.csv` file is as follows:

1. Column 1: the type of implementation (`AMPST`, `MPST`, `binary` or `crossbeam`)
2. Column 2: number of loops
3. Column 3: average running time
4. Column 4: average compilation time

The structure of the `mesh_0.csv` and `ring_0.csv`
files is as follows:

1. Column 1: the type of implementation (`AMPST`, `MPST`, `binary` or `crossbeam`)
2. Column 2: number of participants
3. Column 3: average running time
4. Column 4: average compilation time

Be aware that the scripts adds additional `*.csv`files
on top of the existing ones.
</details>

#### **Option 2**: Running the entire benchmark set (at least 24 hours)

To run the same set of benchmarks as in the paper, i.e ping-pong for up to 500 iterations and ring and mesh for 10 participants) execute the following commands:

```bash
./full_library.sh # set up
```

Then you can run the script:

```bash
./scripts/ping_pong_mesh_ring.sh # This will take more than 24 hours
```

__Note__: we have executed this script on the high performance computing server,
and running the whole script took over 24 hours.
Progress is shown while running each benchmark.

You can also run one the following scripts
to retrieve results for only one kind of protocols:

```bash
./scripts/ping_pong.sh # For ping-pong protocols
./scripts/mesh.sh # For mesh protocols
./scripts/ring.sh # For ring protocols
```

---

## Part III: A walkthrough tutorial on checking your own protocols with `Mp-anon` <a name="PartIII"></a>

You can write your own examples using
(1) generated types from `Scribble` (top-down approach) or
(2) your own types written with `Mp-anon` and then checked using the kmc tool (bottom-up approach).

<!-- Mpanon, the `Rust` library introduced in the paper, has one purpose:
allow the implementation of affine communication protocols in `Rust`.

Those protocols can be either generated with another tool
called `Scribble` or wrote by the developers and then checked
by another tool called `KMC`.
Those two approaches, respectively `top-down` and `bottom-up` approaches,
are described separately hereafter. -->

### 3.1 Top-down: Generating Types from Scribble

In the `top-down` approach, protocols written in the protocol description language `Scribble` are
used for generating Mp-anon types.

You can use our implementation of the recursive `Fibonacci` protocol
provided in the `Scribble` repository as a start. The protocol is located
in [scribble-java/scribble-demos/scrib/fib/src/fib/Fib.scr](scribble-java/scribble-demos/scrib/fib/src/fib/Fib.scr)

1️⃣ &nbsp; Generate Rust Types from Scribble

```bash
cd scribble-java/
./scribble-dist/target/scribblec.sh -ip scribble-demos/scrib/fib/src -d scribble-demos/scrib/fib/src scribble-demos/scrib/fib/src/fib/Fib.scr -rustapi Adder Adder_generated
```

In the above example, we move into the `scribble-java` folder and
run the `Scribble` api for `Rust` on the `Fibonacci` protocol written with `Scribble`.
This command outputs the file `Adder_generated.rs` at the root of the `scribble-java`
directory.

Below we move the file the file `Adder_generated.rs` from the `scribble-java` folder to the `examples` subfolder
of the `mpst_rust_github` folder containing `Mpanon`.

```bash
cd ..
mv scribble-java/Adder_generated.rs mpst_rust_github/examples/Adder_generated.rs
```

Now, you can move back into the `mpst_rust_github` folder and
open the `examples/Adder_generated.rs` file using your preferred editor program
before testing the protocol directly with `Mpanon`.

➡️ &nbsp; From this point we assume that you will remain in the `Mpanon` repository.

<!-- Optional: You can check that the generated types are the same as the one provided in
the [adder](examples/adder.rs) file in the [examples/](examples/) folder,
up to line 73. -->

2️⃣ &nbsp;Compile the Rust types

```bash
cargo run --example="Adder_generated" --features=backing_checking
```

This command contains four parts:

1. `cargo` which calls the `Rust` compiler
2. `run` for compiling and running one or more `Rust` files
3. `--example="Adder_generated` for running the specific *example* `Adder_generated`
4. `--features=baking` for compiling only specific parts of `Mpanon` used for the example.

You will have an error and several warnings when running the previous command.
This is because the `Scribble` api only generates `Rust` types
and the `Rust` compiler needs at least a `main` function.
Hereafter, we provide the code to be append to the `Adder_generated.rs`
file to make it work:

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
in the protocol and the last one is for linking and running
them together.

In the first three parts, we are using the primitives
described in Table 1 of the paper:

1. `send(p)` for sending a payload `p`
2. `recv()` for receiving a payload
3. `offer_mpst!` for receiving a choice
4. `choose_mpst_c_to_all!` for sending a choice

The last part uses `fork_mpst` to fork the different functions together.

All those primitives are generated using
the macro `bundle_impl_with_enum_and_cancel!`.

Now, if you run again the file, it should run correctly:

```bash
cargo run --example="Adder_generated" --features=baking
```

__Optional__: Now that your first example works, we can check that it is still
**safe** using the `KMC` tool. If you want to see how bottom-up can be applied to the
previous example, i.e Adder, check [adder_kmc](adder_kmc).

### 3.2 Bottom-up: Write the types in Rust and check them with the kmc tool

__Need help?__: This example is implemented in `examples/basic.rs`, hence you can use the file as a reference implementation.

__Note__: If you want to see how bottom-up can be applied to the
previous example, i.e Adder, check [adder_kmc](adder_kmc).

1️⃣ &nbsp; First, import the necessary macros from the `Mpanon` library:

```rust
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send}; // The basic types
use mpstthree::bundle_impl_with_enum_and_cancel; // The macro for generating the roles and the MeshedChannels
use mpstthree::role::broadcast::RoleBroadcast; // Optional: used only for protocols with choice/offer
use mpstthree::role::end::RoleEnd; // The final type for the stacks and the names of the roles
use mpstthree::checker_concat; // Used for checking the protocol
```

2️⃣ &nbsp;  Then create the **roles** and the **MeshedChannels** data structure:

```rust
bundle_impl_with_enum_and_cancel!(MeshedChannels, A, B); // generates meshed channels for 3 roles
```

<!-- Replace `A, B` with the different names
of the roles you desire.
They must be in the alphabetical order,
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
type OrderingA0 = RoleB<RoleBroadcast>; // Stack for sending a choice

type MoreA1 = Recv<Response, Send<Branching0fromAtoB, End>>; // Recv Response then send a choice
type OrderingMoreA1 = RoleB<RoleBroadcast>; // Stack for the previous binary type

type DoneA1 = Recv<Stop, End>; // Recv Stop
type OrderingDoneA1 = RoleB<RoleEnd>; // Stack for the previous binary type

// Binary types for B
type StartB0 = Send<Request, Recv<Branching0fromAtoB, End>>; // Send a Request then Recv a choice
type OrderingB0 = RoleA<RoleA<RoleEnd>>; // Stack for receiving a choice from A

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
type EndpointA = MeshedChannels<StartA0, OrderingA0, NameA>;

// B
type EndpointB = MeshedChannels<StartB0, OrderingB0, NameB>;
```

3️⃣  &nbsp;  Check that the types are correct

We can check that the written types are compatible using
the `checker_concat!` macro which translates the types to Communicating Finite State machines(CFSM) and uses the kmc tool to check for compatibility.

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
}
```

Run the checker_concat! macro to check if the types are correct

```bash
cargo run --example=my_basic --features=baking_checking
```

After running the command above, the terminal should display
four additional parts:

1. the first three ones are the **dot** graphs representing `A`, `B` and `C`
2. the last one is the minimal **k** for this protocol. It is **1** for the protocol, as expected.

4️⃣ &nbsp;  Implement the endpoint processes for `A`, `B` and `C`.

5️⃣ &nbsp; Run the example

```bash
cargo run --example=my_basic --features=baking_checking
```

## ADDITIONAL INFORMATION

All set-up and benchmark was performed on the following machine:

* AMD Opteron Processor 6282 SE @ 1.30 GHz x 32, 128 GiB memory, 100 GB of HDD,
OS: ubuntu 20.04 LTS (64-bit), Rustup: 1.24.3,  Rust cargo compiler: 1.56.0

The original benchmarks were generated using:

* Compile and run: `cargo bench --all-targets --all-features --workspace`

<!-- 
See main instructions
([README.md](README.md))
for more information. -->

The documentation of `mpanon` can be generated
with the command `cargo doc --all-features`.
The generated documentation will be accessible in the file
[target/doc/mpstthree/index.html](target/doc/mpstthree/index.html).

The source code is included in the root directory.

<details>
<summary> Rust commands on build, test, compile </summary>

Here is a general description of all commands you can run on build, check and test.
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
<summary> Adder example with kmc <a name="adder_kmc"></a> </summary>

<!-- 
The `KMC` tool checks that a given system of communicating automata is *correct*, i.e., all messages that are sent are received, and no automaton gets permanently stuck in a receiving state.
We are not going to introduce how to use it but how `Mpanon` takes advantage
of it *interactive* mode to check protocols.

`Mpanon` uses the macro `checker_concat!` on the types
to create the communicating automata that the `KMC` tool will be able to read.
This macro returns two elements within a tuple:

1. the graphs of each participant using the **dot** format
2. the minimal **k** checked by the protocol

Our theory only supports protocols which have **k=1**,
but protocols with higher can still be implemented using `Mpanon`.
Futhermore, we restricted **k** to be lower than **50**:
any protocol with **k** higher than 50 will be marked as
incorrect.
Indeed, the `KMC` tool does not have an automated way of checking
the minimal **k** of a protocol and `Mpanon`
checks the protocol for each **k** increasing from 1 to 50. -->

Now, that you have a better idea of the interactions between those
two tools, we can improve our `Adder_generated` example to be checked
by the `KMC` tool using our macro `checker_concat!`.
For this purpose, append the following lines to our file:

```rust

/////////////////////////

fn checking() {
    let (graphs, kmc) = mpstthree::checker_concat!(
        "Adder_checking",
        EndpointA48,
        EndpointC13,
        EndpointB50
        =>
        [
            EndpointC7,
            Branches0AtoC, ADD,
            Branches0BtoC, ADD,
        ],
        [
            EndpointC9,
            Branches0AtoC, BYE,
            Branches0BtoC, BYE,
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
cargo run --example="Adder_generated" --features=baking_checking
```

Notice the different feature used for compiling the example.

If you are unsure about either of the above steps,
the `Rust` code is available in the `adder.rs` file
located in the `examples/` folder.

</details>
