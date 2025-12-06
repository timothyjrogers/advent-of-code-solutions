INPUT_FILE = 'input.txt'

with open(INPUT_FILE, 'r') as f:
    data = f.read().splitlines()

def part1():
    solution = 0
    problems = []
    for action in data[-1].split():
        if action == '*':
            problems.append([1, '*'])
        else:
            problems.append([0, '+'])
    for line in data[:-1]:
        operands = map(int, line.split())
        for i, operand in enumerate(operands):
            if problems[i][1] == '*':
                problems[i][0] *= operand
            else:
                problems[i][0] += operand
    for problem in problems:
        solution += problem[0]
    return solution

def part2():
    solution = 0
    problems = []
    for action in data[-1].split():
        if action == '*':
            problems.append([1, '*'])
        else:
            problems.append([0, '+'])
    
    transposed_values = list(map(''.join,map(list, zip(*data[:-1]))))
    problem = 0
    for value in transposed_values:
        if value.isspace():
            problem += 1
            continue
        if problems[problem][1] == '*':
            problems[problem][0] *= int(value)
        else:
            problems[problem][0] += int(value)
    for problem in problems:
        solution += problem[0]
    return solution

print(part1())
print(part2())