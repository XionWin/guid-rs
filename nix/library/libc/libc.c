#include "libc.h"
int libc_open(const char* path, int flag)
{
    return open(path, flag);
}