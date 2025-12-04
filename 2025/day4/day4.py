
def part1():
    file = open("2025/day4/day4.txt", "r")
    first = file.readline()
    lines = ['.' * (len(first) + 2)]
    lines.append('.' + first + '.')
    for line in file.readlines():
        lines.append('.' + line + '.')
    lines.append(['.' * (len(first) + 2)])
    count = 0
    for line in range(1, len(lines)-1):
        for block in range(1, len(lines[line])-1):
            if paper(lines[line][block]) == 0:
                continue 
            around = 0
            for i in lines[line-1][block-1:block+2]:
                around += paper(i)
            for i in lines[line+1][block-1:block+2]:
                around += paper(i)
            around += paper(lines[line][block-1]) + paper(lines[line][block+1])
            if around < 4:
                count += 1
    print(count)

def part2():
    file = open("2025/day4/day4.txt", "r")
    first = file.readline()
    lines = ['.' * (len(first) + 2)]
    lines.append('.' + first + '.')
    for line in file.readlines():
        lines.append('.' + line + '.')
    lines.append(['.' * (len(first) + 2)])
    count = 0
    changed = True
    while changed:
        changed = False
        for line in range(1, len(lines)-1):
            for block in range(1, len(lines[line])-1):
                if paper(lines[line][block]) == 0:
                    continue 
                around = 0
                for i in lines[line-1][block-1:block+2]:
                    around += paper(i)
                for i in lines[line+1][block-1:block+2]:
                    around += paper(i)
                around += paper(lines[line][block-1]) + paper(lines[line][block+1])
                if around < 4:
                    count += 1
                    lines[line] = lines[line][:block] + '.' + lines[line][block+1:]
                    changed = True
    print(count)


def paper(x):
    return 1 if x == '@' else 0

if __name__ == "__main__":
    part1()
    part2()