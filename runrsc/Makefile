
all:
	make -C ../rvisor-kernel
	gcc -pthread runrsc.c -o runrsc

create: all
	sudo ./runrsc create /home/share/x-chital/runrsc/test

test: create
	cd ./test
	make
	cd ..
	sudo ./runrsc exec /hello