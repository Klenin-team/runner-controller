/*
 * List of avalibale languages.
 *
 * Thus must be configurable, but, unfortunately I have no clue how to do this and I have no time
 * to figure this out.
 */

use std::collections::HashMap;

use super::Language;

pub fn set_languages () -> HashMap<&'static str, Language> {
    let mut languages = HashMap::new();
    languages.insert("python", Language {
        filename: "solution.py",
        compilible: false,
        compile_command: vec![],
        execute_command: vec!["/usr/bin/python", "/space/solution.py"]
    });
    languages.insert("gcc", Language {
        filename: "solution.c",
        compilible: true,
        compile_command: vec!["/usr/bin/gcc", "-o", "/space/a.out",  "-lm", "/space/solution.c"],
        execute_command: vec!["/space/a.out"]
    });
    languages.insert("g++", Language {
        filename: "solution.cpp",
        compilible: true,
        compile_command: vec!["/usr/bin/g++", "-o", "/space/a.out", "-lm", "/space/solution.cpp"],
        execute_command: vec!["/space/a.out"]
    });
    languages.insert("clang", Language {
        filename: "solution.c",
        compilible: true,
        compile_command: vec!["/usr/bin/clang", "-o", "/space/a.out",  "-lm", "/space/solution.c"],
        execute_command: vec!["/space/a.out"]
    });
    languages.insert("clang++", Language {
        filename: "solution.cpp",
        compilible: true,
        compile_command: vec!["/usr/bin/clang++", "-o", "/space/a.out",  "-lm", "/space/solution.cpp"],
        execute_command: vec!["/space/a.out"]
    });
    languages.insert("rust", Language {
        filename: "solution.rs",
        compilible: true,
        compile_command: vec!["/usr/bin/rustc", "-o", "/space/a.out", "/space/solution.rs"],
        execute_command: vec!["/space/a.out"]
    });

    languages
}
