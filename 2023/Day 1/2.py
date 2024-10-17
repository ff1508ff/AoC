total = 0
digit_dict = {
    'nine': '9',
    'eight': '8',
    'seven': '7',
    'six': '6',
    'five': '5',
    'four': '4',
    'three': '3',
    'two': '2',
    'one': '1',
}

def FindNumber(line):
    first_number = None
    last_number = None

    for word, digit in digit_dict.items():
        line = line.replace(word, word + digit + word)

    print(line)

    for char in line:
        if char.isdigit():
            if first_number is None:
                first_number = int(char)
            last_number = int(char)

    number = first_number*10 + last_number
    print(number)
    return number

with open('C:\dev\Project\AoC\Day 1\input.txt', 'r') as file:
    for line in file:
        line = line.strip()
        number = FindNumber(line)
        total += number

print(total)
