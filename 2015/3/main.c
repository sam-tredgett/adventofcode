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
    printf("current coordinates are %i,%i\n", *x, *y);
}

int main() {
    FILE *pfile = NULL;
    char *filename = "input.txt";
    pfile = fopen(filename, "r");
    if(pfile == NULL) {
        printf("failed to read %s\n", filename);
        exit(1);
    }
    int c, x = 0, y = 0;
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
    printf("there were a total of %i distinct visited to houses\n", length);
    return 0;
}
