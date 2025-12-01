INPUT_FILE = 'input.txt'

with open(INPUT_FILE, 'r') as f:
    data = f.readlines()

def part1():
    solution = 0
    dial = 50
    for step in data:
        direction = step[0]
        delta = int(step[1:]) if direction == 'R' else -1 * int(step[1:])
        dial = (dial + delta) % 100
        if dial == 0:
            solution += 1
    return solution

def part2():
    solution = 0
    dial = 50
    for step in data:
        direction = step[0]
        delta = int(step[1:])
        solution += int(delta / 100)
        remainder = delta % 100
        if direction == 'R':
            solution += 1 if dial != 0 and dial + remainder > 99 else 0
            dial = (dial + remainder) % 100
        else:
            solution += 1 if dial != 0 and dial - remainder <= 0 else 0
            dial = (dial - remainder) % 100
    return solution

print(part1())
print(part2())