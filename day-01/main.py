from collections import Counter

with open("./input.txt") as file:
    lines = file.read().splitlines()


def parse(lines: list[str]) -> tuple[list[int], list[int]]:
    metro = list(map(lambda x: x.split(" "), lines))
    first_column, last_column = (
        [int(n[0]) for n in metro],
        [int(n[-1]) for n in metro],
    )
    return first_column, last_column


def part1(column1: list[int], column2: list[int]) -> int:
    column1 = sorted(column1)
    column2 = sorted(column2)
    return sum([abs(i - j) for i, j in zip(column1, column2)])


def part2(column1: list[int], column2: list[int]) -> int:
    counter = Counter(column2)
    return sum(x * counter[x] for x in column1)


if __name__ == "__main__":
    column1, column2 = parse(lines)
    print(f"Part1 solution is {part1(column1, column2)}\n")
    print(f"Part2 solution is {part2(column1, column2)}\n")
