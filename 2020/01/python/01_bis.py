#!/usr/bin/env python3

with open("../input", "r") as f:
    data = sorted([int(x) for x in f.read().splitlines()])
    i = 0
    j = len(data)-1
    while True:
        if (data[i]+data[j]) == 2020:
            print(data[i]*data[j])
            exit(0)
        elif data[i]+data[j] > 2020:
            j = j-1
        elif data[i]+data[j] < 2020:
            i = i+1
