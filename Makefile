C++ = g++
SRC = main.cpp
TARGET = main

INCLUDE_DIR = -I./

RUST_LIB = ./target/release/libcore.a

all: $(TARGET)

rust-build:
	cargo build --release

$(TARGET): clean $(RUST_LIB) $(SRC) rust-build
	$(C++) -o $(TARGET) $(SRC) $(RUST_LIB) $(INCLUDE_DIR) -lpthread -lstdc++

clean:
	@rm -f $(TARGET)

run: $(TARGET)
	@valgrind --leak-check=full ./main
