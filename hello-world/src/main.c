/*
 * Copyright (c) 2012-2014 Wind River Systems, Inc.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/logging/log.h>

#define LOG_LEVEL LOG_LEVEL_INF

LOG_MODULE_REGISTER(zephyr_rs);

/* External declaration for main in Rust. */
void rust_main(void);

int main(void)
{
	LOG_INF("Starting app");
	rust_main();
	LOG_INF("Done with app");
}
