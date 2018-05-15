#include <rng.h>
#include <timer.h>
#include <stdio.h>

int main(void) {
  uint8_t n;
  
  // Print 1 random byte every 1000 ms
  for (int count = 0; ; count++) {
    rng_sync(&n, 1, 1);
    printf("Random byte %d: %x\r\n", count, n);
    delay_ms(1000);
  }
}
