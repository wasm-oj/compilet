#include <stdio.h>

int main() {
  int x = 1;
  for (int i = 1; i <= 10000000; ++i) {
    x *= i;
    x %= 1000000007;
  }
  printf("Hello World! %d\n", x);
  return 0;
}
