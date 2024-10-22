#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <openssl/md5.h>
#include <string.h>

int main (int argc, char *argv[]) {
    unsigned char key[8] = "yzbqklnj";
    int number = 0;
    bool first = false;

    // I didn't know that you can access the result as an array
    unsigned char *result = malloc(16);
    unsigned char *query = malloc(100);
    while (true) {
        sprintf(query, "%s%d", key, number);
        MD5(query, strlen(query), result);
        // !result[?] checks if the first 2 bytes are 0 and the 3rd byte is less than 0x10(16) (this is extremly smart because it checks if it is 0x0F or smaller)
        if (!result[0] && !result[1] && result[2] < 0x10) {
            if (!result[0] && !result[1] && !result[2]) {
                printf("AdventCoin 2: %d\n", number);
                break;
            } else if (!first) {
                first = true;
                printf("AdventCoin 1: %d\n", number);
            }
        }
        number++;
    }
    return 0;
}
