// Do a doubly-linked list where each node contains a pointer to a string stored in the heap
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const int MAX_DATA_LEN = 100;

typedef struct Node
{
    char *data;
    struct Node *prev;
    struct Node *next;
} Node;

typedef struct DoublyLinkedList
{
    Node *head;
    Node *tail;
    int len;
} DoublyLinkedList;

Node *create_node(char *data, Node *prev, Node *next);
DoublyLinkedList *create_doubly_linked_list(char *data);
int find(DoublyLinkedList *dll, char *target);
void append(DoublyLinkedList *dll, char *data);
void insert(DoublyLinkedList *dll, int indx, char *data);
void delete(DoublyLinkedList *dll, int indx);
void free_dll(DoublyLinkedList *dll);
void print_node(Node *n);
void traverse_dll(DoublyLinkedList *dll);

int main()
{
    DoublyLinkedList *dll = create_doubly_linked_list("Head");
    append(dll, "A");
    traverse_dll(dll);

    printf("%d\n", find(dll, "Ba"));

    free_dll(dll);
    return 0;
}

Node *create_node(char *_data, Node *_prev, Node *_next)
{
    Node *n = (Node *)calloc(1, sizeof(Node));

    n->data = (char *)calloc(MAX_DATA_LEN, sizeof(char));
    n->prev = _prev;
    n->next = _next;
    strcpy(n->data, _data);

    return n;
}

DoublyLinkedList *create_doubly_linked_list(char *data)
{
    DoublyLinkedList *dll = (DoublyLinkedList *)calloc(1, sizeof(DoublyLinkedList));
    dll->head = create_node(data, NULL, NULL);
    dll->tail = dll->head;
    dll->len = 1;

    return dll;
}

int find(DoublyLinkedList *dll, char *target)
// Returns the index (zero-based) of the first matched node with same
// data as target. If there are no matched node, then this returns -1.
{
    Node *t = dll->head;
    int i = 0;
    while (t != NULL)
    {
        if (!strcmp(t->data, target)) return i;
        t = t->next;
        i++;
    }

    return -1;
}

void append(DoublyLinkedList *dll, char *data)
{
    if (dll->head == NULL)
    {
        Node *new = create_node(data, NULL, NULL);
        dll->head = new;
        dll->tail = new;
    }
    else if (dll->head == dll->tail)
    {
        Node *new = create_node(data, dll->head, NULL);
        dll->head->next = new;
        dll->tail = new;
    }
    else
    {
        Node *new = create_node(data, dll->tail, NULL);
        dll->tail->next = new;
        dll->tail = new;
    }

    dll->len++;
}

void insert(DoublyLinkedList *dll, int indx, char *data)
// Inserts new node at indx position. All nodes with previous index within the
// range [indx, dll->len-1] have their index incremented by 1.
// indx is zero-based.
{
    if (indx > dll->len)
    {
        printf("Cannot insert node to index %d since it is greater than the length of the doubly-linked list.\n", indx);
        return;
    }
    
    if (indx == dll->len)
    {
        append(dll, data);
        return;
    }

    Node *t = dll->head;

    int i = 0;
    while (i != indx)
    {
        t = t->next;
        i++;
    }

    Node *new = create_node(data, t->prev, t);
    if (t->prev != NULL)
        t->prev->next = new;
    t->prev = new;

    if (indx == 0)
        dll->head = new;
    if (indx == dll->len)
        dll->tail = new;
    dll->len++;
}

void delete(DoublyLinkedList *dll, int indx)
// Similar indexing rules with insert.
{
    if (indx >= dll->len)
    {
        printf("No node at index %d.\n", indx);
        return;
    }

    Node *t = dll->head;
    int i = 0;
    while (i != indx)
    {
        t = t->next;
        i++;
    }

    if (dll->len == 1)
    {
        dll->head = NULL;
        dll->tail = NULL;
        dll->len--;
        free(t);
        return;
    }

    if (indx == 0)
    {
        t->next->prev = NULL;
        dll->head = t->next;
    }
    else if (indx == dll->len - 1)
    {
        t->prev->next = NULL;
        dll->tail = t->prev;
    }
    else
    {
        t->prev->next = t->next;
        t->next->prev = t->prev;
    }
    dll->len--;
    free(t);
}

void free_dll(DoublyLinkedList *dll)
{
    Node *t = dll->tail;
    while (t != dll->head)
    {
        t = t->prev;
        free(t->next);
    }
    free(t);
    free(dll);
}

void print_node(Node *n)
{
    printf("Node %p: %s, Previous: %p, Next: %p\n", n, n->data, n->prev, n->next);
}

void traverse_dll(DoublyLinkedList *dll)
{
    Node *t = dll->head;
    do
    {
        printf("%s->", t->data);
        t = t->next;
    } while (t != NULL);
    printf("NULL\n");
}