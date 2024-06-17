import os
import sys
import statistics

# budgets = [0, 100, 250, 500, 750, 1000, 5000, 10000, 50000, 100000, 500000]
budgets = [0, 1000, 5000, 10000, 50000]

def get_xy_inner(str, dict, toFileName, dirname, domain, instanceId, n):
    xs_mcts_flat = []
    ys_mcts_flat = []
    errs_mcts_flat = []
    for budget in budgets:
        costs = []
        times = []
        for rep in range(0, n):
            filename = os.path.join(dirname, toFileName(domain, instanceId, budget, rep))
            costs.append(dict[filename]["cost"])
#             times.append(dict[filename]["elapsed_time"])
            times.append(budget)
        if any([item == None for item in costs]):
            continue
        print(costs)
        xs_mcts_flat.append(statistics.mean(times))
        ys_mcts_flat.append(statistics.mean(costs))
        if len(costs) > 1:
            errs_mcts_flat.append(statistics.stdev(costs))
        else:
            errs_mcts_flat.append(0.0)
    return ((xs_mcts_flat, ys_mcts_flat, errs_mcts_flat))
