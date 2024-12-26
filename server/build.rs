/*
 * Copyright (c) 2024 Bastiaan van der Plaat
 *
 * SPDX-License-Identifier: MIT
 */

fn main() {
    openapi_generator::generate_schemas_build(
        "openapi.yml",
        format!(
            "{}/api.rs",
            std::env::var("OUT_DIR").expect("OUT_DIR not set")
        ),
    );
}
