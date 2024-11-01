# Makefile

# 选择编译器
CC := gcc
CFLAGS := -Wall -Wextra

# 检测平台
ifeq ($(OS), Windows_NT)
    CC := clang
    TARGET := shell.exe
else
    TARGET := shell
endif

# 源文件
SRCS := main.c
OBJS := $(SRCS:.c=.o)

# 默认目标
all: $(TARGET)

# 编译目标
$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^

# 编译源文件为目标文件
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# 清理生成的文件
clean:
	@if exist $(TARGET) del $(TARGET)
	@if exist *.o del *.o

# 伪目标
.PHONY: all clean
