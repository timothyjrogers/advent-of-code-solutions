INPUT_FILE = 'input.txt'

with open(INPUT_FILE, 'r') as f:
    data = f.read().splitlines()

graph = {}

for line in data:
    node = line[:3]
    connections = line.split(': ')[1].split()
    graph[node] = connections

def part1():
    solution = 0
    queue = ['you']
    while len(queue) > 0:
        cur = queue.pop(0)
        if cur == 'out':
            solution += 1
            continue
        for connection in graph[cur]:
            queue.append(connection)
    return solution

def part2():
    # Find all paths from dac -> fft
    queue = [('dac', [])]
    dac_fft_paths = 0
    seen = set()
    while len(queue) > 0:
        cur, history = queue.pop(0)
        if cur == 'out' :
            continue
        if cur == 'fft':
            dac_fft_paths += 1
            continue
        if cur in seen:
            continue
        seen.add(cur)
        for connection in graph[cur]:
            queue.append((connection, history + [cur]))

    # Find all paths from fft -> dac
    queue = [('fft', [])]
    fft_dac_paths = 0
    seen = set()
    while len(queue) > 0:
        cur, history = queue.pop(0)
        if cur == 'out' :
            continue
        if cur == 'dac':
            fft_dac_paths += 1
            continue
        if cur in seen:
            continue
        seen.add(cur)
        for connection in graph[cur]:
            queue.append((connection, history + [cur]))

    # Find all paths from dac -> out or fft -> out, depending on intermediate paths
    '''
    if dac_fft_paths > 0:
        queue = [('fft', [])]
    else:
        queue = [('dac', [])]
    '''
    queue = [('dac', [])]
    out_paths = 0
    seen = set()
    while len(queue) > 0:
        cur, history = queue.pop(0)
        if cur == 'out':
            out_paths += 1
            continue
        if cur in seen:
            continue
        seen.add(cur)
        for connection in graph[cur]:
            queue.append((connection, history + [cur]))

    # Find all paths from svr -> fft or svr -> dac based on intermedaite paths
    queue = [('svr', [])]
    svr_paths = 0
    seen = set()
    while len(queue) > 0:
        cur, history = queue.pop(0)
        if cur == 'out' or cur == 'dac':#or ((cur == 'dac' and dac_fft_paths == 0) or (cur == 'fft' and fft_dac_paths == 0)):
            continue
        if cur == 'fft':#(cur == 'fft' and fft_dac_paths > 0) or (cur == 'dac' and dac_fft_paths > 0):
            svr_paths += 1
            continue
        if cur in seen:
            continue
        seen.add(cur)
        for connection in graph[cur]:
            queue.append((connection, history + [cur]))

    print(svr_paths)
    print(dac_fft_paths)
    print(fft_dac_paths)
    print(out_paths)
    return svr_paths * max(1, dac_fft_paths) * max(1, fft_dac_paths) * out_paths

#print(part1())
print(part2())