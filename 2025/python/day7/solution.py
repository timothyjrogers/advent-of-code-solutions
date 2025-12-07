from collections import defaultdict

INPUT_FILE = 'input.txt'

with open(INPUT_FILE, 'r') as f:
    data = f.read().splitlines()

def part1():
    solution = 0
    grid = [list(line) for line in data if '^' in line or 'S' in line]
    beams = {}
    for i in range(0, len(grid)+1):
        beams[i] = defaultdict(int)
    beams[0][grid[0].index('S')] = 1
    for row in range(len(grid)):
        for beam in beams[row]:
            if grid[row][beam] == '^':
                solution += 1
                beams[row+1][beam+1] += beams[row][beam]
                beams[row+1][beam-1] += beams[row][beam]
            else:
                beams[row+1][beam] += beams[row][beam]
    return solution


def part2():
    solution = 0
    grid = [list(line) for line in data if '^' in line or 'S' in line]
    beams = {}
    for i in range(0, len(grid)+1):
        beams[i] = defaultdict(int)
    beams[0][grid[0].index('S')] = 1
    for row in range(len(grid)):
        for beam in beams[row]:
            if grid[row][beam] == '^':
                beams[row+1][beam+1] += beams[row][beam]
                beams[row+1][beam-1] += beams[row][beam]
            else:
                beams[row+1][beam] += beams[row][beam]
    for col in beams[len(grid)]:
        solution += beams[len(grid)][col]
    return solution

print(part1())
print(part2())