CC ?= gcc
CFLAGS ?= -O2 -fPIC -Wall
LDFLAGS ?= -shared -ldl -pthread

all: selkies_joystick_interposer.so

selkies_joystick_interposer.so: joystick_interposer.c
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS)

clean:
	rm -f selkies_joystick_interposer.so

.PHONY: all clean
