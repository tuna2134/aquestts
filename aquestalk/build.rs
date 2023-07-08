use reqwest::blocking::Client;
use std::env;
use std::fs;
use zip::ZipArchive;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let url = "https://www.a-quest.com/archive/package/aqtk10_lnx_110.zip";
    let client = Client::new();
    let mut res = client.get(url).send().unwrap();
    let mut file = fs::File::create(format!("{}/aqtk10_lnx_110.zip", out_dir)).unwrap();
    std::io::copy(&mut res, &mut file).unwrap();

    let file = fs::File::open(format!("{}/aqtk10_lnx_110.zip", out_dir)).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();
    archive
        .extract(format!("{}/aqtk10_lnx_110", out_dir))
        .unwrap();

    /*
    fs::copy(
        format!(
            "{}/aqtk10_lnx_110/aqtk10_lnx/lib64/libAquesTalk10.so.1.1",
            out_dir
        ),
        format!("{}/libAquesTalk10.so", out_dir),
    )
    */
    .unwrap();
    fs::copy(
        format!(
            "{}/aqtk10_lnx_110/aqtk10_lnx/lib64/libAquesTalk10.so.1.1",
            out_dir
        ),
        format!("{}/libAquesTalk10.so.1", out_dir),
    )
    .unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=dylib=AquesTalk10");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", out_dir);
}
