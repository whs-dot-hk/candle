fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let builder = bindgen_cuda::Builder::default();
    println!("cargo:info={builder:?}");
    let bindings = builder.build_ptx().unwrap();
    let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write(out_path.join("lib.in")).unwrap();
}
