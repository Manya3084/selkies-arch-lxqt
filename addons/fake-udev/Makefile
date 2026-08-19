CC ?= gcc
CFLAGS ?= -O2 -fPIC -Wall -Wno-unused-parameter
LDFLAGS ?= -shared -Wl,--version-script=libudev.sym -ldl

TARGET = libudev.so.1.0.0-fake

all: $(TARGET) libudev.so.1 libudev.so

$(TARGET): fake-libudev-core.c libudev.h libudev.sym
	$(CC) $(CFLAGS) -o $@ fake-libudev-core.c $(LDFLAGS)

libudev.so.1: $(TARGET)
	ln -sf $(TARGET) $@

libudev.so: libudev.so.1
	ln -sf libudev.so.1 $@

clean:
	rm -f $(TARGET) libudev.so.1 libudev.so

.PHONY: all clean
