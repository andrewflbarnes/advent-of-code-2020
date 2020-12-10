#include <stdio.h>
#include <string.h>

int main(int argc, char **argv) {
    FILE *fptr;
    if ((fptr = fopen("input", "r")) == NULL) {
        printf("Error! opening file");
        return 1;
    }

    char c[100];
    char *letter;
    char *password;
    size_t first;
    size_t second;
    int matched;
    int valid = 0;

    // reads text until newline is encountered
    while (fgets(c, sizeof c, fptr))
    {
        c[strlen(c) - 1] = 0;

        // parse data
        char *ptr = strtok(c, "-");
        sscanf(ptr, "%zu", &first);

        ptr = strtok(NULL, " ");
        sscanf(ptr, "%zu", &second);

        ptr = strtok(NULL, ":");
        letter = ptr;

        ptr = strtok(NULL, " ");
        password = ptr;

        matched = 0;
        if (password[first - 1] == *letter)
        {
            matched = 1;
        }
        if (password[second - 1] == *letter)
        {
            matched += 1;
        }

        if (matched == 1)
        {
            valid += 1;
        }
    }

    printf("Valid passwords: %d\n", valid);

    return 0;
}
