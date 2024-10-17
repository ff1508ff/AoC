#include <stdio.h>

int main (int argc, char *argv[]) {
    FILE* fptr = fopen("../input.txt", "r");
    if (fptr == NULL) {
        printf("File not found\n");
        return 1;
    }

    int floor = 0;
    for (char c = fgetc(fptr); c != EOF; c = fgetc(fptr)) {
        if (c == '(') {
            floor++;
        } else if (c == ')') {
            floor--;
        }
    }
    printf("He reaches floor: %d\n", floor);
    return 0;
}
