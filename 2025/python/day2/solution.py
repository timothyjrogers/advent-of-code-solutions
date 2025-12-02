import functools

INPUT_FILE = 'input.txt'

with open(INPUT_FILE, 'r') as f:
    data = f.readline()

def matches_rule(n):
    s = str(n)
    l = len(s)
    if l % 2 != 0:
        return False
    middle = int(l/2)
    if s[:middle] == s[middle:]:
        return True
    return False

@functools.cache
def factorize(n):
    if n == 1:
        return set()
    factors = set()
    for m in range(1, int(n**0.5) + 1):
        if n % m == 0:
            factors.add(m)
            if m*m != n and n/m != n:
               factors.add(n//m)
    return factors

def matches_rule2(n):
    s = str(n)
    l = len(s)
    sizes = list(factorize(l))
    for size in sizes:
        substring = s[:size]
        valid = True
        i = size
        while i < l:
            if s[i:i+size] != substring:
                valid = False
                break
            i += size
        if valid:
            return True
    return False


def part1():
    solution = 0
    for product_range in data.split(','):
        start, end = product_range.split('-')
        for n in range(int(start), int(end)+1):
            if matches_rule(n):
                solution += n
    return solution

def part2():
    solution = 0
    for product_range in data.split(','):
        start, end = product_range.split('-')
        for n in range(int(start), int(end)+1):
            if matches_rule2(n):
                solution += n
    return solution

print(part1())
print(part2())