#include <sys/ioctl.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/types.h>
#include <sys/wait.h>

#define RVISOR_CREATE 0
#define RVISOR_ADD_PROC 1
#define RVISOR_REMOVE_PROC 2

int i;
int main() {
    // cat /proc/devices | grep rvisor | awk '{print $1}' 可以获取主设备号
    system("mknod --mode=a=rw /dev/rvisor c $(cat /proc/devices | grep rvisor | awk '{print $1}') 0");

    int fd = open("/dev/rvisor", O_RDWR);
    if(fd < 0) goto err;
    
    int r = ioctl(fd, 0, "/home/dnailz");
    
    if(fd < 0) goto err;
    
    int pid = fork();

    if(pid == 0){
        sleep(1);
        int r = open("test.txt", O_RDONLY);
        printf("%d", r);
        system("dmesg");
        exit(0);
    }
    printf("Success? %d",ioctl(fd, 1, pid));

    int i;
    wait(&i);
    return 0;

err:
    perror("main");
    return 0;
}
