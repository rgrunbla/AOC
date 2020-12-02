#!/usr/bin/env python3

with open("../input", "r") as f:
    data = [int(x) for x in f.read().splitlines()]
    for i1 in data:
        for i2 in data:
            for i3 in data:
                if (i1+i2+i3 == 2020):
                    print(i1*i2*i3)
                    exit(0)
