#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

int main(){
    // int i = open("/hello.c", O_RDONLY);
    char buf[200];
    getcwd(buf, 200);
    printf("%s", buf);
    exit(0);
}
