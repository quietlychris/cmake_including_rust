#include <iostream>
#include "rust_hello.h"

using namespace std;

int main() {

  cout << "hello, world! -- from c++ main()" << endl;
  hello_from_rust();
  int my_int = 0; // This integer starts as a zero, will change to a four
  my_int = return_a_four();
  cout << "my_int now equals: " << my_int << endl;
  cout << "                   ^ should be a '4' " << endl;

  return 0;
}
