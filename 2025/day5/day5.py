import time 

def part1():
    file = open("2025/day5/day5.txt", "r")
    ranges = []
    count = 0
    preparing = True
    for line in file.readlines():
        if preparing:
            if line == "\n":
                preparing = False
                continue
            ranges.append([int(x) for x in line.split('-')])
        else:
            count += 1 if in_range(ranges, int(line)) else 0
    print(count)

def part2():
    file = open("2025/day5/day5.txt", "r")
    edges = []
    count = -1
    lower = 0
    upper = 0
    for line in file.readlines():
        if line == '\n':
            break
        edges.append([int(x) for x in line.split('-')])
    edges.sort()
    for edge in edges:
        if edge[0] > upper:
            count += upper - lower + 1
            lower = edge[0]
        if edge[0] > upper or edge[1] > upper:
            upper = edge[1]
    count += upper - lower + 1
    print(count)

def in_range(ranges, value):
    for range in ranges:
        if range[0] <= value and range[1] >= value:
            return True
    return False

if __name__ == "__main__":
    start = time.time()
    part1()
    mid = time.time()
    part2()
    end = time.time()
    print("day 5 part 1: ", (mid-start)*10000000, " ns")
    print("day 5 part 2: ", (end-mid)*1000000000, " ns")