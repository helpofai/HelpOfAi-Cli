/*
 * Copyright (c) 2026 HelpOfAi. All rights reserved.
 *
 * This file is part of the HelpOfAi CLI codebase.
 * It is subject to the MIT license terms in the LICENSE.md file
 * found in the top-level directory of this distribution.
 */

use super::*;

#[test]
fn test_helpofai_data_dir() {
    let dir = helpofai_data_dir().unwrap();
    let display = dir.display().to_string();
    assert!(display.ends_with("HelpOfAi") || display.ends_with("helpofai"));
}

#[test]
fn test_engine_dir() {
    let dir = engine_dir().unwrap();
    let display = dir.display().to_string();
    assert!(display.ends_with("codebase-memory"));
}

#[test]
fn test_engine_binary_path() {
    let bin = engine_binary_path().unwrap();
    let display = bin.display().to_string();
    if cfg!(target_os = "windows") {
        assert!(display.ends_with("helpofai-codebase-memory.exe"));
    } else {
        assert!(display.ends_with("helpofai-codebase-memory"));
    }
}
