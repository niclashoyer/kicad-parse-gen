#[cfg(not(feature = "serde_derive"))]
mod inner {

    extern crate serde_codegen;

    use std::env;
    use std::path::Path;
    use std::fs;

    pub fn main() {
        let out_dir = env::var_os("OUT_DIR").unwrap();
        
        let src = Path::new("src/footprint/data2.rs.in");
        let dst_dir = Path::new(&out_dir).join("footprint");
        fs::create_dir(&dst_dir).unwrap();
        let dst = dst_dir.join("data2.rs");
        
        serde_codegen::expand(&src, &dst).unwrap();
    }
}

#[cfg(feature = "serde_derive")]
mod inner {
    pub fn main() {}
}

fn main() {
    inner::main();
}

