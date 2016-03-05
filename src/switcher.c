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

#include <stddef.h>
#include "crc.h"
#include "switcher.h"

static        bool      _can_boot(image_t *image);
static inline bool      _checksum_valid(const image_t *image);
static inline uint8_t * _image_start(const image_t *image);

/*
 * Mark the image as having successfully booted.
 */
void switcher_set_success(image_t *image)
{
	image->nSuccess = 0;
}

/*
 * Mark the image as having failed to boot.
 */
void switcher_set_failure(image_t *image)
{
	image->nFailure = 0;
}

/*
 * Choose the newest valid image to boot. Return NULL if neither image can be
 * booted.
 */
image_t * switcher_choose(image_t *a,
                                 image_t *b)
{
	bool can_boot_a = _can_boot(a);
	bool can_boot_b = _can_boot(b);

	if (!can_boot_a && !can_boot_b)
		return NULL;

	if (can_boot_a && !can_boot_b)
		return a;

	if (!can_boot_a && can_boot_b)
		return b;

	return (a >= b) ? a : b;
}

/*
 * Boot the given image, initializing the stack and instruction pointers. If
 * the given image is NULL, return.
 */
void switcher_boot(image_t *image)
{
	if (!image)
		return;

	if (image->nSuccess)
		image->nAttempts = (uint8_t)(image->nAttempts << 1);

	/* Set the stack pointer to 0
	   Set the instruction pointer past the header */
	__asm volatile (
			"mov sp, %0;"
			"mov ip, %1;"
			:
			: "r" (0),
			  "r" (_image_start(image))
			);

	__builtin_unreachable();
}

/*
 * Determine if the image can be booted. If the image has not been marked as
 * having succeeded or failed to boot, its checksum will be verified and its
 * validity recorded.
 */
static bool _can_boot(image_t *image)
{
	if (!image->nFailure)
		return false;

	if (!image->nSuccess)
		return true;

	if (image->nValid)
	{
		if (image->nInvalid)
			return false;

		if (!_checksum_valid(image))
		{
			image->nInvalid = 0;
			return false;
		}
		image->nValid = 0;
	}

	return image->nAttempts;
}

/*
 * Determine if the image is valid by running it through a CRC.
 */
static inline bool _checksum_valid(const image_t *image)
{
	/* Run the image and its checksum (the first 3 bytes in the image header)
	   through the CRC. The result must be 0. */
	return (!crc_24(_image_start(image), image->length + 3));
}

/*
 * Calculate the start of the image given the header. The header is positioned
 * directly after the image.
 */
static inline uint8_t * _image_start(const image_t *image)
{
	return ((uint8_t *)image - image->length);
}

