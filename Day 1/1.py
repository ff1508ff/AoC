total = 0

def find_and_add_numbers(line):
    first_number = None
    last_number = None

    for char in line:
        if char.isdigit():
            if first_number is None:
                first_number = int(char)
            last_number = int(char)

    return first_number, last_number

with open('C:\dev\Project\AoC\Day 1\input.txt', 'r') as file:
    for line in file:
        line = line.strip()
        first_number, last_number = find_and_add_numbers(line)
        if first_number is not None:
            total += first_number
        if last_number is not None:
            total += last_number

print(total)
