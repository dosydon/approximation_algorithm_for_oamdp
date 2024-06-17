import matplotlib.pyplot as plt
import matplotlib


beliefChanges = [
[(0.2), (0.2), (0.2), (0.2), (0.2)],
[(0.20066015), (0.20100056), (0.20066015), (0.19701895), (0.20066015)],
[(0.20094337), (0.20351002), (0.20094337), (0.19365987), (0.20094337)],
[(0.20305747), (0.21208945), (0.2030715), (0.17871012), (0.20307149)],
[(0.20156735), (0.2168545), (0.2016716), (0.17823492), (0.20167159)],
[(0.20795137), (0.18650045), (0.20805891), (0.1894303), (0.20805888)],
[(0.23408158), (0.14400671), (0.23420265), (0.1535065), (0.23420258)],
[(0.26969236), (0.08501767), (0.26983187), (0.10562631), (0.26983175)],
[(0.29442266), (0.04755945), (0.29457495), (0.068868175), (0.29457477)],
[(0.31022128), (0.025678065), (0.31038177), (0.0433374), (0.31038153)],
[(0.3777022), (0.013002513), (0.20794596), (0.023451952), (0.3778973)],
[(0.42762336), (0.00775512), (0.122789815), (0.0139875), (0.42784423)],
[(0.46375623), (0.003837502), (0.061489005), (0.006921499), (0.46399575)],
[(0.47975543), (0.002415629), (0.03346881), (0.0043569417), (0.48000315)],
[(0.48953447), (0.001385092), (0.01679493), (0.002498217), (0.48978722)],
[(0.5318177), (0.0006955708), (0.008034106), (0.0012545639), (0.458198)]]
print(beliefChanges)

matplotlib.rcParams.update({'font.size': 16})
plt.xlabel('Time')
plt.ylabel('Observer\'s Belief')
plt.xticks([0, 5, 10, 15])
plt.ylim(0, 1)
plt.plot(range(0, len(beliefChanges)), [belief[0] for belief in beliefChanges], label='A')
plt.plot(range(0, len(beliefChanges)), [belief[1] for belief in beliefChanges], label='B')
plt.plot(range(0, len(beliefChanges)), [belief[2] for belief in beliefChanges], label='C')
plt.plot(range(0, len(beliefChanges)), [belief[3] for belief in beliefChanges], label='D')
plt.plot(range(0, len(beliefChanges)), [belief[4] for belief in beliefChanges], label='E')
plt.legend()
plt.savefig('belief_changes.png')