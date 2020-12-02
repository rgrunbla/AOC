#!/usr/bin/env python3

valid = 0

with open("../input", "r") as f:
    for line in f.read().splitlines():
        policy, password = line.split(":")
        bounds,letter = policy.split(" ")
        first,second=bounds.split("-")
        first=int(first)
        second=int(second)
        
        if (password[first] == letter) ^ (password[second] == letter):
            valid += 1
    print(valid)
