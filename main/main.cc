#include <iostream>
#include "hello.h"

int main() {
   std::string greeting = Hello("World");
   std::cout << greeting << std::endl;
   return 0;
}

