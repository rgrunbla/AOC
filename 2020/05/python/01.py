#!/usr/bin/env python3

with open("../input", "r") as f:
    for i, line in enumerate(f.read().splitlines()):
        sum_ = "".join(list(line.replace('F', '0').replace('B', '1').replace('L', '0').replace('R', '1')))
        print(int(sum_, 2))