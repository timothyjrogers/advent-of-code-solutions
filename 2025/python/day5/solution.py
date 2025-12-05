INPUT_FILE = 'input.txt'

with open(INPUT_FILE, 'r') as f:
    data = list(map(str.strip, f.readlines()))

def part1():
    solution = 0
    ranges = []
    section = 0
    for line in data:
        if line == '':
            section = 1
            continue
        if section == 0:
            start, end = line.split('-')
            ranges.append((int(start), int(end)))
        else:
            fresh = False
            item = int(line)
            for r in ranges:
                if r[0] <= item and item <= r[1]:
                    fresh = True
                    break
            if fresh:
                solution += 1
    return solution

def part2():
    solution = 0
    ranges = []
    for line in data:
        if line == '':
            break
        start, end = line.split('-')
        ranges.append([int(start), int(end)])
    ranges.sort()
    merged_ranges = [ranges[0]]
    
    for i in range(1, len(ranges)):
        end = merged_ranges[-1]
        nxt = ranges[i]
        if nxt[0] <= end[1]:
            end[1] = max(nxt[1], end[1])
        else:
            merged_ranges.append(nxt)
    
    for r in merged_ranges:
        solution += r[1] - r[0] + 1

    return solution

print(part1())
print(part2())