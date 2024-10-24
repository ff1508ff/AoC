#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    FILE *fptr = fopen("../input.txt", "r");
    if (fptr == NULL) {
        printf("File not found\n");
        return 1;
    }

    int niceWordCount = 0;
    char prevChar = ' ';
    char preprevChar = ' ';

    int **containsPair = malloc(26 * sizeof(int *));
    for (int i = 0; i < 26; i++) {
        containsPair[i] = malloc(26 * sizeof(int));
    }

    int repeatLetterWithOneBetween = 0;
    int doublePair = 0;

    for (char c = fgetc(fptr); c != EOF; c = fgetc(fptr)) {
        printf("%c\n", c);
        if (c == '\n') {
            if (repeatLetterWithOneBetween == 1 && doublePair == 1) {
                niceWordCount++;
            }

            repeatLetterWithOneBetween = 0;
            doublePair = 0;
            for (int i = 0; i < 26; i++) {
                for (int j = 0; j < 26; j++) {
                    containsPair[i][j] = 0;
                }
            }
            prevChar = ' ';
            preprevChar = ' ';
            continue;
        }

        if (prevChar != ' ') {
            if (containsPair[prevChar - 'a'][c - 'a'] == 1) {
                doublePair = 1;
            }

            for (int i = 0; i < 26; i++) {
                for (int j = 0; j < 26; j++) {
                    if (containsPair[i][j] == 2) {
                        containsPair[i][j] = 1;
                    }
                }
            }
            containsPair[prevChar - 'a'][c - 'a'] = 2;

            if (preprevChar == c) {
                repeatLetterWithOneBetween = 1;
            }
        }

        preprevChar = prevChar;
        prevChar = c;
    }
    // incase the last line is nice
    if (repeatLetterWithOneBetween == 1 && doublePair == 1) {
        niceWordCount++;
    }

    printf("There are %d nice strings\n", niceWordCount);

    for (int i = 0; i < 26; i++) {
        free(containsPair[i]);
    }
    free(containsPair);
    fclose(fptr);

    return 0;
}
