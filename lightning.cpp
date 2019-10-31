#include <algorithm>
#include <assert.h>
#include <stdio.h>
#include <string>
#include <unistd.h>

struct Buffer {
  char data[BUFSIZ];
  size_t index = 0;

  void extend(const char *str, size_t size) {
    std::copy(str, str + size, data + index);
    index += size;
  }

  void push(char c) { data[index++] = c; }

  void clear() { index = 0; }

  size_t size() const { return index; }
};

int increase_str_num(char *input, int input_size) {
  int i = input_size - 1;
  for (; i >= 0; --i) {
    ++input[i];
    if (input[i] - '0' < 10)
      return input_size - i;
    input[i] = '0';
  }
  assert(!"overflow input");
  return 0;
}

int main() {
  Buffer buffer;
  char num[12] = "00000000000";
  int num_len = 1;
  const char prefix[] = "Hello, ";
  const size_t LINE_MAX_LEN = 17;
  for (unsigned int i = 0; i < 100'000'000; ++i) {
    buffer.extend(prefix, sizeof(prefix) - 1);
    buffer.extend(num + sizeof(num) - 1 - num_len, num_len);
    num_len = std::max(num_len, increase_str_num(num, sizeof(num) - 1));
    buffer.push('\t');
    if (buffer.size() + LINE_MAX_LEN > BUFSIZ) {
      write(STDOUT_FILENO, buffer.data, buffer.size());
      buffer.clear();
    }
  }
  write(STDOUT_FILENO, buffer.data, buffer.size());
}
