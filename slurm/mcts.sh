#!/bin/bash
#
#SBATCH --partition=cpu    # Partition to submit to 
#
#SBATCH --ntasks=1
#SBATCH --time=60:00         # Maximum runtime in D-HH:MM
#SBATCH --mem-per-cpu=8000    # Memory in MB per cpu allocated
#
env RUST_LOG=info ./target/release/mcts $Domain $Instance $NExpansion --horizon $Horizon $Flags