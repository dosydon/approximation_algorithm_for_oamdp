import sys
import json
import os

if __name__ == "__main__":
    result = {}
    filename = sys.argv[1]
    with open(filename) as f:
        for line in f.readlines():
            line = line.strip('\n')
            for key in line.split(' '):
                try:
                    if len(line) <= 0:
                        continue
                    with open(key) as kf:
                        str = kf.read()
                        try:
                            dict = json.loads(str)
                            result[key] = dict
                        except:
                            sys.exit("Error loading json {}".format(key))
                except (FileNotFoundError, PermissionError, OSError):
                    sys.exit("Error opening file {}".format(key))
    print(json.dumps(result))
