#ifndef _CRC_H_
#define _CRC_H_

#include <stddef.h>
#include <stdint.h>

/*
 * Calculate the remainder of the data passed through a 24-bit CRC.
 */
uint32_t crc_24(uint8_t *data, size_t data_len);

#endif /* _CRC_H_ */

