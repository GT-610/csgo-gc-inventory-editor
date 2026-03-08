use std::fs;
use std::path::Path;

fn copy_dir_recursive(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("Failed to create directory");
    
    for entry in fs::read_dir(src).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let src_path = entry.path();
        let dest_path = dst.join(src_path.file_name().unwrap());
        
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path);
        } else {
            fs::copy(&src_path, &dest_path).expect(&format!("Failed to copy file: {:?}", src_path));
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=csgo_gc");
    
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let out_dir = std::env::var("OUT_DIR").unwrap();
    
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(4)
        .expect("Failed to find target directory");
    
    let src_csgo_gc = Path::new("csgo_gc");
    let target_csgo_gc = target_dir.join(&profile).join("csgo_gc");
    let target_editor = target_csgo_gc.join("editor");
    
    if src_csgo_gc.exists() {
        if target_editor.exists() {
            fs::remove_dir_all(&target_editor).expect("Failed to remove existing editor directory");
        }
        println!("cargo:warning=Copying csgo_gc to: {:?}", target_csgo_gc);
        copy_dir_recursive(&src_csgo_gc, &target_csgo_gc);
    }
}
