#include <GFp/base.h>

extern int lib_counter;
int lib_counter = 0;


int galigeigei(int n) {
  lib_counter++;
  if (n == 0 || n == 1) return 1;
  else return galigeigei(n-1) + galigeigei(n-2);
}
