import math
import time

def part1():
    file = open("2025/day6/day6.txt", "r")
    lines = [[int(value)] for value in file.readline().split()]
    for line in file.readlines():
        index = 0
        for num in line.split():
            lines[index].append(int(num) if num.isnumeric() else num)
            index += 1
    print(sum([sum(line[:-1]) if line[-1] == '+' else math.prod(line[:-1]) for line in lines]))

def part2():
    file = open("2025/day6/day6.txt", "r")
    lines = [[value] for value in file.readline()]
    for line in file.readlines():
        index = 0
        for num in line:
            lines[index].append(num)
            index += 1
    count = 0
    addition = None
    part = 0
    for line in lines:
        if addition == None:
            addition = line[-1] == '+'
            part = 0 if addition else 1
        line = [x for x in line if x.isnumeric()]
        if len(line) == 0:
            count += part 
            addition = None
        else:
            value = int(''.join(line))
            part = (part + value) if addition else (part * value)
    print(count)

if __name__ == "__main__":
    start = time.time()
    part1()
    mid = time.time()
    part2()
    end = time.time()
    print("day 6 part 1: ", (mid-start)*10000000, " ns")
    print("day 6 part 2: ", (end-mid)*1000000000, " ns")