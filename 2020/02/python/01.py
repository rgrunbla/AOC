#!/usr/bin/env python3

valid = 0

with open("../input", "r") as f:
    for line in f.read().splitlines():
        policy, password = line.split(":")
        bounds,letter = policy.split(" ")
        mini,maxi=bounds.split("-")
        count = password.count(letter)
        if int(mini) <= count and count <= int(maxi):
            valid += 1
    print(valid)
