BINS = ./turbo-c ./lightning-cpp ./threaded-cpp
BENCHES = \
  target/release/glacial target/release/slow \
  target/release/fast target/release/speedy \
  target/release/turbo ./turbo-c \
  target/release/lightning ./lightning-cpp \
  target/release/serious ./threaded-cpp

CC = clang
CPP = clang++
CFLAGS = -Wall -O3
CXXFLAGS = -Wall -lpthread

all: $(BINS)

./turbo-c: turbo.c
	$(CC) $(CFLAGS) -o turbo-c turbo.c

./lightning-cpp: lightning.cpp
	$(CPP) $(CXXFLAGS) -o lightning-cpp lightning.cpp

./threaded-cpp: threaded.cpp
	$(CPP) $(CXXFLAGS) -o threaded-cpp threaded.cpp

bench: $(BENCHES)
	hyperfine --warmup 2 $(BENCHES) --export-markdown BENCH.md

check: $(BENCHES)
	@for B in $(BENCHES); do echo -n "$$B "; $$B | md5sum ; done

clean:
	-rm -f $(BINS)
