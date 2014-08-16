RUSTC=rustc
RUSTDOC=rustdoc
FLAGS=
BINS=challenge1.out challenge2.out challenge3.out challenge4.out

all: doc out

out: $(BINS)

.PHONY: all doc clean

%.test: %.rs
	$(RUSTC) $(FLAGS) --test -o target/$@ $<

%.out: %.rs
	$(RUSTC) $(FLAGS) -o target/$@ $<

clean:
	rm $(OUTPUT)
	rm -rf doc
