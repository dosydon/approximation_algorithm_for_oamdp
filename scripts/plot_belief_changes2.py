import matplotlib.pyplot as plt
import matplotlib
beliefChanges = [[(0.2), (0.2), (0.2), (0.2), (0.2)],
[(0.20404714), (0.19937466), (0.1979198), (0.19652507), (0.20213328)],
[(0.2207715), (0.1928253), (0.18981965), (0.180177), (0.21640658)],
[(0.24775274), (0.17876555), (0.17503878), (0.15677907), (0.24166387)],
[(0.2654463), (0.16833948), (0.16535544), (0.14217578), (0.25868306)],
[(0.26815912), (0.16673079), (0.16395664), (0.13983487), (0.26131856)],
[(0.29901218), (0.15113212), (0.15007462), (0.10855921), (0.29122186)],
[(0.36734277), (0.09429702), (0.1116023), (0.06898566), (0.3577723)]]

print(beliefChanges)

matplotlib.rcParams.update({'font.size': 16})
plt.xlabel('Time')
plt.ylabel('Observer\'s Belief')
plt.ylim(0, 1)
plt.plot(range(0, len(beliefChanges)), [belief[0] for belief in beliefChanges], label='A')
plt.plot(range(0, len(beliefChanges)), [belief[1] for belief in beliefChanges], label='B')
plt.plot(range(0, len(beliefChanges)), [belief[2] for belief in beliefChanges], label='C')
plt.plot(range(0, len(beliefChanges)), [belief[3] for belief in beliefChanges], label='D')
plt.plot(range(0, len(beliefChanges)), [belief[4] for belief in beliefChanges], label='E')
plt.legend()
plt.savefig('belief_changes.png')
