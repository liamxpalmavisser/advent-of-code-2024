with open("../src/bin/example1.txt") as file:
    lines = file.read().splitlines()

def parse(lines: list[str]) -> tuple[list[int], list[int]]:
    gert = list(map(lambda x: x.split(" "), lines))
    first_column, last_column = ([int(n[0]) for n in gert], [int(n[-1]) for n in gert])
    return first_column, last_column

def find_diff(column1: list[int], column2: list[int]) -> int:
    column1 = sorted(column1)
    column2 = sorted(column2)
    return sum([abs(i - j) for i, j in zip(column1, column2)])

if __name__ == "__main__":
    column1, column2 = parse(lines)
    print(find_diff(column1, column2))
