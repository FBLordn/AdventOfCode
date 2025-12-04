import re
import time

def part1():
    file = open("2025/day2/day2.txt", "r")
    sum = 0
    for val in [x.split("-") for x in file.readline().split(",")]:
        for i in [str(x) for x in range(int(val[0]), int(val[1])+1)]:
            if i[:(len(i)//2) + len(i)%2] == i[len(i)//2 + len(i)%2:]:
                sum += int(i)
    print(f"Part 1: {sum}")

def part2():
    file = open("2025/day2/day2.txt", "r")
    sum = 0
    for val in [x.split("-") for x in file.readline().split(",")]:
        for i in [str(x) for x in range(int(val[0]), int(val[1])+1)]:
            if re.search('^(\\d+)(\\1)+$', i):
                sum += int(i)
    print(f"Part 2: {sum}")
if __name__ == '__main__':
    start = time.time()
    part1()
    mid = time.time()
    part2()
    end = time.time()
    print("day 2 part 1: ", mid-start, " secs")
    print("day 2 part 1: ", end-mid, " secs")