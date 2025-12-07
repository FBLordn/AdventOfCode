def part1():
    count = 0
    lines = [list(line) for line in open("2025/day7/day7.txt", "r").readlines()]
    for line in range(1, len(lines)):
        for char in range(len(lines[line])):
            if lines[line][char] == '^' and lines[line-1][char] == '|':
                lines[line][char-1:char+2] = ['|','^','|']
                count += 1
            if lines[line][char] == '.' and (lines[line-1][char] == '|' or lines[line-1][char] == 'S'):
                lines[line][char] = '|'
    print(count)

def part2():
    lines: list[list[str | int]] = [list(line) for line in open("2025/day7/day7.txt", "r").readlines()]
    for line in range(1, len(lines)):
        for char in range(len(lines[line])):
            above = lines[line-1][char]
            match lines[line][char]:
                case '^':
                    if isinstance(above, int):
                        lines[line][char-1] = above + (lines[line][char-1] if isinstance(lines[line][char-1], int) else 0)
                        lines[line][char+1] = above
                case '.':
                    if isinstance(above, int):
                        lines[line][char] = above
                    elif above == 'S':
                        lines[line][char] = 1
                case _:
                    if isinstance(above, int):
                        lines[line][char] += above
    print(sum([x for x in lines[-1] if isinstance(x, int)]))

if __name__ == "__main__":
    part1()
    part2()