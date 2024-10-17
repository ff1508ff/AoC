#include <stdio.h>

int main (int argc, char *argv[]) {
    FILE* fptr = fopen("../input.txt", "r");
    if (fptr == NULL) {
        printf("File not found\n");
        return 1;
    }

    return 0;
}
