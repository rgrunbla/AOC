#!/usr/bin/env python3

fields = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"}

def isvalid(field, passport_field):
    if field == 'byr':
        return passport_field.isnumeric() and 1920 <= int(passport_field) and int(passport_field) <= 2002
    elif field == 'iyr':
        return passport_field.isnumeric() and 2010 <= int(passport_field) and int(passport_field) <= 2020
    elif field == 'eyr':
        return passport_field.isnumeric() and 2020 <= int(passport_field) and int(passport_field) <= 2030
    elif field == 'hgt':
        if passport_field.endswith('cm'):
            passport_field = passport_field.rstrip('cm')
            return 150 <= int(passport_field) and int(passport_field) <= 193
        elif passport_field.endswith('in'):
            passport_field = passport_field.rstrip('in')
            return 59 <= int(passport_field) and int(passport_field) <= 76
    elif field == 'hcl':
        return len(passport_field) == 7 and passport_field[0] == '#' and set(passport_field[1::]).issubset(set(['a', 'b', 'c', 'd', 'e', 'd', 'f', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))
    elif field == 'ecl':
        return passport_field in set(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"])
    elif field == 'pid':
        return len(passport_field) == 9 and set(passport_field[1::]).issubset(set([str(a) for a in list(range(10))]))
    else:
        return True

invalid = 0
valid = 0
with open("../input", "r") as f:
    passport = {}
    for i, line in enumerate(f.read().splitlines()):
        if not line:
            missings = set()
            for field in fields:
                if field not in passport:
                    missings.add(field)
                else:
                    if not isvalid(field, passport[field]):
                        print("{} is not valid {}".format(field, passport[field]))
                        missings.add(field)
            if (len(missings) == 0) or (len(missings) == 1 and 'cid' in missings): # full passport, good
                valid += 1
            else: # invalid passport
                print(passport)
                invalid += 1
            passport = {}
        else:
            for keyval in line.split(" "):
                key, val = keyval.split(":")
                passport[key] = val
    print(valid)
    print(invalid)