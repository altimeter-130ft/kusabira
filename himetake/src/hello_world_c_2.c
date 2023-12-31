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

#include <stdint.h>
#include <stdio.h>

#include "hello_world_c_types.h"

#include "hello_world_import_from_rust.h"

#include "hello_world_c_2.h"

static int32_t hello_world_c_2_callback(const int8_t *msg);

int32_t
hello_world_c_2_fn(hello_world_c_2_cb callback)
{
	int32_t ret, ret2;

	ret = printf("Hello world 2, printed in C, callback = %p.\n", callback);

	ret2 = callback((const int8_t *)"from C");
	printf("hello_world_c_2_fn callback; ret2 = %d.\n", ret2);

	ret2 = hello_world_rust_2_fn(hello_world_c_2_callback);
	printf("hello_world_rust_2_fn; ret2 = %d.\n", ret2);

	return ret;
}

static int32_t
hello_world_c_2_callback(const int8_t *msg)
{
	int32_t ret;

	ret = printf("Hello world 2, printed in C, %s.\n", msg);

	return ret;
}
