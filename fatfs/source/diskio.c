#include "ff.h"			/* Obtains integer types */
#include "diskio.h"		/* Declarations of disk functions */

extern DSTATUS disk_status(BYTE);
extern DSTATUS disk_initialize(BYTE);
extern DRESULT disk_read(BYTE, BYTE*, DWORD, UINT);
#if FF_FS_READONLY == 0
extern DRESULT disk_write(BYTE, const BYTE*, DWORD, UINT);
#endif
extern DRESULT disk_ioctl(BYTE, BYTE, void*);
