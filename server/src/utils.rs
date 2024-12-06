/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use std::collections::BTreeMap;

pub fn convert_garde_report(report: garde::Report) -> BTreeMap<String, Vec<String>> {
    let mut errors: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (key, value) in report.iter() {
        let key = key.to_string();
        if let Some(errors) = errors.get_mut(&key) {
            errors.push(value.to_string());
            continue;
        }
        errors.insert(key, vec![value.to_string()]);
    }
    errors
}
