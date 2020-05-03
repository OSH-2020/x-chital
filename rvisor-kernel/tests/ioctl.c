#include <sys/ioctl.h>
#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/types.h>
#include <sys/wait.h>

int i;
int main() {
    int fd = open("/dev/rvisor", O_RDWR);
    if(fd < 0) goto err;
    
    int r = ioctl(fd, 0, "/Users/dnailz");
    
    if(fd < 0) goto err;
    
    int pid = fork();

    if(pid == 0){
        sleep(1);
        int r = open("/pslog.txt", O_RDONLY);
        printf("%d", r);
        exit(0);
    }
    ioctl(fd, 1, pid);

    int i;
    wait(&i);
    return 0;
err:
    perror("main");
    return 0;
}
