#include <errno.h>
#include <limits.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "lzlib.h"

#ifndef min
#define min(x, y) ((x) <= (y) ? (x) : (y))
#endif

int ffdecompress(FILE *const infile, FILE *const outfile) {
  enum { buffer_size = 16384 };
  uint8_t buffer[buffer_size];
  LZ_Decoder *const decoder = LZ_decompress_open();
  while (true) {
    int len, ret;
    int size = min(buffer_size, LZ_decompress_write_size(decoder));
    if (size > 0) {
      len = fread(buffer, 1, size, infile);
      ret = LZ_decompress_write(decoder, buffer, len);
      if (ret < 0 || ferror(infile))
        break;
      if (feof(infile))
        LZ_decompress_finish(decoder);
    }
    ret = LZ_decompress_read(decoder, buffer, buffer_size);
    if (ret < 0)
      break;
    len = fwrite(buffer, 1, ret, outfile);
    if (len < ret)
      break;
    if (LZ_decompress_finished(decoder) == 1)
      return 0;
  }
  return 1;
}