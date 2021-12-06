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
Also, the tool need an access to ```localhost``` for the tests.

---

## STEP 0: Getting started

For running the docker file on your own machine,
assuming you downloaded it and you have Docker installed on your machine:

1. open a terminal
2. move to the folder containing your docker file with ```cd```
3. run the command ```docker run -it [the name of the docker file]```

The password and user are both `multicrusty`.
During the compilation of the docker file,
tests are ran for the different tools used in this artifact.

In the following, we assume that you are in the main directory of the docker file.

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

```sh
cd mpst_rust_github/
cargo run --example="Adder_generated" --features="baking"
```

From this point we assume that you will remain in the `MultiCrusty` repository.

You will have an error and several warnings will running the previous command.
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
cargo run --example="Adder_generated" --features="baking"
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
cargo run --example="Adder_generated" --features="baking_checking"
```

Notice the different feature used for compiling the example.

After running the command above, the terminal should display
four additional parts:

1. the first three ones are the **dot** graphs representing `A`, `B` and `C`
2. the last one is the minimal **k** for this protocol. It is **1** for the protocol, as expected.

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
./scripts/examples_literature.py # Will take at least several minutes, progress is displayed in the terminal
```

which runs our tool on each example listed in Table 2.

The results will be in the file `benchmarks_from_literature.csv` where:

* Column 1: file name,
* Column 2: **check** time
* Column 3: **build** time
* Column 4: **build --release** time
* Column 5: **run** time

The columns 2, 3 and 4 gather the time for executing the
respective commands `cargo check`, `cargo build` and `cargo build --release`
from using

For the artifact evaluation, this script runs 2 iterations of each
example. You can change this by adapting the `maxiterations` variable
in `ae-lit-benchmarks.py`. The script contains the path to each
example.

The script above only records timings and sizes. To verify that each
example satisfy the stated properties, please use the *interactive
mode* on the example files.

NB: please rename or delete `benchmarks-fromlit.csv` before running
the script again or new results will be appended to the old file.


### Results in Figure 6 (generated examples)

The purpose of these set of benchmarks is to demonstracte the
scalability of the tool on large examples. All the data used in the
paper is available in `data/benchmarks-data-run-on-8core-machine/`.


#### Running the entire benchmark set (12 hours)

```
./ae-parameterised-benchmarks.py  # This will take about 12 hours
./mkplot.py
```

NB: we have executed this script on the CAV VM, and running the whole
script took just over 12 hours. The results are available in
`./data/benchmarks-data-run-on-CAV-VM`. We propose a similar but
smaller set of benchmarks below.

The `ae-parameterised-benchmarks.py` scripts generate
intermediate timing data that is recorded in 3 files (see files named
`parametrised-benchmarks-X-*.csv`).

The structure of these files is as follows:

* Column 1: n (size of alphabet) 
* Column 2: k (bound)
* Column 3: m (number of peers / 2)
* Column 2: number of states in RTS
* Column 3: number of transitions in RTS
* Column 4: average checking time
* Column 5- : timing of each run 

For the artifact evaluation, this script runs 1 iteration of each
example. You can change via the `maxiterations` variable in
`ae-lit-benchmarks.py` (larger values will give smoother data).

The `mkplot.py` generates .eps plots which should replicate Figure 6
(modulo slowdown due to single core), these plots will be found in the
./plots` directory.

#### Running a smaller benchmark set

You can run a smaller set of benchmarks using the following script:

```
./ae-quick-parameterised-benchmarks.py  # This will take about 2h20m
./mkplot.py
```

This takes about 2h20m on the VM by using smaller k,n, and m. The
structure of the outputs are the same the ones described above.

#### Replicating data points


Alternatively, you can run specific data points using the command
`./GenCFSMs <n> <k> <m>`. For instance,

```
$ ./GenCFSMs 2 4 1 > cfsm.txt
$  time ./KMC cfsm.txt 1 4 --fsm --debug
Trying with bound 1 (*7* states and *6* transitions in RTS)
Trying with bound 2 (*31* states and *30* transitions in RTS)
Trying with bound 3 (*127* states and *126* transitions in RTS)
Trying with bound 4 (*766* states and *1020* transitions in RTS)
4-OBI OK, 4-C/SIBI OK, 4-exhaustivity OK, checking for safety...
4-MC: True

real	0m1.606s
user	0m1.604s
sys	0m0.000s
```

This generate a system of communicating automata consiting of 4
partitions of 2 automata (8 peers), that exchange messages 'a' and 'b'
(n=2), and send a series of 4 messages before receiving 4 times (k=4).

Above the second parameter of `GenCFSM` (2) should be greater than or
equal to the third parameter of `KMC`. Otherwise, you will get

```
$ ./GenCFSMs 2 4 1 > cfsm.txt
$ time ./KMC cfsm.txt 1 3 --fsm --debug  # Note: 3 < 4
Trying with bound 1 (*7* states and *6* transitions in RTS)
Trying with bound 2 (*31* states and *30* transitions in RTS)
Trying with bound 3 (*127* states and *126* transitions in RTS)
I don't know!

real	0m0.024s
user	0m0.020s
sys	0m0.000s

```




---------------------------------------------------------------------
## STEP 3: Checking your own communicating automata

You can write your own examples using a (1) textual (session
type-like) language or (2) finite-state machine language.


### Textual (default).

The file should consist of a list of named session types, i.e., a list
of `A: Ta, ..., Z:Tz` where each Ta..Tz belongs to a language
generated by the following grammar:

```
T ::= rec x . T
  |   A!msg; T
  |   {B1!m1; T1, ..., Bk!mk; Tk}
  |   end
  |   x
```

For instance, the encoding of our running example
(Figure 1) in this language is as follows:
```
C: rec x . S!req;S!data; {S?ko;x, S?ok;end, S?err;end}

S: rec x . C?req; { C!ko;C?data;x
       	   	  , C!ok;C?data; rec y. L!log;y}

L: rec y. S?log;y
```


See `tests/running-example.txt` for the corresponding file.

Example of invocation:

```
$ ./KMC tests/running-example.txt 3
CSA: True
Basic: True
reduced 3-exhaustive: True
reduced 3-safe: True
```

### State machines (--fsm flag).

The file should consist of a list of numbered state machines, e.g.,
this is the encoding of our running example (Figure 1) where "--"
indicate a comment.

```haskell
.outputs  -- <-- This is machine #0
.state graph
q0 1 ! req q1
q1 1 ! data q2
q2 1 ? ko q0
q2 1 ? error q3
q2 1 ? ok q4
.marking q0 -- <-- initial state
.end


.outputs -- <-- This is machine #1
.state graph
q0 0 ? req q1
q1 0 ! ko q2
q2 0 ? data q0
q1 0 ! ok q3
q3 0 ? data q4
q4 2 ! log q4
.marking q0  -- <-- initial state
.end


.outputs -- <-- This is machine #2
.state graph
q0 1 ? log q0
.marking q0  -- <-- initial state
.end
```

This example is also in `tests/benchmarks/client-server-logger.txt`


Example of invocation:
```
$ ./KMC tests/benchmarks/client-server-logger.txt --fsm 3
CSA: True
Basic: True
reduced 3-exhaustive: True
reduced 3-safe: True
```






# ADDITIONAL INFORMATION

All set-up and benchmark was performed on the following machine:

* Intel Core i7-7700 CPU @ 3.60GHz x 8
* 15.5 GiB memory
* OS: ubuntu 16.04 LTS (64-bit)
* GHC 7.10.3
* Python 2.7.12

The original benchmarks were generated using:

* Compile: `ghc KMC -threaded 	       # to enable multicore-support`
* Run: `./KMC <path> <bound> +RTS -N8   # to use 8 cores`

See main instructions ([README.md](https://bitbucket.org/julien-lange/kmc-cav19/src/master/README.md)) for more information.

The source code is included in the root directory (*.hs files). The
most interesting file is CFSM.hs which includes functions to build the
reduced transition system (buildReduceTS) and the several functions
checking the desidred properties (ksafe, kexhaustive, koutputindep,
kinputindep, etc).


NB: This tool is part of a large tool suite (under-construction) which
is available at http://bitbucket.org/julien-lange/k-checking/.

