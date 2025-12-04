import copy
INPUT_FILE = 'input.txt'

with open(INPUT_FILE, 'r') as f:
    data = f.readlines()

def check_neighbors(grid, x, y):
    paper_neighbors = 0
    if grid[y-1][x-1] == '@':
        paper_neighbors += 1
    if grid[y-1][x] == '@':
        paper_neighbors += 1
    if grid[y-1][x+1] == '@':
        paper_neighbors += 1
    if grid[y][x+1] == '@':
        paper_neighbors += 1
    if grid[y+1][x+1] == '@':
        paper_neighbors += 1
    if grid[y+1][x] == '@':
        paper_neighbors += 1
    if grid[y+1][x-1] == '@':
        paper_neighbors += 1
    if grid[y][x-1] == '@':
        paper_neighbors += 1
    return paper_neighbors
    
def part1():
    solution = 0
    grid = ['.' * (len(data[0])+2)]
    for line in data:
        grid.append('.' + line + '.')
    grid.append('.' * len(grid[0]))

    for y in range(1, len(grid) - 1):
        for x in range(1, len(grid[0]) - 1):
            if grid[y][x] != '@':
                continue
            paper_neighbors = check_neighbors(grid, x, y)
            if paper_neighbors < 4:
                solution += 1
    return solution

def part2():
    solution = 0
    grid = [['.'] * (len(data[0])+2)]
    for line in data:
        grid.append(list('.' + line + '.'))
    grid.append(['.'] * len(grid[0]))

    prev_removed = -1
    new_grid = copy.deepcopy(grid)
    while True:
        removed = 0
        for y in range(1, len(grid) - 1):
            for x in range(1, len(grid[0]) - 1):
                if grid[y][x] != '@':
                    continue
                paper_neighbors = check_neighbors(grid, x, y)
                if paper_neighbors < 4:
                    removed += 1
                    new_grid[y][x] = '.'
        if removed == 0:
            break
        solution += removed
        grid = new_grid
        new_grid = copy.deepcopy(grid)
    return solution

print(part1())
print(part2())