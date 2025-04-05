#include <stdio.h>

int main(int argc, char * argv[]) 
{
  int8_t a = __INT8_MAX__;
  int8_t b = __INT8_MAX__;
  int8_t c = a*b;

  int16_t a16 = __INT8_MAX__;
  int16_t b16 = __INT8_MAX__;
  int16_t c16 = a16*b16;

  printf("a is %d and b is %d\n", a, b);
  printf("a*b is %d \n", c);

  printf("a16 is %d and b16 is %d\n", a16, b16);
  printf("a*b is %d \n", c16);

  printf("(a16*b16) & 0xff is %d\n", (c16 & 0xff));

  return 0;
}
