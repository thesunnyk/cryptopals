RUSTC=rustc
RUSTDOC=rustdoc
FLAGS=
SOURCES=base64.rs
OUTPUT=hellorust

all: doc test $(OUTPUT)

.PHONY: all doc clean

$(OUTPUT): $(SOURCES)
	$(RUSTC) $(FLAGS) $<

doc: $(SOURCES)
	$(RUSTDOC) $<

test: $(SOURCES)
	$(RUSTC) --test $<

clean:
	rm $(OUTPUT)
	rm -rf doc
