#!/usr/bin/env python3

with open("../input", "r") as f:
    group = set()
    sum_ = 0
    for i, line in enumerate(f.read().splitlines()):
        if not line:
            sum_ += len(group)
            group = set()
        group = group.union(set(line))
    sum_ += len(group)
    print(sum_)