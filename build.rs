#![deny(unsafe_code)]
use std::env;


#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub const SEARCH: &str = "vendor/sdl2/win64";

fn main() {
    let cwd = env::current_dir().unwrap();
    let mut search = cwd.clone();
    search.push(SEARCH);

    println!(
        "cargo:rustc-link-search=native={}",
        search.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=SDL2");
    
    let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        std::fs::copy(
            format!("{}{}", search.to_string_lossy(), "/SDL2.dll"), 
            format!("{}{}", out_path.to_string_lossy(), "/SDL2.dll")
        ).unwrap();
    }
}
