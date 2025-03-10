fn main() {
    println!("cargo:rerun-if-changed=src/extism-runtime.wasm");

    let fn_macro = "
#define EXTISM_FUNCTION(N) extern void N(ExtismCurrentPlugin*, const ExtismVal*, ExtismSize, ExtismVal*, ExtismSize, void*)
#define EXTISM_GO_FUNCTION(N) extern void N(void*, ExtismVal*, ExtismSize, ExtismVal*, ExtismSize, uintptr_t)
";
    if let Ok(bindings) = cbindgen::Builder::new()
        .with_crate(".")
        .with_language(cbindgen::Language::C)
        .with_no_includes()
        .with_sys_include("stdint.h")
        .with_sys_include("stdbool.h")
        .with_pragma_once(true)
        .with_after_include(fn_macro)
        .rename_item("Size", "ExtismSize")
        .rename_item("ValType", "ExtismValType")
        .rename_item("ValUnion", "ExtismValUnion")
        .rename_item("CurrentPlugin", "ExtismCurrentPlugin")
        .rename_item("CancelHandle", "ExtismCancelHandle")
        .rename_item("Plugin", "ExtismPlugin")
        .rename_item("Function", "ExtismFunction")
        .with_style(cbindgen::Style::Type)
        .generate()
    {
        bindings.write_to_file("extism.h");
    }
}
