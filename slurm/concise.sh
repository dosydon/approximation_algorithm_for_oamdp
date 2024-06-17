#!/bin/bash
#
#SBATCH --partition=cpu    # Partition to submit to 
#
#SBATCH --ntasks=1
#SBATCH --time=60:00         # Maximum runtime in D-HH:MM
#SBATCH --mem-per-cpu=2000    # Memory in MB per cpu allocated

./venv/bin/python3 scripts/compare_concise.py $Filenames $Domain --horizons $Horizons --num_bins $NBins --ids $Ids --n_trials $NTrials --n_expansions $NExpansions> $Output
