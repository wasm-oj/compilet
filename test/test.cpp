#include <iostream>

int main() {
  int x = 1;
  for (int i = 1; i <= 10000000; ++i) {
    x *= i;
    x %= 1000000007;
  }
  std::cout << "Hello, World! " << x << std::endl;
  return 0;
}
