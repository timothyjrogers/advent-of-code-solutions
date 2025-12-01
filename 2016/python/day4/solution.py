from collections import defaultdict
from functools import cmp_to_key

INPUT_FILE = 'input.txt'

with open(INPUT_FILE, 'r') as f:
    data = f.readlines()

def cmp_letter_counts(a, b):
    if a[1] > b[1]:
        return 1
    elif a[1] < b[1]:
        return -1
    if a[0] > b[0]:
        return -1
    elif a[0] < b[0]:
        return 1
    return 0

def part1():
    answer = 0
    for line in data:
        letter_counts = defaultdict(int)
        sector_id_string = ''
        checksum = ''
        phase = 0
        for c in line:
            if c.isdigit():
                phase = 1
            if c == '[':
                phase = 2
            if phase == 0 and c != '-':
                letter_counts[c] += 1
            elif phase == 1:
                sector_id_string += c
            elif phase == 2:
                if c == '[' or c == ']':
                    continue
                checksum += c
        frequencies = sorted(list(letter_counts.items()), key=cmp_to_key(cmp_letter_counts), reverse=True)
        calculated_checksum = ''.join([x[0] for x in frequencies][:5])
        if calculated_checksum == checksum:
            answer += int(sector_id_string)
    return answer

print(part1())