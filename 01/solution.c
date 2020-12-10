#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char **argv)
{
    FILE *fptr;
    char file[] = "input";
    char line[100];
    int values[2000];
    int target = 2020;
    int idx = 0;

    if ((fptr = fopen("input", "r")) == NULL)
    {
        printf("Unable to open file: %s\n", file);
        return 1;
    }

    while (fgets(line, sizeof line, fptr))
    {
        values[idx] = atoi(line);
        ++idx;
    }

    --idx;
    int i;
    int iLimit = idx - 1;
    int j;
    int jLimit = idx;
    int val1;
    int val2;

    for (i = 0; i < iLimit; ++i)
    {
        val1 = values[i];
        for (j = i + 1; j < jLimit; ++j)
        {
            val2 = values[j];
            if ((val1 + val2) == target)
            {
                printf("Matched values: (%d)%d (%d)%d with product %d\n", i, val1, j, val2, val1 * val2);
                return 0;
            }
        }
    }

    return 2;
}
