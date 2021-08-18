/* Defines for framebuffer */
#ifndef __ARM_PCS_VFP
#define __ARM_PCS_VFP
#endif

#ifndef LIBC_H
#define LIBC_H

#include <fcntl.h>

#define FRAMEBUFFER_H_VERSION 10000 /* Version 1.00 */

#ifdef __cplusplus
extern "C"
{
#endif

	extern int libc_open(const char* path, int flag);

#ifdef __cplusplus
}
#endif

#endif /* LIBC_H */
