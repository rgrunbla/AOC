#!/usr/bin/env python3

with open("../input", "r") as f:
    all_seats = set()
    for i, line in enumerate(f.read().splitlines()):
        sum_ = "".join(list(line.replace('F', '0').replace('B', '1').replace('L', '0').replace('R', '1')))
        all_seats.add(sum_)
        
    for seat in range(max(all_seats)):
        if seat-1 in all_seats and seat+1 in all_seats and seat not in all_seats:
            print(seat)
