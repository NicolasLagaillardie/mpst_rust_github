# Stay Safe under Panic: Affine Rust Programming with Multiparty Session Types (MPST)

## by Nicolas Lagaillardie, Rumyana Neykova and Nobuko Yoshida

---
The purpose of this document is to describe in details the steps
required to assess the artifact associated to our paper.

We would like you to be able to

1. understand how to use the tool to write and verify affine protocols using MPST,
2. reproduce our benchmarks (i.e., Table 2 and Figure 9), and
3. use the tool to verify your own communication protocols.

/!\ *Bear in mind that the benchmark data in the paper was generated
using an 32-cores AMD OpteronTM Processor 6282 SE
machine (the tool makes heavy use of multicore, when available)
with a quota of more than 100.000 files and 100 GB of HDD.
In addition, the tool need an access to `localhost` for the tests.

---

## STEP 0: Getting started

For running the docker file on your own machine,
assuming you downloaded it and you have Docker installed on your machine:

1. open a terminal
2. move to the folder containing your docker file with `cd`
3. run the command `docker run -it [the name of the docker file]`. You may need to `sudo` this command.

The password and user in this docker image are both `multicrusty`.
During the compilation of the docker file,
tests are ran for the different tools used in this artifact,
hence it may take some time to compile.

Thereafter, we assume that you are in the main directory of the docker file.

## STEP 1: Understanding MultiCrusty

MultiCrusty, the `Rust` library introduced in the paper, has one purpose:
allow the implementation of affine communication protocols in `Rust`.

Those protocols can be either generated with another tool
called `Scribble` or wrote by the developers and then checked
by another tool called `KMC`.
Those two approaches, respectively `top-down` and `bottom-up` approaches,
are described separately hereafter.

### Top-down

In the `top-down` approach, protocols written with `Scribble` are
used for generating MultiCrusty types.
`Scribble` is currently the most reliable tool when it comes to write
and check communication protocols.

You can use our implementation of the recursive `Fibonacci` protocol
provided in the `Scribble` repository as a start:

```sh
cd scribble-java/
./scribble-dist/target/scribblec.sh -ip scribble-demos/scrib/fib/src -d scribble-demos/scrib/fib/src scribble-demos/scrib/fib/src/fib/Fib.scr -rustapi Adder Adder_generated
cd ..
mv scribble-java/Adder_generated.rs mpst_rust_github/examples/Adder_generated.rs
```

In the above example, we move into the `scribble-java` folder and
run the `Scribble` api for `Rust` on the `Fibonacci` protocol written with `Scribble`.
This command outputs the file `Adder_generated.rs` at the root of the `scribble-java`
directory.
We then move this file from the `scribble-java` folder to the `examples` subfolder
of the `mpst_rust_github` folder containing `MultiCrusty`.

Now, you can move into the `mpst_rust_github` folder and
open this file using your preferred editor program
before testing the protocol directly with `MultiCrusty`.

From this point we assume that you will remain in the `MultiCrusty` repository.

```sh
cargo run --example="Adder_generated" --features=baking
```

This command contains four parts:

1. `cargo` which calls the `Rust` compiler
2. `run` for compiling and running one or more `Rust` files
3. `--example="Adder_generated` for running the specific *example* `Adder_generated`
4. `--features=baking` for compiling only specific parts of `MultiCrusty` used for the example. This allows faster compilation than compiling the whole library. The different features available are shown in our [README.md](README.md#available-features).

You will have an error and several warnings when running the previous command.
This is because the `Scribble` api only generates `Rust` types
and the `Rust` compiler needs at least a `main` function.
Hereafter, we provide the code to be added to the `Adder_generated.rs`
file to make it work:

```rust
fn endpoint_a(s: EndpointA12) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    offer_mpst!(s, {
        Branches0AtoC::ADD(s) => {
            let (_,s) = s.recv()?;
            s.close()
        },
        Branches0AtoC::BYE(s) => {
            let (_,s) = s.recv()?;
            s.close()
        },
    })
}

/////////////////////////

fn endpoint_b(s: EndpointB14) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branches0BtoC::ADD(s) => {
            let (_,s) = s.recv()?;
            let s = s.send(0)?;
            s.close()
        },
        Branches0BtoC::BYE(s) => {
            let (_,s) = s.recv()?;
            let s = s.send(())?;
            s.close()
        },
    })
}

/////////////////////////

fn endpoint_c(s: EndpointC7) -> Result<(), Box<dyn Error>> {
    recurs_c(s, 5)
}

fn recurs_c(s: EndpointC7, loops: i32) -> Result<(), Box<dyn Error>> {
    let s = s.send(0)?;

    if loops <= 0 {
        let s: EndpointC5 = choose_mpst_c_to_all!(s, Branches0AtoC::BYE, Branches0BtoC::BYE);
        let _ = s.send(())?;

        Ok(())
    } else {
        let s: EndpointC3 = choose_mpst_c_to_all!(s, Branches0AtoC::ADD, Branches0BtoC::ADD);
        let s = s.send(0)?;

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

```sh
cargo run --example="Adder_generated" --features=baking
```

Now that your first example works, we can check that it is still
**safe** using the `KMC` tool.

### Bottom-up

The `KMC` tool has one purpose: check that a given system of communicating automata is *correct*, i.e., all messages that are sent are received, and no automaton gets permanently stuck in a receiving state.
We are not going to introduce how to use it but how `MultiCrusty` takes advantage
of it *interactive* mode to check protocols.

`MultiCrusty` uses the macro `checker_concat!` on the types
to create the communicating automata that the `KMC` tool will be able to read.
This macro returns two elements within a tuple:

1. the graphs of each participant using the **dot** format
2. the minimal **k** checked by the protocol

Our theory only supports protocols which have **k=1**,
but protocols with higher can still be implemented using `MultiCrusty`.
Futhermore, we restricted **k** to be lower than **50**:
any protocol with **k** higher than 50 will be marked as
incorrect.
Indeed, the `KMC` tool does not have an automated way of checking
the minimal **k** of a protocol and `MultiCrusty`
checks the protocol for each **k** increasing from 1 to 50.

Now, that you have a better idea of the interactions between those
two tools, we can improve our `Adder_generated` example to be checked
by the `KMC` tool using our macro `checker_concat!`.
For this purpose, add the following lines to our file:

```rust

/////////////////////////

fn checking() {
    let (graphs, kmc) = mpstthree::checker_concat!(
        "adder_checking",
        EndpointA12,
        EndpointC7,
        EndpointB11
        =>
        [
            EndpointC3,
            Branches0AtoC, ADD,
            Branches0BtoC, ADD,
        ],
        [
            EndpointC5,
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

```sh
cargo run --example="Adder_generated" --features=baking_checking
```

Notice the different feature used for compiling the example.

After running the command above, the terminal should display
four additional parts:

1. the first three ones are the **dot** graphs representing `A`, `B` and `C`
2. the last one is the minimal **k** for this protocol. It is **1** for the protocol, as expected.

If you are unsure about either of the above steps,
the `Rust` code is available in the `adder.rs` file
located in the `examples/` folder.

---

## STEP 2: Running the benchmarks

### Results in Table 2 (examples from the literature)

The purpose of these examples is to demonstrate how the tool works on
existing examples from the literature.

The examples in this table are located inn the folder `examples/`
and duplicated in the `benches/main_all/baking/` folder.
The data for these benchmarks can be
re-generated using the following script:

```sh
./scripts/examples_literature.sh # Will take at least several dozens of minutes, progress is displayed in the terminal
```

which runs our tool on each example listed in Table 2.

The results will be in the file `results/benchmarks_main_from_literature_0.csv` where:

* Column 1: file name,
* Column 2: **check** time
* Column 3: **build** time
* Column 4: **build --release** time
* Column 5: **run** time

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
with `cargo check` than the two others,
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

Be aware that the scripts adds additional `benchmarks_main_from_literature_*.csv`files
on top of the existing ones.

### Results in Figure 9 (ping-pong, mesh and ring protocols)

The purpose of these set of benchmarks is to demonstrate the
scalability of the tool on large examples.

#### Running the entire benchmark set (at least 24 hours)

```sh
./scripts/ping_pong_mesh_ring.sh # This will take more than 24 hours
```

NB: we have executed this script on the high performance computing server,
and running the whole script took over 24 hours.
Progress is shown while running each benchmark.
We propose a similar but smaller set of benchmarks below.

The `ping_pong_mesh_ring.py` scripts generate 3 files:
`ping_ping_0.csv`, `mesh_0.csv` and `ring_0.csv`
in the folder `results/`.

The structure of the `ping_ping_0.csv` file is as follows:

1. Column 1: number of loops
2. Column 2: average running time
3. Column 3: average compilation time

The structure of the `mesh_0.csv` and `ring_0.csv`
files is as follows:

1. Column 1: number of participants
2. Column 2: average running time
3. Column 3: average compilation time

#### Running a smaller benchmark set

You can run a smaller set of benchmarks by commenting
lines in the files respective
`ping_pong.rs`, `mesh.rs` and `ring.rs`
in the folder `benches/` such as:

```rust
...
// ping_pong_all::ping_pong_1::ping_pong,
...
```

instead of

```rust
...
ping_pong_all::ping_pong_1::ping_pong,
...
```

then by running again the command line

```sh
./scripts/ping_pong_mesh_ring.sh
```

you will be able to get the results
for the non-documented protocols.
You can also run one the following scripts
to retrieve results for only one kind of protocols:

```sh
./scripts/ping_pong.sh # For ping-pong protocols
./scripts/mesh.sh # For mesh protocols
./scripts/ring.sh # For ring protocols
```

Be aware that the scripts adds additional `*.csv`files
on top of the existing ones.

---

## STEP 3: Checking your own protocols written with `MultiCrusty`

You can write your own examples using
(1) generated types from `Scribble` or
(2) your own types written `MultiCrusty`.

### Generated types from `Scribble`

Assuming you know how to write `Scribble` protocols,
put your own in the folder `../scribble-java/scribble-demos/scrib/fib/`
and use:

```sh
cd scribble-java/
./scribble-dist/target/scribblec.sh -ip scribble-demos/scrib/fib/src -d scribble-demos/scrib/fib/src scribble-demos/scrib/fib/src/fib/[input file without extension].scr -rustapi [name of the protocol] [output file without extension]
cd ..
mv scribble-java/[input file without extension].rs mpst_rust_github/examples/[output file without extension].rs
cd  mpst_rust_github/
```

then refer to the [top-down](#top-down) section to run your example.

### Your own types written `MultiCrusty`

Your file should be in the folder `examples/`.

First, import the necessary macros and types from the `MultiCrusty`:

```rust
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send}; // The basic types
use mpstthree::bundle_impl_with_enum_and_cancel; // The macro for generating the roles and the MeshedChannels
use mpstthree::role::broadcast::RoleBroadcast; // Optional: used only for protocols with choice/offer
use mpstthree::role::end::RoleEnd; // The final type for the stacks and the names of the roles
use mpstthree::checker_concat; // Used for checking the protocol
```

Then create the **roles** and the **MeshedChannels** types:

```rust
bundle_impl_with_enum_and_cancel!(MeshedChannels, A, B);
```

Replace `A, B` with the different names
of the roles you desire.
They must be in the alphabetical order,
and a comma at the end is optional.
The new generated types will be `MeshedChannels`
and `RoleX` where `X` is the provided name in the macro inputs.

A good practice in `Rust` is to write the simplest types first,
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

Now that we have the types you can check the whole protocol with
the `checker_concat!` macro:

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

This example is also in `examples/basic.rs`

Example of running:

```sh
cargo run --example=basic --features=baking_checking
```

## ADDITIONAL INFORMATION

All set-up and benchmark was performed on the following machine:

* AMD Opteron Processor 6282 SE @ 1.30 GHz x 32
* 128 GiB memory
* 100 GB of HDD
* OS: ubuntu 20.04 LTS (64-bit)
* Rustup: 1.24.3
* Rust cargo compiler: 1.56.0

The original benchmarks were generated using:

* Compile and run: `cargo bench --all-targets --all-features --workspace`

See main instructions
([README.md](https://github.com/NicolasLagaillardie/mpst_rust_github/blob/master/README.md))
for more information and the documentation of the library
[here](https://docs.rs/mpstthree/latest/mpstthree/index.html).

The source code is included in the root directory.
