import re

INPUT_FILE = 'input.txt'

with open(INPUT_FILE, 'r') as f:
    data = f.readlines()

def is_valid_triangle(sides):
    a = sides[0]
    b = sides[1]
    c = sides[2]
    return a + b > c and b + c > a and c + a > b

def part1():
    valid_triangles = 0
    for line in data:
        sides = list(map(int, re.findall(r'\d+', line)))
        sides.sort()
        if is_valid_triangle(sides):
            valid_triangles += 1
    return valid_triangles

def part2():
    valid_triangles = 0
    lists = [[],[],[]]
    for line in data:
        sides = list(map(int, re.findall(r'\d+', line)))
        lists[0].append(sides[0])
        lists[1].append(sides[1])
        lists[2].append(sides[2])
    i = 0
    while i < len(lists[0]) - 2:
        sides = [lists[0][i], lists[0][i+1], lists[0][i+2]]
        if is_valid_triangle(sides):
            valid_triangles += 1
        sides = [lists[1][i], lists[1][i+1], lists[1][i+2]]
        if is_valid_triangle(sides):
            valid_triangles += 1
        sides = [lists[2][i], lists[2][i+1], lists[2][i+2]]
        if is_valid_triangle(sides):
            valid_triangles += 1
        i += 3
    return valid_triangles

print(part1())
print(part2())