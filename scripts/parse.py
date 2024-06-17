import sys
import re
import json

if __name__ == "__main__":
    result = {}
    filename = sys.argv[1]
    with open(filename) as f:
        str = f.read()
        m = re.search("Elapsed time: (\d+\.\d+)s", str)
        if m:
            result["elapsed_time"] = float(m.groups(0)[0])
        else:
            result["elapsed_time"] = None
        m2 = re.search("Legibility Cost: (\d+\.\d+)", str)
        if m2:
            result["legibility_cost"] = float(m2.groups(0)[0])
        else:
            result["legibility_cost"] = None

        m3 = re.search("Cost: (-*\d+\.\d+)", str)
        if m3:
            result["cost"] = float(m3.groups(0)[0])
        else:
            result["cost"] = None

        m4 = re.search("Root Value: (\d+\.\d+)", str)
        if m4:
            result["root_value"] = float(m4.groups(0)[0])
        else:
            result["root_value"] = None

        m5 = re.search("Belief Simulated Value: (\d+\.\d+)", str)
        if m5:
            result["belief_simulated_value"] = float(m5.groups(0)[0])
        else:
            result["belief_simulated_value"] = None

        m6 = re.search("Num States: (\d+)", str)
        if m6:
            result["num_states"] = int(m6.groups(0)[0])
        else:
            result["num_states"] = None

        m7 = re.search("Num Domain States: (\d+)", str)
        if m7:
            result["num_domain_states"] = int(m7.groups(0)[0])
        else:
            result["num_domain_states"] = None

        m8 = re.search("Domain Simulated Value: (\d+\.\d+)", str)
        if m8:
            result["domain_simulated_value"] = float(m8.groups(0)[0])
        else:
            result["domain_simulated_value"] = None

        m9 = re.search("Domain Value: (\d+\.\d+)", str)
        if m9:
            result["domain_value"] = float(m9.groups(0)[0])
        else:
            result["domain_value"] = None

        m10 = re.search("num_lp_solved:(\d+)", str)
        if m10:
            result["num_lp_solved"] = float(m10.groups(0)[0])
        else:
            result["num_lp_solved"] = None
        print(json.dumps(result))
