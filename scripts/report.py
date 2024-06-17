import sys
import json
import os
import statistics
from collections import defaultdict
from json2html import *
import argparse

def toGridVIFilename(domain, assignmentId, nBin, horizon, rep):
    return "grid_vi_{}_{}_{}_{}_{}.json".format(domain, assignmentId, nBin, horizon, rep)

def toRTDPFilename(domain, assignmentId, nBin, nTrial, horizon, rep):
    return "rtdp_{}_{}_{}_{}_{}_{}.json".format(domain, assignmentId, nBin, nTrial, horizon, rep)

def toLRTDPFilename(domain, assignmentId, nBin, nTrial, horizon, rep):
    return "lrtdp_{}_{}_{}_{}_{}_{}.json".format(domain, assignmentId, nBin, nTrial, horizon, rep)

def toLRTDPDFilename(domain, assignmentId, nBin, nTrial, horizon, rep):
    return "lrtdp_d_{}_{}_{}_{}_{}_{}.json".format(domain, assignmentId, nBin, nTrial, horizon, rep)

def toRTDPDFilename(domain, assignmentId, nBin, nTrial, horizon, rep):
    return "rtdp_d_{}_{}_{}_{}_{}_{}.json".format(domain, assignmentId, nBin, nTrial, horizon, rep)

def toMCTSFilename(domain, instanceId, nExpansion, horizon, rep):
    return "mcts_{}_{}_{}_{}_{}.json".format(domain, instanceId, nExpansion, horizon, rep)

def mean(x):
    if len(x) > 0:
        if any([item == None for item in x]):
            return None
        else:
            return statistics.mean(x)
    else:
        return None

if __name__ == "__main__":
    cli = argparse.ArgumentParser()
    cli.add_argument(
            "--horizons",
            nargs="*",
            type=str,
            default=[3, 5, 7]
    )
    cli.add_argument(
            "--num_bins",
            nargs="*",
            type=str,
            default=[10]
    )
    cli.add_argument(
            "--ids",
            nargs="*",
            type=str,
            default=[101, 102, 104, 901]
    )
    cli.add_argument(
            "--n_trials",
            nargs="*",
            type=str,
            default=[0, 100, 1000, 10000, 50000, 100000, 500000, 1000000]
    )
    cli.add_argument(
            "--num_rep",
            type=int,
            default=5
    )
    cli.add_argument(
            "filename",
            type=str,
            default=[]
    )
    cli.add_argument(
            "domain",
            type=str,
            default=""
    )
    args = cli.parse_args()
    print(args, file=sys.stderr)
    filenameMerged = args.filename

    dirname = (os.path.dirname(filenameMerged))
    nBins = args.num_bins
    result = [] 
    domain = args.domain

    with open(filenameMerged) as f:
        str = f.read()
        dict = json.loads(str)

        for assignmentId in args.ids:
            differentHorizons = {}
            for horizon in args.horizons:
                algorithms = []

#                 entries = []
#                 for nBin in nBins:
#                     costs = []
#                     times = []
#                     root_values = []
#                     num_states = []
#                     num_dom_states = []
#                     for i in range(0, args.num_rep):
#                         filename = os.path.join(dirname, toRTDPFilename(domain, assignmentId, nBin, 0, horizon, i))
#                         costs.append(dict[filename]["legibility_cost"])
#                         root_values.append(dict[filename]["root_value"])
#                         times.append(dict[filename]["elapsed_time"])
#                         num_states.append(dict[filename]["num_states"])
#                         num_dom_states.append(dict[filename]["num_domain_states"])
# 
#                     entry = {
#                             "elapsedTime": mean(times),
#                             "legibility cost": mean(costs),
#                             "root value": mean(root_values),
#                             "nBin": nBin,
#                             "num states": mean(num_states),
#                             "num domain states": mean(num_dom_states),
#                             }
#                     entries.append(entry)
#                 algorithms.append({"RTDP": entries})

#                 entries = []
#                 for nBin in nBins:
#                     costs = []
#                     root_values = []
#                     times = []
#                     num_states = []
#                     num_dom_states = []
#                     for i in range(0, args.num_rep):
#                         filename = os.path.join(dirname, toRTDPDFilename(domain, assignmentId, nBin, 0, horizon, i))
#                         costs.append(dict[filename]["legibility_cost"])
#                         root_values.append(dict[filename]["root_value"])
#                         times.append(dict[filename]["elapsed_time"])
#                         num_states.append(dict[filename]["num_states"])
#                         num_dom_states.append(dict[filename]["num_domain_states"])
# 
#                     entry = {
#                             "elapsedTime": mean(times),
#                             "legibility cost": mean(costs),
#                             "root value": mean(root_values),
#                             "nBin": nBin,
#                             "num states": mean(num_states),
#                             "num domain states": mean(num_dom_states),
#                             }
#                     entries.append(entry)
#                 algorithms.append({"RTDPD": entries})

                entries = []
                for nBin in nBins:
                    costs = []
                    times = []
                    root_values = []
                    num_states = []
                    num_dom_states = []
                    for i in range(0, args.num_rep):
                        filename = os.path.join(dirname, toLRTDPFilename(domain, assignmentId, nBin, 0, horizon, i))
                        costs.append(dict[filename]["legibility_cost"])
                        root_values.append(dict[filename]["root_value"])
                        times.append(dict[filename]["elapsed_time"])
                        num_states.append(dict[filename]["num_states"])
                        num_dom_states.append(dict[filename]["num_domain_states"])

                    entry = {
                            "elapsedTime": mean(times),
                            "legibility cost": mean(costs),
                            "root value": mean(root_values),
                            "nBin": nBin,
                            "num states": mean(num_states),
                            "num domain states": mean(num_dom_states),
                            }
                    entries.append(entry)
                algorithms.append({"LRTDP": entries})

                entries = []
                for nBin in nBins:
                    costs = []
                    times = []
                    root_values = []
                    num_states = []
                    num_dom_states = []
                    for i in range(0, args.num_rep):
                        filename = os.path.join(dirname, toLRTDPDFilename(domain, assignmentId, nBin, 0, horizon, i))
                        costs.append(dict[filename]["legibility_cost"])
                        root_values.append(dict[filename]["root_value"])
                        times.append(dict[filename]["elapsed_time"])
                        num_states.append(dict[filename]["num_states"])
                        num_dom_states.append(dict[filename]["num_domain_states"])

                    entry = {
                            "elapsedTime": mean(times),
                            "legibility cost": mean(costs),
                            "root value": mean(root_values),
                            "nBin": nBin,
                            "num states": mean(num_states),
                            "num domain states": mean(num_dom_states),
                            }
                    entries.append(entry)
                algorithms.append({"LRTDPD": entries})

                entries = []
                for nBin in nBins:
                    costs = []
                    times = []
                    root_values = []
                    num_states = []
                    num_dom_states = []
                    for i in range(0, args.num_rep):
                        filename = os.path.join(dirname, toGridVIFilename(domain, assignmentId, nBin, horizon, i))
                        costs.append(dict[filename]["legibility_cost"])
                        root_values.append(dict[filename]["root_value"])
                        times.append(dict[filename]["elapsed_time"])
                        num_states.append(dict[filename]["num_states"])
                        num_dom_states.append(dict[filename]["num_domain_states"])
                    entry = {
                            "elapsedTime": mean(times),
                            "legibility cost": mean(costs),
                            "root value": mean(root_values),
                            "nBin": nBin,
                            "num states": mean(num_states),
                            "num domain states": mean(num_dom_states),
                            }
                    entries.append(entry)
                algorithms.append({"GridVI": entries})

                differentHorizons["{}".format(horizon)] = algorithms

                result.append({
                        "domain": domain,
                        "assignmentId": assignmentId,
                        "differentHorizons": differentHorizons
                    })

    html = """<!DOCTYPE html>
<head>
  <style>
      li {{ display: inline; float: left }}
  </style>
</head>
<body>
{}
</body>""".format(json2html.convert(json = result))
    print(html)
