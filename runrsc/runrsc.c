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

#define PORT 12290
#define RVISOR_CREATE 0
#define RVISOR_ADD_PROC 1
#define RVISOR_REMOVE_PROC 2


/* TODO: 

Usage could be provided in the future.

Socket can only handle limited numbers of instructions.(100 instructions)

`ps`: Info is provided by boot, instead of `ps`.  Based on an assumption that the whole runrsc is run in ONE shell. Could be improved if we want to run it in different shell  

*/



int boot_pid;

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
    rmdir("/sys/fs/cgroup/memory/rvisor");
}

int create(char *path){
    system("dmesg --clear");
    system("rmmod rvisor");    // RESPOSE: your syntax error!
    //  problem with 'rmmod'.
    int suc = system("insmod /home/share/orig-rvisor-kernel/rvisor.ko");
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
        pthread_mutex_lock(&mutex);
        temp->pid = pid;
        temp->next = head->next;
        head->next = temp;
        pthread_mutex_unlock(&mutex);
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
    FILE *fp;


    //cgroup   limit in memory
    mkdir("/sys/fs/cgroup/memory/rvisor", 0777);
    if ((fp = fopen("/sys/fs/cgroup/memory/rvisor/memory.limit_in_bytes","w")) == NULL) {
        perror("openfile"); exit(0); }
    fprintf(fp,"67108864");
    fclose(fp);
    if ((fp = fopen("/sys/fs/cgroup/memory/rvisor/memory.kmem.limit_in_bytes","w")) == NULL) {
        perror("openfile"); exit(0); }
    fprintf(fp,"67108864");
    fclose(fp);
    if ((fp = fopen("/sys/fs/cgroup/memory/rvisor/memory.swappiness","w")) == NULL) {
        perror("openfile"); exit(0); }
    fprintf(fp,"0");
    fclose(fp);
    if ((fp = fopen("/sys/fs/cgroup/memory/rvisor/cgroup.procs","w")) == NULL) {
        perror("openfile"); exit(0); }
    fprintf(fp,"%d",boot_pid);
    fclose(fp);

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

int execute(char **argv){
    extern char **environ;
    pid_t pid;
    FILE *fp;
    int background = 0;
    pid = fork();
    if(pid == 0){
        char buffer[15];
        char **envir = environ;
        char *env_temp;
        char env_name[128];
        char *path = argv[0];
        char *env_to_set[64];
        int i, j = 0;

        if ((fp = fopen("/sys/fs/cgroup/memory/rvisor/cgroup.procs","a")) == NULL) {
        perror("openfile"); exit(0); }
        fprintf(fp,"%d",getpid());
        fclose(fp);


        for(i = 1; argv[i]; i++){
            if (strcmp(argv[i], "-env") == 0) {
                if (argv[++i]) env_to_set[j++] = argv[i];
                else {printf("missing envir after '-env'\n"); exit(0); }
            }
        }
        
        if (strcmp(argv[i-1], "&") == 0) background = 1;

        /*clear envs*/
        while (*envir) {
            env_temp = *envir;
            for(i = 0; *(env_temp + i) != '='; i++){
                env_name[i] = *(env_temp + i);
            }
            env_name[i] = '\0';
            unsetenv(env_name);
            envir++;
        }

        for(j = 0; env_to_set[j]; j++){
            i = putenv(env_to_set[j]);
            if(i < 0) {printf("putenv"); exit(0);}
        }

        i = chdir("/");
        if(i < 0) {perror("chdir"); exit(0);}
        i = putenv("PWD=/");
        if(i < 0) {perror("putenv"); exit(0);}
        
//      system("env");

//      system("pwd");

        int fd = open("/dev/rvisor", O_RDWR);
        if (fd == -1) perror("Execute, rvisor");
        ioctl(fd, RVISOR_ADD_PROC, getpid());
        close(fd);

        printf("son process = %d\n",getpid());
        sprintf(buffer,"%09d \n",getpid());
        send_to_boot(buffer);
        char *args[2]; 
        args[0] = path;
        args[1] = NULL;

        sleep(0.5);
        execvp(args[0], args);
        perror("execvp");
        exit(0);
    }
    else {
        if (!background){
            //printf("father process = %d\n",getpid());
            int i;
            wait(&i);
        }
        exit(0);
    }
}

void print_usage(){
    printf("Runrsc Usage:\n");
    printf("    ./runrsc [command]\n\n");
    printf("To create a container: \n\t./runrsc create [path]\n");
    printf("To print process status info: \n\t./runrsc ps\n");
    printf("To shutdown all processes and the container itself: \n\t./runrsc shutdown\n");
    printf("To start a process in the container: \n\t./runrsc exec [path][option]\n");
    printf("\toption: -env [name=value]\n");
}


int main(int argc, char **argv) {
    pid_t pid;

    //TODO if argc==1 usage
    if (argc < 2){
        print_usage();
        return 0;
    }
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
    else if (strcmp(argv[1], "exec") == 0) {
        //send_to_boot("0\n");
        if (!argv[2]) {
            perror("Exec Error: expect guest path.");
        }
        else {
            execute(argv + 2);
        }
    }
    else if (strcmp(argv[1], "ps") == 0){
        send_to_boot("ps\n");
        sleep(1); //so output won't be disturbed by shell
    } 
    else if (strcmp(argv[1], "shutdown") == 0){
        send_to_boot("shutdown\n");
        sleep(1);
    }
    else print_usage();
}