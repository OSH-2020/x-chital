
all: getdesh
	make -C ../rvisor-kernel
	gcc -static -pthread runrsc.c -o runrsc

getdesh:
	make -C /home/dnailz/OSH-2020-Labs/lab2/
	cp /home/dnailz/OSH-2020-Labs/lab2/desh alpine/root
create: all
	sudo ./runrsc create /home/share/x-chital/runrsc/test

test: create
	cd ./test
	make
	cd ..
	sudo ./runrsc exec /hello
fetch_alpine:
	wget http://dl-cdn.alpinelinux.org/alpine/v3.11/releases/x86_64/alpine-minirootfs-3.11.6-x86_64.tar.gz
