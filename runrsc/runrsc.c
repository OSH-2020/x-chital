#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/wait.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/prctl.h>
#include <fcntl.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <pthread.h>
#include <arpa/inet.h>
#include <sys/ioctl.h>

#define PORT 1253
#define RVISOR_CREATE 0
#define RVISOR_ADD_PROC 1
#define RVISOR_REMOVE_PROC 2

/* To test:

You can use "/home/hardyho/test". It prints "test..." every 10 secs.

*/


/* TODO: 

pthread_mutex_lock should be setted when operations about the linked list are done. Not set yet.

Usage could be provided in the future.

Socket cannot handle the situation where the pid exit. Maybe could be handled if we send message from the boot to clients every certain time. 

`ps`: Info is provided by boot, instead of `ps`.  Based on an assumption that the whole runrsc is run in ONE shell. Could be improved if we want to run it in different shell  

*/


/*PROBLEMS: 

The PORT is settled. Sometimes(unpredictable) need to switch the PORT and compile it again before you run it.

*/


/*NEED HELP:

Is `rmmod` required or not? Seems that there's a problem related to kernel-module when running rmmod in this program.

In result of the absence of `rmmod`, when running `create`:
```
insmod: ERROR: could not insert module ../rvisor-kernel/rvisor.ko: File exists
mknod: /dev/rvisor: File exists
```
Are these acceptable?

Check more about the function related to kernel-module. Don't know that part too much so that the debugging isn't mainly focus on that part.

*/




int boot_pid;  // for `shutdown` to kill the boot.(required or not?)

struct pid_node{
    int pid;
    struct pid_node *next;
} *head;
pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;

int pid_alive(int pid){
    //Just a trick, may be better way 
    //if pid is alive, return 1
    //else return 0
    char cmd[64];
    char result[32];
    FILE * fp;
    sprintf(cmd, "ps -p %d | wc", pid); //if already killed, only title will be outputed be `ps`, otherwise there will be 2 lines.
    if ((fp = popen(cmd, "r") ) == NULL){
        printf("Popen Error!\n");
		return -1;
	}
    memset(result, 0, 32);
    fgets(result, 32, fp);
    pclose(fp);
    return (atoi(result)-1);
}

int pid_info(char *result,int pid){
    // Get the second line of the result of `ps`(the first line is the titie)
    char cmd[64];
    FILE * fp;
    sprintf(cmd, "ps -p %d", pid);
    if ((fp = popen(cmd, "r") ) == NULL){
        printf("Popen Error!\n");
		return -1;
	}
    memset(result, 0, 64);
    while(fgets(result, 64, fp) != NULL);
    pclose(fp);
    result[strlen(result)-1] = '\0';
    return 1;
}


int ps(){
    struct pid_node *temp;
    int pid;
    char result[64];
    printf("  PID TTY          TIME CMD\n");
    for(temp = head; temp->next; temp = temp->next){
        pid = temp->next->pid;
        if (pid_alive(pid)) {
            pid_info(result, pid);
            printf("%s\n",result);
        }
    }
}

int killall(){
    struct pid_node *temp, *temp1;
    int pid;
    char cmd[20];
    int fd = open("/dev/rvisor", O_RDWR);
    if (fd == -1) perror("Kill, rvisor");
    for(temp = head; temp->next; temp = temp->next){
        pid = temp->next->pid;
        if (pid_alive(pid)) {  
            ioctl(fd, RVISOR_REMOVE_PROC, pid); 
            sprintf(cmd, "kill -9 %d", pid);
            //printf("cmd:%s\n",cmd);
            system(cmd);
            printf("%d has been killed.\n",pid);
        }
    }
    temp = head->next;
    head->next = NULL;
    for (; temp; temp = temp1){
        temp1 = temp->next;
        free(temp);
    }
    sprintf(cmd, "kill -9 %d", boot_pid); //Kill boot?
    system(cmd);
}

int create(char *path){
    system("dmesg --clear");
    system("rmmod rvisor");    // RESPOSE: your syntax error!
    //  problem with 'rmmod'.
    int suc = system("insmod ../rvisor-kernel/rvisor.ko");
    if(suc != 0) { printf("insmod failed!\n"); exit(1);}

    system("mknod --mode=a=rw /dev/rvisor c $(cat /proc/devices | grep rvisor | awk '{print $1}') 0");
    int fd = open("/dev/rvisor", O_RDWR);
    if (fd == -1) perror("Create, rvisor");
    int r = ioctl(fd, RVISOR_CREATE, path);
    if (r < 0) {
        perror("Create, rvisor");
        return 0;
    }
    close(fd);
}

void *handler(void *data){

    /*To deal with the message that's sended by client. "ps","shutdown","(PID)"*/

    int *client_id = (int *)data;
    int i;
    char buffer[15];
    //printf("%d handler!\n",*client_id);
    read(*client_id, buffer, 15);
    //printf("read success.\n");
    if (buffer[0] == 'p'){
        printf("ps!\n");
        ps();
    }
    else if (buffer[0] == 's'){
        printf("shutdown!\n");
        killall();
    }
    else {
        int pid;
        pid = atoi(buffer);
        printf("pid = %d\n",pid);
        struct pid_node *temp;
        temp = malloc(sizeof(struct pid_node));
        //pthread_mutex_lock(&mutex);
        temp->pid = pid;
        temp->next = head->next;
        head->next = temp;
        //pthread_mutex_unlock(&mutex);
    }
    close(*client_id);
    return NULL;
}

int boot(){

    /*Work as a server*/

    pthread_t thread[100];
    int i;
    int client_num = 0;
    int fd_clients[100];
    struct Node *temp;
    int fd, fd_temp;
    printf("boot!\n");
    boot_pid = getpid();
    head = malloc(sizeof(struct pid_node));
    head->next = NULL;
    if ((fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
        perror("socket");
        return 1;
    }
    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = inet_addr("127.0.0.1");
    addr.sin_port = htons(PORT);
    socklen_t addr_len = sizeof(addr);
    if (bind(fd, (struct sockaddr *)&addr, sizeof(addr))) {
        perror("bind");
        return 1;
    }
    if (listen(fd, 100)) {
        perror("listen");
        return 1;
    }
    printf("boot ready to accept\n");

    i = 0;
    while(1){
        while (client_num < 100){
            if ((fd_temp = accept(fd, NULL, NULL)) != -1) {
                printf("%d connected.\n",fd_temp);
                fd_clients[i] = fd_temp;
                pthread_create(&thread[i], NULL, handler, (void *)&fd_clients[i]); //Create a thread to handle it
                pthread_detach(thread[i]);
                client_num++;
                i++;
            }
            else{
                perror("accept");
                return 1;
            }
        }
    }
    return 0;
}


int send_to_boot(char *message)
{
    //printf("send_to_boot!\n");
    int fd = socket(AF_INET, SOCK_STREAM, 0);    
    if (fd == -1) {
        perror("socket");
        return 1;
    } 
    struct sockaddr_in addr;      
    addr.sin_family = AF_INET;
    addr.sin_port = htons(PORT);
    addr.sin_addr.s_addr = inet_addr("127.0.0.1");
    int res = connect(fd, (struct sockaddr*)&addr, sizeof(addr));   
    if (res == -1){
        perror("connect");
        return 1;
    }
    //printf("client:%d connect.\n",fd);
    write(fd, message, 15);
    //printf("write success.\n");
    close(fd);
    return 0;
}

int execute(char *path){
    pid_t pid;
    pid = fork();
    if(pid == 0){
        sleep(1);
        char buffer[15];
        printf("son process = %d\n",getpid());
        sprintf(buffer,"%09d \n",getpid());
        send_to_boot(buffer);
        char *args[2]; 
        args[0] = path;
        args[1] = NULL;

        /* should use execvp(), but there's a bug*/
        sleep(0.5);
        //system(path);
        execvp(args[0], args);
        perror("execvp");
        exit(0);
    }
    else{
        //printf("father process = %d\n",getpid());
        int fd = open("/dev/rvisor", O_RDWR);
        if (fd == -1) perror("Execute, rvisor");
        ioctl(fd, RVISOR_ADD_PROC, pid);

        int i;
        wait(&i);
        close(fd);
        exit(0);
    }
}


int main(int argc, char **argv) {
    pid_t pid;

    //TODO if argc==0 usage

    if (strcmp(argv[1], "create") == 0){
        if (!argv[2]) {
            perror("Create Error: expect root path.");
        }
        else create(argv[2]);
        pid = fork();
        if (pid == 0)
        boot();
        else exit(0);
    }
    if (strcmp(argv[1], "exec") == 0) {
        //send_to_boot("0\n");
        if (!argv[2]) {
            perror("Exec Error: expect guest path.");

            
        }
        else {
            execute(argv[2]);
        }
    }
    if (strcmp(argv[1], "boot") == 0){
        boot();
    }
    if (strcmp(argv[1], "ps") == 0){
        send_to_boot("ps\n");
        sleep(1); //so that output won't be disturbed by shell
    } 
    if (strcmp(argv[1], "shutdown") == 0){
        send_to_boot("shutdown\n");
        sleep(1);
    }
}