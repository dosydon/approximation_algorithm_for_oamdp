#!/usr/bin/env python
import matplotlib.pyplot as plt
from matplotlib.cm import get_cmap
import statistics
import json
import os
import sys

def toGridVIFilename(domain, instanceId, nBin, horizon, rep):
    return "grid_vi_{}_{}_{}_{}_{}.json".format(domain, instanceId, nBin, horizon, rep)

def toRTDPFilename(domain, assignmentId, nBin, nTrial, horizon, rep):
    return "rtdp_{}_{}_{}_{}_{}_{}.json".format(domain, assignmentId, nBin, nTrial, horizon, rep)

def toLRTDPFilename(domain, assignmentId, nBin, nTrial, horizon, rep):
    return "lrtdp_{}_{}_{}_{}_{}_{}.json".format(domain, assignmentId, nBin, nTrial, horizon, rep)

def toLRTDPDFilename(domain, assignmentId, nBin, nTrial, horizon, rep):
    return "lrtdp_d_{}_{}_{}_{}_{}_{}.json".format(domain, assignmentId, nBin, nTrial, horizon, rep)

def toRTDPDFilename(domain, assignmentId, nBin, nTrial, horizon, rep):
    return "rtdp_d_{}_{}_{}_{}_{}_{}.json".format(domain, assignmentId, nBin, nTrial, horizon, rep)

budgets = [100, 500, 1000, 5000, 10000, 50000, 100000, 500000, 1000000, 5000000]
nBins = [1, 2, 4, 8, 16, 32]
nTrials = [100, 500, 1000, 10000, 50000, 100000, 500000, 1000000, 5000000]

def get_xy_grid_vi(filenameMerged, domain, instanceId, horizon, n):
    dirname = (os.path.dirname(filenameMerged))
    with open(filenameMerged) as f:
        str = f.read()
        dict = json.loads(str)
        xs = []
        ys = []
        errs = []
        for nBin in nBins:
            costs = []
            times = []
            for rep in range(0, n):
                filename = os.path.join(dirname, toGridVIFilename(domain, instanceId, nBin, horizon, rep))
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

def get_xy_lrtdp(filenameMerged, domain, instanceId, horizon, n):
    dirname = (os.path.dirname(filenameMerged))
    with open(filenameMerged) as f:
        str = f.read()
        dict = json.loads(str)

        xs = []
        ys = []
        errs = []
        for nBin in nBins:
            costs = []
            times = []
            for rep in range(0, n):
                filename = os.path.join(dirname, toLRTDPFilename(domain, instanceId, nBin, 0, horizon, rep))
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

def get_xy_lrtdp_d(filenameMerged, domain, instanceId, horizon, n):
    dirname = (os.path.dirname(filenameMerged))
    with open(filenameMerged) as f:
        str = f.read()
        dict = json.loads(str)

        xs = []
        ys = []
        errs = []
        for nBin in nBins:
            costs = []
            times = []
            for rep in range(0, n):
                filename = os.path.join(dirname, toLRTDPDFilename(domain, instanceId, nBin, 0, horizon, rep))
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
    plt.xlabel("Time (s)", fontsize=18)
    plt.ylabel("Cost", fontsize=18)


    xs_grid_vi,ys_grid_vi, errs = get_xy_grid_vi(filenameMerged, domain, instanceId, horizon, n)
    print((xs_grid_vi, ys_grid_vi))
    plt.errorbar(xs_grid_vi, ys_grid_vi, errs, label="GridVI", color=cmap1.colors[0], marker='+')
    for ((x, y), k) in zip(zip(xs_grid_vi, ys_grid_vi), nBins):
        plt.text(x, y, "{}".format(k))

    xs_rtdp,ys_rtdp, errs = get_xy_lrtdp(filenameMerged, domain, instanceId, horizon, n)
    print((xs_rtdp, ys_rtdp))
    plt.errorbar(xs_rtdp, ys_rtdp, errs, label="LRTDP", color=cmap1.colors[1], marker='+')
    for ((x, y), k) in zip(zip(xs_rtdp, ys_rtdp), nBins):
        plt.text(x, y, "{}".format(k))

    xs_rtdp,ys_rtdp, errs = get_xy_lrtdp_d(filenameMerged, domain, instanceId, horizon, n)
    print((xs_rtdp, ys_rtdp))
    plt.errorbar(xs_rtdp, ys_rtdp, errs, label="LRTDPD", color=cmap1.colors[2], marker='+', linestyle="dashed")
    for ((x, y), k) in zip(zip(xs_rtdp, ys_rtdp), nBins):
        plt.text(x, y, "{}".format(k))

    plt.grid()
#     plt.legend(prop={"size": 18}, bbox_to_anchor=(1.04, 1.0), loc="upper left")
    plt.legend(prop={"size": 18}, loc="upper right")
    dirname = (os.path.dirname(args.filename))
    tree_hash = os.path.basename(dirname)
    os.makedirs("figs", exist_ok=True)
    plt.savefig("figs/{}/{}_offline_{}.png".format(tree_hash, domain, instanceId), bbox_inches = "tight")

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