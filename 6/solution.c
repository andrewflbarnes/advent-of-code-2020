#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void reset(int arr[]) {
    for (int i = 0; i < 26; ++i) {
        arr[i] = 0;
    }
}

int count_set(int arr[]) {
    int set = 0;
    for (int i = 0; i < 26; ++i) {
        if (arr[i] == 1) {
            ++set;
        }
    }

    // printf("Counted: %d\n", set);
    return set;
}

void print_arr(int arr[]) {
    for (int i = 0; i < 26; ++i)
    {
        printf("%d", arr[i]);

    }
    printf("\n");
}

int main(int argc, char **argv)
{
    FILE *fptr;
    char file[] = "input";
    char line[28];
    int answers_all[26] = {0};
    int answers_all_count = 0;
    int answers_any[26] = {0};
    int answers_any_count = 0;
    int these_answers[26];
    bool new_group = true;

    if ((fptr = fopen("input", "r")) == NULL)
    {
        printf("Unable to open file: %s\n", file);
        return 1;
    }

    while (fgets(line, sizeof line, fptr))
    {
        // printf("Line: %lu - %s\n", strlen(line), line);
        if (line[strlen(line) - 1] == '\n') {
            line[strlen(line) - 1] = '\0';
        }

        if (strlen(line) == 0) {
            answers_any_count += count_set(answers_any);
            answers_all_count += count_set(answers_all);
            // printf("Any: %d\nAll: %d\n", answers_any_count, answers_all_count);
            reset(answers_any);
            reset(answers_all);
            new_group = true;
        }
        else {
            int len = strlen(line);
            int idx;
            reset(these_answers);
            for (int i = 0; i < len; ++i)
            {
                idx = line[i] - 0x61;
                these_answers[idx] = 1;
            }

            // print_arr(answers_any);
            // print_arr(answers_all);
            for (int i = 0; i < 26; ++i)
            {
                answers_any[i] |= these_answers[i];

                if (new_group)
                {
                    answers_all[i] = these_answers[i];
                } else {
                    answers_all[i] &= these_answers[i];
                }

            }
            // print_arr(these_answers);
            // print_arr(answers_any);
            // print_arr(answers_all);

            if (new_group)
            {
                new_group = false;
            }
        }
    }

    answers_any_count += count_set(answers_any);
    answers_all_count += count_set(answers_all);

    printf("Any: %d\nAll: %d\n", answers_any_count, answers_all_count);

    return 0;
}