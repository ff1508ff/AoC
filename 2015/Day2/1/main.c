#include <stdio.h>

int main (int argc, char *argv[]) {
    FILE* fptr = fopen("../input.txt", "r");
    if (fptr == NULL) {
        printf("File not found\n");
        return 1;
    }

    int total = 0;

    int l = 0;
    int w = 0;
    int h = 0;

    int number= 0;

    for (char c = fgetc(fptr); c != EOF; c = fgetc(fptr)) {
        // also possable to use else instead of continue, but I like the astetic of continue and this is my code so live with it (I dont think the performance is better on one of them)
        if (c == '\n') {
            h = number;
            int lw = l * w;
            int wh = w * h;
            int hl = h * l;

            total += 2 * lw + 2 * wh + 2 * hl + (lw < wh ? (lw < hl ? lw : hl) : (wh < hl ? wh : hl));

            // reset
            l = 0;
            w = 0;
            h = 0;
            number = 0;
            continue;
        }

        if (c == 'x') {
            // ther is no value 0 for l, w, h
            if (l == 0) {
                l = number;
            } else if (w == 0) {
                w = number;
            }
            number= 0;
            continue;
        }

        number = number * 10 + (c - '0');
    }
    printf("Total: %d\n", total);

    return 0;
}
