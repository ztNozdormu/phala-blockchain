typedef unsigned int uint32_t;
typedef unsigned int size_t;

extern int lib_counter;
int lib_counter = 0;


int galigeigei(int n) {
  lib_counter++;
  if (n == 0 || n == 1) return 1;
  else return galigeigei(n-1) + galigeigei(n-2);
}

int galigeigei3(int n) {
  lib_counter++;
  if (n == 0 || n == 1) return 1;
  else return galigeigei3(n-1) + galigeigei3(n-2);
}

void a_new_function(int n) {
  galigeigei(n);
}

void GFp_bn_mul_mont(uint32_t *rp, const uint32_t *ap, const uint32_t *bp, const uint32_t *np, const uint32_t *n0, size_t num) {
  galigeigei3(num);
}