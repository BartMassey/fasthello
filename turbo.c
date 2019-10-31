#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
 
#define MAX(a, b) ((a) > (b) ? (a) : (b))
 
int increase_str_num(char *input, int input_size) {
    int i = input_size - 1;
    for(; i >= 0; --i) {
        ++input[i];
        int n = input[i] - '0';
        if (n == 10) input[i] = '0';
        else break;
    }
    return input_size - i;
}
 
int main() {
    char buffer[BUFSIZ];
    char num[12] = "000000000000";
    int num_len = 1;
    const char prefix[] = "Hello, ";
    int buf_index = 0;
    const int line_max_len = 17;
    unsigned int i = 0;
    int n;
 
    int num_len_increase = 1;
    const unsigned int target = 100000000;
 
    while(i < target) {
        while(buf_index < sizeof(buffer) - line_max_len && i < target) {
            memcpy(buffer + buf_index, prefix, sizeof(prefix) - 1);
            buf_index += sizeof(prefix) - 1;
            num_len = MAX(num_len, num_len_increase);
            memcpy(buffer + buf_index, num + sizeof(num) - num_len, num_len);
            buf_index += num_len;
            ++i;
            buffer[buf_index] = '\t';
            ++buf_index;
            num_len_increase = increase_str_num(num, sizeof(num));
        }
        n = write(STDOUT_FILENO, buffer, buf_index);
        if (n == -1) {
            perror("write");
            return -1;
        }
        buf_index = 0;
    }
    return 0;
}
