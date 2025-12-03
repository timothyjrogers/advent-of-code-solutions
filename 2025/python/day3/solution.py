INPUT_FILE = 'input.txt'

with open(INPUT_FILE, 'r') as f:
    data = f.readlines()

def part1():
    solution = 0
    for line in data:
        batteries = list(map(int, line.strip()))
        tens = max(batteries[:-1])
        ones = max(batteries[batteries.index(tens)+1:])
        solution += tens*10 + ones
    return solution

def part2():
    solution = 0
    for line in data:
        batteries = list(map(int, line.strip()))
        digits = [0] * 12
        start = 0
        for i in range(12):
            end = len(batteries) - 11 + i
            if start > end:
                digits[i:] = batteries[start:]
                break
            battery = max(batteries[start:end])
            start += batteries[start:end].index(battery)+1
            digits[i] = battery
        joltage = int(''.join([str(x) for x in digits]))
        solution += joltage
    return solution

print(part1())
print(part2())