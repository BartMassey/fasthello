BINS = turbo-c turbo-cpp

CC = clang
CPP = clang++
CFLAGS = -Wall -O3

all: $(BINS)

turbo-c: turbo.c
	$(CC) $(CFLAGS) -o turbo-c turbo.c

turbo-cpp: turbo.cpp
	$(CPP) $(CFLAGS) -o turbo-cpp turbo.cpp

clean:
	-rm -f $(BINS)
