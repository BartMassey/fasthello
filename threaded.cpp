#include <atomic>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <thread>
#include <unistd.h>
#include <vector>

int increase_str_num(char *input, int input_size) {
  int i = input_size - 1;
  for (; i >= 0; --i) {
    ++input[i];
    int n = input[i] - '0';
    if (n == 10)
      input[i] = '0';
    else
      break;
  }
  return input_size - i;
}

int_fast32_t set_num(int n, char num[9]) {
  int i{8};
  while (n) {
    num[i--] = n % 10 + '0';
    n /= 10;
  }
  return 8 - i;
}

int main() {
  constexpr int_fast32_t target{100'000'000};
  // Experiment with different thread counts.
  const auto N_THREADS{std::thread::hardware_concurrency()};
  constexpr int_fast32_t BUFFER_SIZE{BUFSIZ * 4};
  constexpr int_fast32_t PER_BUFFER{BUFFER_SIZE / 17};
  constexpr char prefix[]{"Hello, "};

  std::atomic<int> turn{0};
  const auto task = [&](int id) {
    char buffer[BUFFER_SIZE];
    char num[9]{'0', '0', '0', '0', '0', '0', '0', '0', '0'};
    int_fast32_t buf_index{0};
    int_fast32_t i{id * PER_BUFFER};
    while (i < target) {
      int_fast32_t num_len{i ? set_num(i, num) : 1};
      int_fast32_t todo{std::min(PER_BUFFER, target - i)};
      while (todo--) {
        memcpy(buffer + buf_index, prefix, sizeof(prefix) - 1);
        buf_index += sizeof(prefix) - 1;
        memcpy(buffer + buf_index, num + sizeof(num) - num_len, num_len);
        buf_index += num_len;
        ++i;
        buffer[buf_index] = '\t';
        ++buf_index;
        num_len += (increase_str_num(num, sizeof(num)) > num_len);
      }
      // if (i == target) buf_index--;
      while (turn.load(std::memory_order_relaxed) % N_THREADS != id)
            std::this_thread::yield();
      write(STDOUT_FILENO, buffer, buf_index);
      turn.fetch_add(1, std::memory_order_relaxed);
      buf_index = 0;
      i += (N_THREADS - 1) * PER_BUFFER;
    }
  };

  std::vector<std::thread> threads;
  threads.reserve(N_THREADS);
  for (int id{0}; id < N_THREADS; id++)
    threads.emplace_back(task, id);

  for (auto &t : threads)
    t.join();
  return 0;
}