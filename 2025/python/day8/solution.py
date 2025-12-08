INPUT_FILE = 'input.txt'

with open(INPUT_FILE, 'r') as f:
    data = f.read().splitlines()

points = [list(map(int, x)) for x in map(lambda x: x.split(','), data)]

def euclidean_distance(p1, p2):
    return (p2[0] - p1[0])**2 + (p2[1] - p1[1])**2 + (p2[2] - p1[2])**2

def part1():
    solution = 1
    circuits = {}
    distances = []
    for i in range(len(points)):
        circuits[i] = {i}
    for i in range(len(points)-1):
        for j in range(i+1, len(points)):
            dist = euclidean_distance(points[i], points[j])
            distances.append((i, j, dist))
    distances.sort(key=lambda x: x[2])
    for i, j, distance in distances[:1000]:
        if i in circuits[j]:
            continue
        circuit = circuits[i] | circuits[j]
        for point in circuit:
            circuits[point] = circuit
    sizes = [len(v) for k,v in circuits.items() if k == min(v)]
    sizes.sort(reverse=True)
    for size in sizes[:3]:
        solution *= size
    return solution

def part2():
    solution = 1
    circuits = {}
    distances = []
    for i in range(len(points)):
        circuits[i] = {i}
    for i in range(len(points)-1):
        for j in range(i+1, len(points)):
            dist = euclidean_distance(points[i], points[j])
            distances.append((i, j, dist))
    distances.sort(key=lambda x: x[2])
    for i, j, distance in distances:
        if i in circuits[j]:
            continue
        circuit = circuits[i] | circuits[j]
        for point in circuit:
            circuits[point] = circuit
        sizes = [len(v) for k,v in circuits.items() if k == min(v)]
        if len(sizes) == 1:
            return points[i][0] * points[j][0]

print(part1())
print(part2())