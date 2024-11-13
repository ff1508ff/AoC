#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

// I need this for memset
#include <string.h>

int main(int argc, char *argv[]) {
    FILE *fptr = fopen("../input.txt", "r");
    if (fptr == NULL) {
        printf("File not found\n");
        return 1;
    }

    int maxWidth = 0;
    int maxHeight = 0;

    for (char c = fgetc(fptr); c != EOF; c = fgetc(fptr)) {
        if (c == 'v' || c == '^') {
            maxHeight++;
            continue;
        }

        if (c == '<' || c == '>') {
            maxWidth++;
            continue;
        }
    }
    printf("maxHeight: %d, maxWidth: %d\n", maxHeight, maxWidth);
    // reset file pointer
    fseek(fptr, 0, SEEK_SET);

    // dynamic 2d array
    bool **map = malloc(maxHeight * sizeof(bool *));
    for (int i = 0; i < maxHeight; i++) {
        map[i] = malloc(maxWidth * sizeof(bool));
    }

    int newHouses = 1;

    int SantaX = maxWidth / 2;
    int SantaY = maxHeight / 2;
    int RobotX = maxWidth / 2;
    int RobotY = maxHeight / 2;
    map[SantaY][SantaX] = true;
    bool SantaTurn = true;

    for (char c = fgetc(fptr); c != EOF; c = fgetc(fptr)) {
        // I'm using a pointer, because it keeps things cleaner (I think)
        int *x = SantaTurn ? &SantaX : &RobotX;
        int *y = SantaTurn ? &SantaY : &RobotY;
        if (c == 'v') {
            (*y)--;
        } else if (c == '^') {
            (*y)++;
        } else if (c == '<') {
            (*x)--;
        } else if (c == '>') {
            (*x)++;
        }

        if (map[*y][*x] == false) {
            newHouses++;
            map[*y][*x] = true;
        }
        SantaTurn = !SantaTurn;
    }
    printf("Santa and the Robot visited %d unique houses\n", newHouses);

    for (int i = 0; i < maxHeight; i++) {
        free(map[i]);
    }
    free(map);
    fclose(fptr);
    return 0;
}
