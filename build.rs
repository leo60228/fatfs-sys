use bindgen;
use cc;
use std::env;
use std::path::PathBuf;
use std::fs::read_dir;

fn main() {
    let mut build = cc::Build::new();
    build.warnings(false);
    build.include("hactool");
    build.include("hactool/mbedtls/include");
    for ent in read_dir("hactool/mbedtls/library/").unwrap().chain(read_dir("hactool/").unwrap()) {
        if let Ok(ent) = ent {
            if ent.path().extension().map(|e| e.to_string_lossy() == "c").unwrap_or(false) &&
               ent.path().file_name().map(|e| e.to_string_lossy() != "main.c").unwrap_or(false) {
                build.file(ent.path());
            }
        }
    }
    build.compile("libhactool.a");

    let bindings = if let Ok(_) = std::env::var("BINDGEN_LIBNX") {
        bindgen::Builder::default()
            .trust_clang_mangling(false)
            .rust_target(bindgen::RustTarget::Nightly)
            .ctypes_prefix("lang_items")
            .header("wrapper.h")
            .clang_arg("-I/opt/devkitpro/libnx/include")
            .clang_arg("-I/opt/devkitpro/devkitA64/aarch64-none-elf/include")
            .clang_arg("-I/opt/devkitpro/devkitA64/lib/gcc/aarch64-none-elf/8.2.0/include")
			.default_enum_style(bindgen::EnumVariation::Rust)
            .blacklist_type("u8")
            .blacklist_type("u16")
            .blacklist_type("u32")
            .blacklist_type("u64")
			.blacklist_type("FILE")
			.raw_line("pub type FILE = libc::FILE;")
            .generate()
			.expect("Couldn't generate bindings!")
    } else {
		bindgen::Builder::default()
			.clang_arg("-Ihactool")
			.clang_arg("-Ihactool/mbedtls/include")
			.header("wrapper.h")
			.default_enum_style(bindgen::EnumVariation::Rust)
			.blacklist_type("FILE")
			.raw_line("pub type FILE = libc::FILE;")
			.generate()
			.expect("Couldn't generate bindings!")
	};

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
