from enum import Enum


def parse(file_path: str) -> list[list[int]]:
    with open(file_path) as file:
        lines = file.read().splitlines()
        return [list(map(int, line.split())) for line in lines]


class Trend(Enum):
    Up = 1
    Down = -1


def row_is_safe(row: list[int]) -> bool:
    trend = None

    for i, j in zip(row, row[1:]):
        diff = j - i
        if (diff == 0) | (abs(diff) > 3):
            return False
        
        new_trend = Trend(1 if diff > 0 else -1)
        if trend and trend != new_trend:
            return False
        
        trend = new_trend
    return True


def can_be_safe_by_removal(row: list[int]) -> bool:
    return any(row_is_safe(row[:i] + row[i + 1 :]) for i in range(len(row)))

def main():
    rows = parse("input.txt")

    n_safe = sum(row_is_safe(row) for row in rows)
    print(f"Part 1 solution {n_safe}")

    n_safe_with_removal = sum(can_be_safe_by_removal(row) for row in rows)
    print(f"Part 2 solution {n_safe_with_removal}")

if __name__ == "__main__":
    main()
