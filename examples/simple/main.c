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

#include "switcher.h"

#define HEADER_ADDR_A  0x0007FFF8
#define HEADER_ADDR_B  0x000FFFF8
#define HEADER_A       ((image_t *volatile)HEADER_ADDR_A)
#define HEADER_B       ((image_t *volatile)HEADER_ADDR_B)

int main(void)
{
	switcher_boot(switcher_choose(HEADER_A, HEADER_B));

	/* Neither of the images are bootable! */
	while (true)
		;
}

