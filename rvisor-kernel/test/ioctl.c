#include <sys/ioctl.h>
#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

int i;
int main() {
    int fd = open("/dev/rvisor", O_RDWR);
    if(fd < 0) {
        perror("open");
    }
    int i = ioctl(fd, 1, &i);
    if(i == -1) {
        perror("ioctl");
    }
    return 0;
}
