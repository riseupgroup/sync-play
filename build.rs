#[cfg(not(debug_assertions))]
use static_files::resource_dir;

fn main() -> std::io::Result<()> {
    println!("cargo::rerun-if-changed=./frontend");
    #[cfg(not(debug_assertions))]
    return resource_dir("./frontend/build").build();
    #[cfg(debug_assertions)]
    return Ok(());
}
