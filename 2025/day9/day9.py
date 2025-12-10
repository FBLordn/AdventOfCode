import time

def part1():
    file = open("2025/day9/test.txt", "r")
    biggest = 0
    coords  = [[int(x) for x in line.split(',')] for line in file]
    for i in range(len(coords)-1):
        for j in range(i+1, len(coords)):
            dist = (abs(coords[i][0]-coords[j][0])+1)*(abs(coords[i][1]-coords[j][1])+1)  
            if biggest < dist:
                biggest = dist 
    print(biggest)

def part2():
    file = open("2025/day9/day9.txt", "r")
    biggest = 0
    coords  = [[int(x) for x in line.split(',')] for line in file]
    borders = coords.copy()
    for i in range(len(coords)):
        j = coords[(i+1) % len(coords)]
        for x in range(coords[i][0],j[0], 1 if coords[i][0] < j[0] else -1):
            borders.append([x, j[1]])
        for y in range(coords[i][1],j[1], 1 if coords[i][1] < j[1] else -1):
            borders.append([j[0], y])
    biggest = []
    for i in range(len(coords)-1):
        for j in range(i+1, len(coords)):
            biggest.append([(abs(coords[i][0]-coords[j][0])+1)*(abs(coords[i][1]-coords[j][1])+1), coords[i], coords[j]])
    for dist, i, j in sorted(biggest, reverse=True):
        min_max_x = [i[0],j[0]] if i[0] < j[0] else [j[0],i[0]]
        min_max_y = [i[1],j[1]] if i[1] < j[1] else [j[1],i[1]]
        valid = True
        for x,y in borders:
            if x > min_max_x[0] and x < min_max_x[1] and y > min_max_y[0] and y < min_max_y[1]:
                valid = False
                break
        if valid:
            print(dist)
            break

if __name__ == "__main__":
    start = time.time()
    part1()
    mid = time.time()
    part2()
    end = time.time()
    print("day 9 part 1: ", (mid-start)*10000000, " ns")
    print("day 9 part 2: ", (end-mid)*1000000000, " ns")
