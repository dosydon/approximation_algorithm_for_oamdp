#!/usr/bin/env python
import matplotlib.pyplot as plt
from matplotlib.cm import get_cmap
import statistics
import json
import os
import sys

def toMCTSFilename(domain, instanceId, budget, horizon, rep):
    return "mcts_{}_{}_{}_{}_{}.json".format(domain, instanceId, budget, horizon, rep)

def toMCTSFullFilename(domain, instanceId, budget, horizon, rep):
    return "mcts_full_{}_{}_{}_{}_{}.json".format(domain, instanceId, budget, horizon, rep)

def toGridVIFilename(domain, instanceId, nBin, horizon):
    return "grid_vi_{}_{}_{}_{}.json".format(domain, instanceId, nBin, horizon)

def toRTDPFilename(domain, assignmentId, nBin, nTrial, horizon, rep):
    return "rtdp_{}_{}_{}_{}_{}_{}.json".format(domain, assignmentId, nBin, nTrial, horizon, rep)

def toLRTDPFilename(domain, assignmentId, nBin, nTrial, horizon, rep):
    return "lrtdp_{}_{}_{}_{}_{}_{}.json".format(domain, assignmentId, nBin, nTrial, horizon, rep)

def toLRTDPDFilename(domain, assignmentId, nBin, nTrial, horizon, rep):
    return "lrtdp_d_{}_{}_{}_{}_{}_{}.json".format(domain, assignmentId, nBin, nTrial, horizon, rep)

def toRTDPDFilename(domain, assignmentId, nBin, nTrial, horizon, rep):
    return "rtdp_d_{}_{}_{}_{}_{}_{}.json".format(domain, assignmentId, nBin, nTrial, horizon, rep)

budgets = [100, 500, 1000, 5000, 10000, 50000, 100000, 500000, 1000000, 5000000]
# budgets = [100, 500, 1000, 5000, 10000, 50000]
# nBins = [1, 4, 8]
# nTrials = [100, 500, 1000, 10000, 50000]
nTrials = [100, 500, 1000, 10000, 50000, 100000, 500000, 1000000, 5000000]

def get_xy_mcts(filenameMerged, domain, instanceId, horizon, n):
    dirname = (os.path.dirname(filenameMerged))
    with open(filenameMerged) as f:
        str = f.read()
        dict = json.loads(str)

        xs_mcts_flat = []
        ys_mcts_flat = []
        errs_mcts_flat = []
        for budget in budgets:
            costs = []
            times = []
            for rep in range(0, n):
                filename = os.path.join(dirname, toMCTSFilename(domain, instanceId, budget,horizon, rep))
                costs.append(dict[filename]["cost"])
                times.append(dict[filename]["elapsed_time"])

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

def get_xy_mcts_full(filenameMerged, domain, instanceId, horizon, n):
    dirname = (os.path.dirname(filenameMerged))
    with open(filenameMerged) as f:
        str = f.read()
        dict = json.loads(str)

        xs_mcts_flat = []
        ys_mcts_flat = []
        errs_mcts_flat = []
        for budget in budgets:
            costs = []
            times = []
            for rep in range(0, n):
                filename = os.path.join(dirname, toMCTSFullFilename(domain, instanceId, budget,horizon, rep))
                costs.append(dict[filename]["cost"])
                times.append(dict[filename]["elapsed_time"])

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

# def get_xy_grid_vi(filenameMerged, domain, instanceId, horizon):
#     dirname = (os.path.dirname(filenameMerged))
#     with open(filenameMerged) as f:
#         str = f.read()
#         dict = json.loads(str)
# 
#         xs = []
#         ys = []
#         for nBin in nBins:
#             filename = os.path.join(dirname, toGridVIFilename(domain, instanceId, nBin, horizon))
#             cost = (dict[filename]["cost"])
#             time = dict[filename]["elapsed_time"]
# 
#             if cost == None or time == None:
#                 continue
#             xs.append(time)
#             ys.append(cost)
#         return xs, ys

def get_xy_rtdp(filenameMerged, domain, instanceId, horizon, nBin, n):
    dirname = (os.path.dirname(filenameMerged))
    with open(filenameMerged) as f:
        str = f.read()
        dict = json.loads(str)

        xs = []
        ys = []
        errs = []
        for nTrial in nTrials:
            costs = []
            times = []
            for rep in range(0, n):
                filename = os.path.join(dirname, toRTDPFilename(domain, instanceId, nBin, nTrial, horizon, rep))
                costs.append(dict[filename]["cost"])
                times.append(dict[filename]["elapsed_time"])
            if any([item == None for item in costs]):
                continue
            xs.append(statistics.mean(times))
            ys.append(statistics.mean(costs))

            if len(costs) > 1:
                errs.append(statistics.stdev(costs))
            else:
                errs.append(0.0)
        return xs, ys, errs

def get_xy_lrtdp(filenameMerged, domain, instanceId, horizon, nBin, n):
    dirname = (os.path.dirname(filenameMerged))
    with open(filenameMerged) as f:
        str = f.read()
        dict = json.loads(str)

        xs = []
        ys = []
        errs = []
        for nTrial in nTrials:
            costs = []
            times = []
            for rep in range(0, n):
                filename = os.path.join(dirname, toLRTDPFilename(domain, instanceId, nBin, nTrial, horizon, rep))
                costs.append(dict[filename]["cost"])
                times.append(dict[filename]["elapsed_time"])
            if any([item == None for item in costs]):
                continue
            xs.append(statistics.mean(times))
            ys.append(statistics.mean(costs))
            if len(costs) > 1:
                errs.append(statistics.stdev(costs))
            else:
                errs.append(0.0)
        return xs, ys, errs

def get_xy_lrtdp_d(filenameMerged, domain, instanceId, horizon, nBin, n):
    dirname = (os.path.dirname(filenameMerged))
    with open(filenameMerged) as f:
        str = f.read()
        dict = json.loads(str)

        xs = []
        ys = []
        errs = []
        for nTrial in nTrials:
            costs = []
            times = []
            for rep in range(0, n):
                filename = os.path.join(dirname, toLRTDPDFilename(domain, instanceId, nBin, nTrial, horizon, rep))
                costs.append(dict[filename]["cost"])
                times.append(dict[filename]["elapsed_time"])
            if any([item == None for item in costs]):
                continue
            xs.append(statistics.mean(times))
            ys.append(statistics.mean(costs))

            if len(costs) > 1:
                errs.append(statistics.stdev(costs))
            else:
                errs.append(0.0)
        return xs, ys, errs

def get_xy_rtdp_d(filenameMerged, domain, instanceId, horizon, nBin, n):
    dirname = (os.path.dirname(filenameMerged))
    with open(filenameMerged) as f:
        str = f.read()
        dict = json.loads(str)

        xs = []
        ys = []
        errs = []
        for nTrial in nTrials:
            costs = []
            times = []
            for rep in range(0, n):
                filename = os.path.join(dirname, toRTDPDFilename(domain, instanceId, nBin, nTrial, horizon, rep))
                costs.append(dict[filename]["cost"])
                times.append(dict[filename]["elapsed_time"])
            if any([item == None for item in costs]):
                continue
            xs.append(statistics.mean(times))
            ys.append(statistics.mean(costs))

            if len(costs) > 1:
                errs.append(statistics.stdev(costs))
            else:
                errs.append(0.0)
        return xs, ys, errs

def plot(filenameMerged, domain, instanceId, horizon, n):
    cmap1 = get_cmap("Set1")
    plt.clf()
    plt.tight_layout()
    plt.xticks(fontsize=18)
    plt.yticks(fontsize=18)
    plt.xscale('log')
#     plt.xlabel("Num Iterations", fontsize=18)
    plt.xlabel("Time (s)", fontsize=18)
    plt.ylabel("Cost", fontsize=18)

#     xs_mcts_flat,ys_mcts_flat,errs_mcts_flat = get_xy_mcts(filenameMerged, domain, instanceId, horizon, n)
#     plt.errorbar(xs_mcts_flat, ys_mcts_flat, errs_mcts_flat, label="UCT", color=cmap1.colors[0], marker='+')

    xs_mcts_full,ys_mcts_full,errs_mcts_full = get_xy_mcts_full(filenameMerged, domain, instanceId, horizon, n)
    plt.errorbar(xs_mcts_full, ys_mcts_full, errs_mcts_full, label="UCT", color=cmap1.colors[0], marker='+',linestyle="solid")
# 
    for i, nBin in enumerate([4, 8]):
        xs_rtdp,ys_rtdp, errs = get_xy_rtdp_d(filenameMerged, domain, instanceId, horizon, nBin, n)
        print((xs_rtdp, ys_rtdp))
        plt.errorbar(xs_rtdp, ys_rtdp, errs, label="RTDP K={}".format(nBin), color=cmap1.colors[1 + i], marker='.',linestyle="dashdot")

    for i, nBin in enumerate([4, 8]):
        xs_rtdp,ys_rtdp, errs = get_xy_lrtdp_d(filenameMerged, domain, instanceId, horizon, nBin, n)
        print((xs_rtdp, ys_rtdp))
        plt.errorbar(xs_rtdp, ys_rtdp, errs, label="LRTDP K={}".format(nBin), color=cmap1.colors[1 + i], marker='x', linestyle="dashed")

    plt.grid()
#     plt.legend(prop={"size": 18}, bbox_to_anchor=(1.04, 1.0), loc="upper left")
    plt.legend(prop={"size": 18}, loc="upper right")
    dirname = (os.path.dirname(args.filename))
    tree_hash = os.path.basename(dirname)
    os.makedirs("figs", exist_ok=True)
    plt.savefig("figs/{}/{}_online_{}.png".format(tree_hash, domain, instanceId), bbox_inches = "tight")

if __name__ == '__main__':
    import argparse
    cli = argparse.ArgumentParser()
    cli.add_argument(
            "filename",
            type=str,
            default=""
    )
    cli.add_argument(
            "domain",
            type=str,
            default="baker"
    )
    cli.add_argument(
            "instanceId",
            type=int,
            default=1
    )
    cli.add_argument(
            "--horizon",
            type=int,
            default=20
    )
    cli.add_argument(
            "--n",
            type=int,
            default=5
    )
    args = cli.parse_args()
    print(args, file=sys.stderr)

    plot(args.filename, args.domain, args.instanceId, args.horizon, args.n)