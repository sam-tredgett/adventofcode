#include <stddef.h>
#include <stdint.h>
#include<stdio.h>

typedef struct dict_entry_s {
    const char *key;
    int value;
} dict_entry_s;

typedef struct dict_s {
    int len;
    int cap;
    dict_entry_s *entry;
} dict_s, *dict_t;

int dict_find_index(dict_t dict, const char *key) {
    for (int i = 0; i < dict->len; i++) {
        if (!strcmp(dict->entry[i], key)) {
            return i;
        }
    }
    return -1;
}

int dict_find(dict_t dict, const char *key, int def) {
    int idx = dict_find_index(dict, key);
    return idx == -1 ? def : dict->entry[idx].value;
}

void dict_add(dict_t dict, const char *key, int value) {
   int idx = dict_find_index(dict, key);
   if (idx != -1) {
       dict->entry[idx].value = value;
       return;
   }
   if (dict->len == dict->cap) {
       dict->cap *= 2;
       dict->entry = realloc(dict->entry, dict->cap * sizeof(dict_entry_s));
   }
   dict->entry[dict->len].key = strdup(key);
   dict->entry[dict->len].value = value;
   dict->len++;
}

dict_t dict_new(void) {
    dict_s proto = {0, 10, malloc(10 * sizeof(dict_entry_s))};
    dict_t d = malloc(sizeof(dict_s));
    *d = proto;
    return d;
}

void dict_free(dict_t dict) {
    for (int i = 0; i < dict->len; i++) {
        free(dict->entry[i].key);
    }
    free(dict->entry);
    free(dict);
}

  // north (^), south (v), east (>), or west (<). After each move, he delivers 
  // ^ = y + 1, v = y - 1, > = x + 1, < x - 1
  // current implementation causes integer underflow
  // this is because we can go -1 on a 0 value int 
void calculate_coordinate_translation(char c, int *x, int *y) {
    switch (c){
        case '^':
            printf("found up\n");
            *y = *y + 1;
            break;
        case 'v':
            printf("found down\n");
            *y = *y - 1;
            break;
        case '>':
            printf("found right\n");
            *x = *x + 1;
            break;
        case '<':
            printf("found left\n");
            *x = *x - 1;
            break;
    }
    printf("Current coordinates are %i,%i\n", *x, *y);
}

int main() {
    // Objective is to read the input.txt file in the same directory as this
    // and interpret this with logic such that we can generate an answer based 
    // on the following prompt:
    //
    // --- Day 3: Perfectly Spherical Houses in a Vacuum ---
    // Santa is delivering presents to an infinite two-dimensional grid of houses.

    // He begins by delivering a present to the house at his starting location,
    // and then an elf at the North Pole calls him via radio and tells him where 
    // to move next. Moves are always exactly one house to the 
    // north (^), south (v), east (>), or west (<). After each move, he delivers 
    // another present to the house at his new location.

    // However, the elf back at the north pole has had a little too much eggnog, 
    // and so his directions are a little off, and Santa ends up visiting some 
    // houses more than once. How many houses receive at least one present?

    // For example:

    // > delivers presents to 2 houses: one at the starting location, and one to the east.
    // ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
    // ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.

    FILE *pfile = NULL;
    char *filename = "input.txt";
    pfile = fopen(filename, "r");
    if(pfile == NULL) {
        printf("failed to read %s\n", filename);
        return -1;
    }

    int c;
    // Represent the positions santa visits as coordinates in a dictionary
    // the dictionary begins as empty, for each move santa makes his position 
    // relative to the start is checked, e.g., we 
    // start with two integers, allowed to be negative, which are walked as x and y
    // from the origin. each move will increase/decrease x and y based on the move.
    // comparison wont be needed if the ints begin at 0, as they will track themselves by following Moves
    
    // each move, we need to capture the current location in a tbd data structure
    // if we find a coord we've already visited, we don't add anything new to the tbd datastructure
    // length of the tbd data structure will be our final answer
    

    int x = 0, y = 0;
    while((c = fgetc(pfile)) != EOF) {
        printf("%c\n", c);
        calculate_coordinate_translation(c, &x, &y);
        // we have our correct positions
        // we need to store this in a DISTINCT data structure with some size
    }
    fclose(pfile);
    pfile = NULL;


    return 0;
}



