#!/bin/bash
#
#SBATCH --partition=cpu    # Partition to submit to 
#
#SBATCH --ntasks=1
#SBATCH --time=60:00         # Maximum runtime in D-HH:MM
#SBATCH --mem-per-cpu=2000    # Memory in MB per cpu allocated

./venv/bin/python3 scripts/plot_offline.py $Filename $Domain $AssignmentId --horizon $Horizon
