
def part1():
    file = open("2025/day3/day3.txt")
    count = 0
    for line in file.read().splitlines():
        first = max(line[:-1])
        second = max(line[line.index(first)+1:])
        count += 10*int(first) + int(second)
    print(count)

def part2():
    file = open("2025/day3/day3.txt")
    count = 0
    for line in file.read().splitlines():
        temp = 0
        batteries = [getHighestIndex(line,0,len(line)-12)]
        for i in range(12,1,-1):
            temp += int(line[batteries[-1]]) * pow(10,i-1)
            batteries.append(int(getHighestIndex(line, batteries[-1]+1, len(line)-i+2)))
        temp += int(line[batteries[-1]])
        count += temp
    print(count)

def getHighestIndex(list, start, end):
    max = start 
    for i in range(start, min(len(list), end)):
        if list[max] < list[i]:
            max = i
    return max

if __name__ == "__main__":
    #part1()
    part2()