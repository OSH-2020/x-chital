
#include <stdio.h>
#include <unistd.h>
int main() {
    FILE * f = fopen("/test.c", "r");
    printf("Hello World %d\n", f == NULL);
    return 0;
}