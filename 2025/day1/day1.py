
def part1():
    file = open('2025/day1/day1.txt', "r")
    c = 0
    t = 50
    for line in file:
        k = line[0]
        r = int(line[1:])
        t = (t + r * (1 if k == 'R' else -1)) % 100
        if t == 0:
            c += 1
    print(f"Answer is: {c}")

def part2():
    file = open('2025/day1/day1.txt', "r")
    c = 0
    t = 50
    for line in file:
        r = int(line[1:])
        if line[0] == 'R':
            c += (t+r) // 100
        else:
            v = 100 if t==0 else t
            c += (100-v+r) // 100
            r = 0-r
        t = (t + r) % 100
    print(f"Answer is: {c}")


if __name__ == '__main__':
    part1()
    part2()