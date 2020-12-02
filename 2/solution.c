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
    size_t min;
    size_t max;
    int valid = 0;

    // reads text until newline is encountered
    while (fgets(c, sizeof c, fptr))
    {
        c[strlen(c) - 1] = 0;

        // parse data
        char *ptr = strtok(c, "-");
        sscanf(ptr, "%zu", &min);

        ptr = strtok(NULL, " ");
        sscanf(ptr, "%zu", &max);

        ptr = strtok(NULL, ":");
        letter = ptr;

        ptr = strtok(NULL, " ");
        password = ptr;

        int matched = 0;
        size_t idx = 0;
        size_t limit = strlen(password);

        for (idx = 0; idx < limit; ++idx)
        {
            if (password[idx] == *letter)
            {
                matched += 1;
            }
        }

        if (matched >= min && matched <= max)
        {
            valid += 1;
        }
    }

    printf("Valid passwords: %d\n", valid);

    return 0;
}
