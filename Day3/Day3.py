import pandas as pd

with open('input') as f:
    data = [[int(c) for c in line.strip()] for line in f.readlines()]

frame = pd.DataFrame(data)
print(len(frame))
sums = frame.sum() / len(frame)

sums = list(sums.round())

print(sums)
gamma = sum([2**i * n for i, n in enumerate(reversed(sums))])
epsilon = sum([2**i * (1 - n) for i, n in enumerate(reversed(sums))])
print(gamma * epsilon)
