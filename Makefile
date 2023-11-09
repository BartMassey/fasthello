BINS = ./naïve-c ./turbo-c ./lightning-cpp
BENCHES = \
  target/release/naïve \
  target/release/dumbest \
  target/release/glacial target/release/slow \
  target/release/fast target/release/speedy \
  target/release/turbo ./turbo-c \
  target/release/lightning ./lightning-cpp \
  target/release/ludicrous \
  target/release/serious

CC = clang
CPP = clang++
CFLAGS = -Wall -O3

all: $(BINS)
	cargo build --release

./naïve-c: naïve.c
	$(CC) $(CFLAGS) -o naïve-c naïve.c

./turbo-c: turbo.c
	$(CC) $(CFLAGS) -o turbo-c turbo.c

./lightning-cpp: lightning.cpp
	$(CPP) $(CFLAGS) -o lightning-cpp lightning.cpp

bench: $(BENCHES)
	hyperfine --warmup 2 $(BENCHES) --export-markdown BENCH.md

check: $(BENCHES)
	@for B in $(BENCHES); do echo -n "$$B "; $$B | md5sum ; done

clean:
	-rm -f $(BINS)
	-cargo clean
