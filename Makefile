BIN_DIR:=$(HOME)/dots/personal/.local/bin
BIN:=latext

test: FORCE;
	cargo build
	./target/debug/latext ./test/main.tex

build:
	@cargo build --release

load_bin:
	@rm -f $(BIN_DIR)/$(BIN)
	@cp ./target/release/$(BIN) $(BIN_DIR)

all:
	make build
	make load_bin

open:
	open -a Skim ./.build/main.pdf

FORCE: ;
