#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include "carg_parser.h"
#include "cbuffer.c"
#include "decoder.c"
#include "decoder.h"
#include "encoder.c"
#include "encoder.h"
#include "encoder_base.c"
#include "encoder_base.h"
#include "fast_encoder.c"
#include "fast_encoder.h"
#include "lzip.h"
#include "lzlib.h"
#include <errno.h>
#include <limits.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#ifndef min
#define min(a,b) ((a)<(b)?(a):(b))
#endif