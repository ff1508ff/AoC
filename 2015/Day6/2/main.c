#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    FILE *fptr = fopen("../input.txt", "r");
    if (fptr == NULL) {
        printf("File not found\n");
        return 1;
    }

    int **map = malloc(1000 * sizeof(int *));
    for (int i = 0; i < 1000; i++) {
        map[i] = calloc(1000, sizeof(int)); // calloc instead to set all values to 0
    }

    enum { NONE, ON, OFF, TOGGLE } action;

    struct coords {
        int x, y;
    };

    struct coords start = {-1, -1};
    struct coords end = {-1, -1};
    int number = -1;

    for (char c = fgetc(fptr); c != EOF; c = fgetc(fptr)) {
        if (action == NONE) {
            fseek(fptr, 5, SEEK_CUR);
            switch (fgetc(fptr)) {
            case 'n':
                action = ON;
                break;
            case 'f':
                action = OFF;
                break;
            case ' ':
                action = TOGGLE;
                break;
            default:
                action = NONE;
                printf("Error: unknown action\n");
                return 1;
            }
        }
        if (c == ' ' || c == ',' || c == '\n') {
            if (number >= 0) {
                if (start.x == -1) {
                    start.x = number;
                } else if (start.y == -1) {
                    start.y = number;
                } else if (end.x == -1) {
                    end.x = number;
                } else if (end.y == -1) {
                    end.y = number;
                } else {
                    printf("Error: too many numbers\n");
                    return 1;
                }
            }
            number = -1; // -1 to make sure that it is actually a new number when its used
            if (c == '\n') {
                if (start.x == -1 || start.y == -1 || end.x == -1 || end.y == -1) {
                    printf("Error: not enough numbers\n");
                    return 1;
                }

                // I expect that the second number is bigger than the first
                for (int x = start.x; x <= end.x; x++) {
                    for (int y = start.y; y <= end.y; y++) {
                        // also possible is:
                        // map[y][x] = action == ON || (action == TOGGLE && !map[y][x]);
                        // but I think this is easier to read and has better error handling

                        switch (action) {
                        case ON:
                            map[y][x] += 1;
                            break;
                        case OFF:
                            if (map[y][x] > 0)
                                map[y][x] -= 1;
                            break;
                        case TOGGLE:
                            map[y][x] += 2;
                            break;
                        default:
                            printf("Error: unknown action\n");
                            return 1;
                        }
                    }
                }

                // reset variables
                start = (struct coords){-1, -1};
                end = (struct coords){-1, -1};
                action = NONE;
            }
            continue;
        }
        if (c >= '0' && c <= '9') {
            if (number == -1) {
                number = 0;
            }
            number = number * 10 + (c - '0');
        }
    }

    number = 0;
    for (int i = 0; i < 1000; i++) {
        for (int j = 0; j < 1000; j++) {
            number += map[i][j];
        }
    }
    printf("Number of lights on: %d\n", number);

    for (int i = 0; i < 1000; i++) {
        free(map[i]);
    }
    free(map);
    fclose(fptr);
    return 0;
}
