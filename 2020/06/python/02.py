#!/usr/bin/env python3

with open("../input", "r") as f:
    group = None
    sum_ = 0
    for i, line in enumerate(f.read().splitlines()):
        if not line:
            sum_ += len(group)
            group = None
        elif group == None:
            group = set(line)
        else:
            group = group.intersection(set(line))
    print(group)
    print(sum_)