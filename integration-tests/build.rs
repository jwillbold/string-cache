use unicase_string_cache_codegen;

use std::env;
use std::path::Path;

fn main() {
    unicase_string_cache_codegen::AtomType::new("TestAtom", "test_atom!")
        .ascii_atoms(&[
            "a",
            "b",
            "address",
            "area",
            "body",
            "font-weight",
            "br",
            "html",
            "head",
            "id",
            "ascii",
        ])
        .write_to_file(&Path::new(&env::var("OUT_DIR").unwrap()).join("test_atom.rs"))
        .unwrap()
}
