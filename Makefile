RUSTC=rustc
RUSTDOC=rustdoc
FLAGS=
SOURCES=hello.rs
OUTPUT=hellorust

all: doc $(OUTPUT)

.PHONY: all doc clean

$(OUTPUT): $(SOURCES)
	$(RUSTC) $(FLAGS) $<

doc: $(SOURCES)
	$(RUSTDOC) $<

clean:
	rm $(OUTPUT)
	rm -rf doc
