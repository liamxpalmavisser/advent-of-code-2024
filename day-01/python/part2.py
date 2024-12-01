from collections import Counter

with open("../src/bin/input.txt") as file:
    lines = file.read().splitlines()

def parse(lines: list[str]) -> tuple[list[int], list[int]]:
    gert = list(map(lambda x: x.split(" "), lines))
    first_column, last_column = ([int(n[0]) for n in gert], [int(n[-1]) for n in gert])
    return first_column, last_column

def get_new(column1: list[int], column2: list[int]) -> int:
    counter = Counter(column2)
    return sum(x * counter[x] for x in column1)

if __name__ == "__main__":
    column1, column2 = parse(lines)
    total_sum = get_new(column1, column2)
    breakpoint()
