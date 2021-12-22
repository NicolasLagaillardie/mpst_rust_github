# All reviewers
For the rest of the evaluation, please use the updated links. 
- Updated artifact (docket image): https://www.dropbox.com/sh/fi8b43601tr1eg0/AABZf3z-8ewFR_PhFp_9R4dga?dl=0,
- Updated artifact instrructions/readme: https://gist.github.com/ecoopartifact22/0dd3c058f5599a5e80ed52cb9757e78d.

Note that you will need to download and load the docker image again. 

# Reviewer C: 
We thank the reviewer for their detailed comments. We have addressed each problem below. 

### P1: how to build a docker image

The reviewer is not required to build a docker image, since we provide the docker image itself. The docker image can be loaded following the instructions in the _Getting started_ part of the readme file (https://gist.github.com/ecoopartifact22/0dd3c058f5599a5e80ed52cb9757e78d#getting-started).
We kindly suggest the reviewer to use the provided docker image to make sure the correct version of the repository is being tested. 

### P2: the repository includes a LICENSE file 

We apologise: we have produced a new version of the repo that omits the licensing files.


### P3 to run `cargo test` I had to switch to the Github directory.
 
Yes, this is what the instructions refer to as the main directory. We have modified the instructions from "Thereafter, we assume that you are in the main directory of the docker file." to  "Thereafter, we assume that you are in the mpst_rust_github directory of the docker file."

### P4 `cargo test --benches --all-features --workspace` caused an error:

This error happens when the Rust compiler crashes due to lack of RAM (signal: 9, SIGKILL: kill reports that The compiler has been killed by the OS due to insufficient RAM). As stated in the readme file, “a minimum of 16GB RAM and 50 GB of disk space” to run the benchmarks in the paper is needed. If not enough RAM is supplied to the VM, the compiler will crash. 
As noted by the reviewer, trying on a more powerful machine solves these issues. 

### P5 The commands in `./scripts` were not marked as executable
We have updated the docker image to make all files in the scripts folder executable. 

The reviewers can also modify and make the files executable as follows: 
```chmod +x  <fully_qualified_name_of__the_file> ```

### P6 Running `./scripts/examples_literature.sh` resulted in the error:

Fixed in the new version. The script was called with the wrong argument.

### P7 `./lightweight_library.sh` should be `./scripts/lightweight_library.sh`

Fixed in the new readme file. 

### P8 `./scripts/ping_pong_mesh_ring.sh` resulted in the error

Fixed in the new version of the artifact and the readme. The script was running an extra example that was not included in the artifact nor in the paper. 
We have also split the lightweight and the full version of the scripts because it was overriding the produced files. 
