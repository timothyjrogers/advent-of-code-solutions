INPUT_FILE = 'input.txt'

with open(INPUT_FILE, 'r') as f:
    data = f.readlines()

def part1():
    keypad = ['1','2','3','4','5','6','7','8','9']
    index = 4
    code = []
    for line in data:
        for step in line:
            match step:
                case 'U':
                    index = index - 3 if index > 2 else index
                case 'R':
                    index = index + 1 if index % 3 < 2 else index
                case 'D':
                    index = index + 3 if index < 6 else index
                case 'L':
                    index = index - 1 if index % 3 > 0 else index
        code.append(keypad[index])
    return ''.join(code)

def part2():
    keypad = ['','','1','','','','2','3','4','','5','6','7','8','9','','A','B','C','','','','D','','']
    index = 10
    code = []
    for line in data:
        for step in line:
            match step:
                case 'U':
                    index = index - 5 if (index > 4 and keypad[index - 5] != '') else index
                case 'R':
                    index = index + 1 if (index % 5 < 4 and keypad[index + 1] != '') else index
                case 'D':
                    index = index + 5 if (index < 20 and keypad[index + 5] != '') else index
                case 'L':
                    index = index - 1 if (index % 5 > 0 and keypad[index - 1] != '') else index
        code.append(keypad[index])
    return ''.join(code)

print(part1())
print(part2())