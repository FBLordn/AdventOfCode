import math

def part1():
    file = open("2025/day8/day8.txt", "r")
    magic_number = 1000 # 10 for test 1000 for input
    coords = [[[int(x) for x in line.split(',')]] for line in file.readlines()] 
    compare = []
    for i in range(len(coords)-1):
        for j in range(i+1,len(coords)):
            compare.append([distance(coords[i][0], coords[j][0]), coords[i][0], coords[j][0]])
    for con in sorted(compare):
        l1 = [circ for circ in coords if con[1] in circ][0]
        l2 = [circ for circ in coords if con[2] in circ][0]
        if not l1 is l2:
            l1 += l2
            coords.remove(l2)
        magic_number -= 1
        if magic_number == 0:
            break
    print(math.prod(sorted([len(circ) for circ in coords], reverse=True)[:3]))

def part2():
    file = open("2025/day8/day8.txt", "r")
    coords = [[[int(x) for x in line.split(',')]] for line in file.readlines()] 
    compare = []
    for i in range(len(coords)-1):
        for j in range(i+1,len(coords)):
            compare.append([distance(coords[i][0], coords[j][0]), coords[i][0], coords[j][0]])
    for con in sorted(compare):
        l1 = [circ for circ in coords if con[1] in circ][0]
        l2 = [circ for circ in coords if con[2] in circ][0]
        if not l1 is l2:
            l1 += l2
            coords.remove(l2)
            if len(coords) == 1:
                print(con[1][0] * con[2][0])
                break

def distance(coord1, coord2):
    return math.sqrt((coord1[0]-coord2[0])**2+(coord1[1]-coord2[1])**2+(coord1[2]-coord2[2])**2)

if __name__ == "__main__":
    part1()
    part2()