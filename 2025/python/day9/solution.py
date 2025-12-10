import shapely

INPUT_FILE = 'input.txt'

with open(INPUT_FILE, 'r') as f:
    data = f.read().splitlines()

points = [list(map(int, x)) for x in map(lambda x: x.split(','), data)]

def part1():
    solution = 0
    for i in range(len(points)-1):
        for j in range(i+1, len(points)):
            area = (abs(points[i][0] - points[j][0])+1) * (abs(points[i][1] - points[j][1])+1)
            if area > solution:
                solution = area
    return solution

def part2():
    solution = 0
    shape = shapely.Polygon(points + [points[0]])
    for i in range(len(points)-1):
        for j in range(i+1, len(points)):
            if shapely.contains(shape, shapely.box(*points[i], *points[j])):
                area = (abs(points[i][0] - points[j][0])+1) * (abs(points[i][1] - points[j][1])+1)
                if area > solution:
                    solution = area
    return solution

print(part1())
print(part2())
