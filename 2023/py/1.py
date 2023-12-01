from pathlib import Path
import re


def part_1(doc):

    total = 0

    for line in doc.splitlines():
        value = get_value(line)
        total += value

    return total


def get_value(s: str) -> int:
    digits = re.findall(r"\d", s)

    number = digits[0] + digits[-1]

    return int(number)


def part_2(doc):
    total = 0

    for line in doc.splitlines():
        value = get_value_2(line)
        total += value

    return total


DIGITS = {
    w: str(d)
    for (w, d) in zip(
        ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"],
        range(1, 11),
    )
}


def get_value_2(s: str) -> int:
    digits = re.findall(rf"(?=(\d|{'|'.join(DIGITS)}))", s)

    first_and_last = []
    for index in 0, -1:
        val = digits[index]
        first_and_last.append(DIGITS.get(val, val))
    print(first_and_last, s, digits)

    number = "".join(first_and_last)

    return int(number)


if __name__ == "__main__":
    doc = (Path(__file__).parents[1] / "input/1").read_text()
    # print(part_1(doc))
    print(part_2(doc))
