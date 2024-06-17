#  Approximation Algorithms for Observer-Aware MDPs
This repository contains the code used in the experiments in our paper [Approximation Algorithms for Observer-Aware MDPs](https://openreview.net/pdf?id=UXsERjAZy8).

The code is tested with rustc 1.80.0-nightly.

# Building
```
cargo build --release
```

# Usage
To run Grid-VI, issue the following command:
```
./target/release/grid_vi --help
Usage: grid_vi [OPTIONS] <DOMAIN> <ID> <N_BIN_PER_DIM>

Arguments:
  <DOMAIN>         
  <ID>             
  <N_BIN_PER_DIM>  

Options:
  -d, --display            
  -n, --n <N>              [default: 10]
  -h, --horizon <HORIZON>  [default: 13]
  -h, --help               Print help
  -V, --version            Print version
```

Similarly, to run Grid-(L)RTDP, issue the following command:
```
./target/release/rtdp --help
Usage: rtdp [OPTIONS] <DOMAIN> <ID> <N_BIN_PER_DIM> <NUM_TRIALS>

Arguments:
  <DOMAIN>         
  <ID>             
  <N_BIN_PER_DIM>  
  <NUM_TRIALS>     

Options:
  -n, --n <N>              [default: 10]
  -h, --horizon <HORIZON>  [default: 13]
  -l, --lrtdp              
  -d, --domain-heuristic   
  -h, --help               Print help
  -V, --version            Print version
```