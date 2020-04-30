KDIR ?= /lib/modules/$(shell uname -r)/build

KERN_OBJ = rvisor

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR)

clean:
	-sudo rmmod $(KERN_OBJ)
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean

ins: all
	-sudo dmesg --clear
	-sudo rmmod $(KERN_OBJ)
	sudo insmod $(KERN_OBJ).ko
doc:
	cargo doc --document-private-items