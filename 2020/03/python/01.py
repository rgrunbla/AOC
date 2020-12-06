#!/usr/bin/env python3

tree = 0
free = 0

for slope in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]:
right = 3
down = 1
with open("../input", "r") as f:
    x = 0
    length = 0
    for i, line in enumerate(f.read().splitlines()):
        length = len(line)
        if (i % down) != 0:
            continue
        if line[x % length] == '.':
            free += 1
        else:
            tree += 1
        x += right

print("Tree: %d" % tree)
print("Free: %d" % free)
