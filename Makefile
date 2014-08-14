RUSTC=rustc
RUSTDOC=rustdoc
FLAGS=
SOURCES=base64.rs xor.rs
TEST=xor.test base64.test

all: doc test

test: $(TEST)

.PHONY: all doc clean

%.test: %.rs
	$(RUSTC) $(FLAGS) --test -o target/$@ $<

clean:
	rm $(OUTPUT)
	rm -rf doc
