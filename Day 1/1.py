total = 0

def FindNumber(line):
    first_number = None
    last_number = None

    for char in line:
        if char.isdigit():
            if first_number is None:
                first_number = int(char)
            last_number = int(char)
    number = first_number*10 + last_number
    return number

with open('C:\dev\Project\AoC\Day 1\input.txt', 'r') as file:
    for line in file:
        line = line.strip()
        number = FindNumber(line)
        total += number

print(total)
