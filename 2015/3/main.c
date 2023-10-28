#include<stddef.h>
#include<stdint.h>
#include<stdio.h>
#include<stdlib.h>


struct LinkedList {
    int x, y;
    struct LinkedList *next;
};

void map_coords_to_linked_list(struct LinkedList **linked_list, int *found_x, int *found_y){
    if(*linked_list == NULL) {
        struct LinkedList *newNode = (struct LinkedList *)malloc(sizeof(struct LinkedList));
        if (newNode == NULL) {
            fprintf(stderr, "Memory allocation error\n");
            exit(1);
        }
        newNode->x = *found_x;
        newNode->y = *found_y;
        newNode->next = NULL;
        *linked_list = newNode;
    } else if ((*linked_list)->x == *found_x && (*linked_list)->y == *found_y) {
        return;
    } else {
        map_coords_to_linked_list(&((*linked_list)->next), found_x,found_y);
    }
}


int get_linked_list_length(struct LinkedList *linked_list, int *length){
    if(linked_list == NULL) {
        return *length;
    } else {
        *length = *length + 1;
        return get_linked_list_length(linked_list->next, length);
    }
}

  // north (^), south (v), east (>), or west (<). After each move, he delivers 
  // ^ = y + 1, v = y - 1, > = x + 1, < x - 1
void calculate_coordinate_translation(char c, int *x, int *y) {
    switch (c){
        case '^':
            *y = *y + 1;
            break;
        case 'v':
            *y = *y - 1;
            break;
        case '>':
            *x = *x + 1;
            break;
        case '<':
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
    int x = 0, y = 0;
    struct LinkedList *head = (struct LinkedList *)malloc(sizeof(struct LinkedList));
    head->x = x;
    head->y = y;
    head->next = NULL;
    
    while((c = fgetc(pfile)) != EOF) {
        printf("%c\n", c);
        calculate_coordinate_translation(c, &x, &y);
        map_coords_to_linked_list(&head, &x, &y);
    }
    fclose(pfile);
    pfile = NULL;

    int length = 0;
    get_linked_list_length(head, &length);
    printf("There were a total of %i distinct visited to houses\n", length);

    return 0;
}
