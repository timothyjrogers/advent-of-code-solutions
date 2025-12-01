from enum import Enum

INPUT_FILE = 'input.txt'

with open(INPUT_FILE, 'r') as f:
    data = f.readline()

steps = data.split(', ')

class Facing(Enum):
    NORTH = 1,
    EAST = 2,
    SOUTH = 3,
    WEST = 4

def update_facing(facing, rotation):
    match facing:
        case Facing.NORTH:
            return Facing.WEST if rotation == 'L' else Facing.EAST
        case Facing.EAST:
            return Facing.NORTH if rotation == 'L' else Facing.SOUTH
        case Facing.SOUTH:
            return Facing.EAST if rotation == 'L' else Facing.WEST
        case Facing.WEST:
            return Facing.SOUTH if rotation == 'L' else Facing.NORTH

def part1():
    x = 0
    y = 0
    facing = Facing.NORTH
    for step in steps:
        rotation = step[0]
        distance = int(step[1:])
        facing = update_facing(facing, rotation)
        match facing:
            case Facing.NORTH:
                y += distance
            case Facing.EAST:
                x += distance
            case Facing.SOUTH:
                y -= distance
            case Facing.WEST:
                x -= distance
    return x + y

def part2():
    x = 0
    y = 0
    facing = Facing.NORTH
    seen = set((0,0))
    for step in steps:
        rotation = step[0]
        distance = int(step[1:])
        facing = update_facing(facing, rotation)
        match facing:
            case Facing.NORTH:
                for n in range(distance):
                    y += 1
                    if (x,y) in seen:
                        return x + y
                    seen.add((x,y))
            case Facing.EAST:
                for n in range(distance):
                    x += 1
                    if (x,y) in seen:
                        return x + y
                    seen.add((x,y))
            case Facing.SOUTH:
                for n in range(distance):
                    y -= 1
                    if (x,y) in seen:
                        return x + y
                    seen.add((x,y))
            case Facing.WEST:
                for n in range(distance):
                    x -= 1
                    if (x,y) in seen:
                        return x + y
                    seen.add((x,y))

print(part1())
print(part2())