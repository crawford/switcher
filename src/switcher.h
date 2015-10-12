/*
 * Copyright 2015 Alex Crawford
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef _SWITCHER_H_
#define _SWITCHER_H_

#include <stdint.h>
#include <stdbool.h>

/*
 * Represents a bootable image. The struct should be initialized with 1s except
 * for the length, checksum, and version when it is flashed. The struct shall
 * follow the image it describes such that the checksum immediately follows the
 * image.
 */
typedef struct {
	uint32_t checksum:24;
	uint8_t  version:8;
	uint32_t length:24;
	bool     nValid:1;
	bool     nInvalid:1;
	bool     nSuccess:1;
	bool     nFailure:1;
	uint8_t  nAttempts:4;
} __attribute__ ((packed)) image_t;

/*
 * Mark the image as having successfully booted.
 */
inline void      switcher_set_success(image_t *image);

/*
 * Mark the image as having failed to boot. It will not be considered for boot
 * in the future.
 */
inline void      switcher_set_failure(image_t *image);

/*
 * Choose the newest, valid image to boot. An image is considered valid if it
 * hasn't been marked as failed, it hasn't been marked invalid, its checksum is
 * valid, and it hasn't exhausted its attempts. If no images can be booted,
 * return NULL.
 */
inline image_t * switcher_choose(image_t *a,
                                 image_t *b);

/*
 * Boot the given image. If the image hasn't been marked successfully booted,
 * decrement the number of attempts. If the given image is NULL, this function
 * is a no-op.
 */
inline void      switcher_boot(image_t *image);

#endif /* _SWITCHER_H_ */

