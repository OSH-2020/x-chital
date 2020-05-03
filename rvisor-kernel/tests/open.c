
#include <unistd.h>
#include <sys/syscall.h>   /* For SYS_xxx definitions */

int main() {
    syscall(__NR_open, "asdf", 0);
    return 0;
}