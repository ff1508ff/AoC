#include <stdio.h>

int main(int argc, char *argv[]) {
    FILE *fptr = fopen("../input.txt", "r");
    if (fptr == NULL) {
        printf("File not found\n");
        return 1;
    }

    int niceWordCount = 0;
    char prevChar = ' ';

    int vowelsCount = 0;
    int doubleChar = 0;
    int forbidden = 0;

    for (char c = fgetc(fptr); c != EOF; c = fgetc(fptr)) {
        if (c == '\n') {
            if (vowelsCount >= 3 && doubleChar == 1 && forbidden == 0) {
                niceWordCount++;
            }

            vowelsCount = 0;
            prevChar = ' ';
            doubleChar = 0;
            forbidden = 0;
            continue;
        }

        if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
            vowelsCount++;
        }

        if (c == prevChar) {
            doubleChar = 1;
        }

        if ((prevChar == 'a' && c == 'b') || (prevChar == 'c' && c == 'd') ||
            (prevChar == 'p' && c == 'q') || (prevChar == 'x' && c == 'y')) {
            forbidden = 1;
        }
        prevChar = c;
    }
    printf("There are %d nice strings\n", niceWordCount);
    return 0;
}
