fn main() {
    cc::Build::new()
        .warnings(true)
        .file("src/distorm/src/mnemonics.c")
        .file("src/distorm/src/textdefs.c")
        .file("src/distorm/src/prefix.c")
        .file("src/distorm/src/operands.c")
        .file("src/distorm/src/insts.c")
        .file("src/distorm/src/instructions.c")
        .file("src/distorm/src/distorm.c")
        .file("src/distorm/src/decoder.c")
        .include("src/distorm/include")
        // .include("src/distorm/src")
        .cpp(false)
        .opt_level(2)
        .define("DISTORM_STATIC", "1")
        .define("SUPPORT_64BIT_OFFSET", "1")
        .compile("distorm");

    let bindings = bindgen::Builder::default()
        .header("src/distorm/include/distorm.h")
        .generate()
        .expect("Unable to generate bindings!");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
