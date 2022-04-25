#!/usr/bin/env python3
#
# Compute a histogram of the lengths of each line in stdin.

import sys

def histogram(things):
    frequencies = dict()

    for thing in things:
        count = frequencies.get(thing, 0)
        frequencies[thing] = count + 1

    return frequencies 

if __name__ == "__main__":
    print(histogram(map(lambda s: len(s.strip()), sys.stdin)))
