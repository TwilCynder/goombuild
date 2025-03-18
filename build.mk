PS4_PATH=test/projetS4/

all: base

base:
	cargo run -- -o test.mk

ps4: 
	cargo run -- -o $(PS4_PATH)/Makefile
