/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

/*
 * Copyright 2023 Seigo Tanimura <seigo.tanimura@gmail.com> and contributors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/*
 * MIT License
 *
 * Copyright (c) 2023 Seigo Tanimura <seigo.tanimura@gmail.com> and contributors.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <libcw2.h>
#include <libcw_debug.h>

#include "unixcw_libcw_demo.h"


extern cw_debug_t cw_debug_object;
extern cw_debug_t cw_debug_object_dev;
#ifdef deprecated
extern cw_debug_t cw_debug_object_ev;
#endif /* deprecated */


enum cw_return_values
unixcw_libcw_demo_1(const char *msg)
{
	enum cw_return_values ret;
	cw_ret_t cw_ret;
	cw_gen_config_t cw_gen_config;
	cw_gen_t *cw_gen;

	ret = CW_SUCCESS;

	printf("%s: starting.\n", __func__);

	/* Enable the debug log. */
	cw_debug_set_flags(&cw_debug_object, CW_DEBUG_MASK);
	cw_debug_object.level = CW_DEBUG_DEBUG;
	cw_debug_set_flags(&cw_debug_object_dev, CW_DEBUG_MASK);
	cw_debug_object_dev.level = CW_DEBUG_INFO;

	/* Configure the generator. */
	memset(&cw_gen_config, 0, sizeof(cw_gen_config));
	cw_gen_config.sound_system = CW_AUDIO_NULL;
	strncpy(cw_gen_config.sound_device,
		CW_DEFAULT_NULL_DEVICE,
		sizeof(cw_gen_config.sound_device));

	/*
	 * Create the generator.
	 * Use the NULL audio device for the demo purpose.
	 */
	cw_gen = cw_gen_new(&cw_gen_config);
	if (NULL == cw_gen) {
		perror("cw_gen_new");
		ret = CW_FAILURE;
		goto err0;
	}

	/* Start the generator. */
	cw_ret = cw_gen_start(cw_gen);
	if (CW_SUCCESS != cw_ret) {
		perror("cw_gen_start");
		ret = CW_FAILURE;
		goto err1;
	}

	/* Start sending the message in the background. */
	cw_ret = cw_gen_enqueue_string(cw_gen, msg);
	if (CW_SUCCESS != cw_ret) {
		perror("cw_gen_enqueue_string");
		ret = CW_FAILURE;
		goto err2;
	}

	/* Wait for the send to complete. */
	cw_ret = cw_gen_wait_for_queue_level(cw_gen, 0);
	if (CW_SUCCESS != cw_ret) {
		perror("cw_gen_wait_for_queue_level");
		ret = CW_FAILURE;
		goto err2;
	}

err2:
	/* Stop the generator. */
	cw_ret = cw_gen_stop(cw_gen);
	if (CW_SUCCESS != cw_ret) {
		fprintf(stderr, "cw_gen_stop() failed.\n");
		ret = CW_FAILURE;
		goto err1;
	}

err1:
	/* Clean up the generator. */
	cw_gen_delete(&cw_gen);

err0:
	return (ret);
}

