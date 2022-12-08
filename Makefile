
FLAMEGRAPHS_DIR ?= $(HOME)/src/FlameGraph
BIN = ./target/release/advent2019

run:
	cargo run --release

fmt:
	cargo fmt

check-fmt:
	cargo fmt --check

$(BIN): src/*.rs
	cargo build --release

perf.out: $(BIN)
	sudo dtrace -c "$(BIN)" -o $@ -n 'profile-997 /execname == "'$$(basename $(BIN))'"/ { @[ustack(100)] = count(); }'

perf.svg: perf.out
	$(FLAMEGRAPHS_DIR)/stackcollapse.pl < perf.out | $(FLAMEGRAPHS_DIR)/flamegraph.pl > perf.svg


clean:
	rm -f perf.svg perf.out
