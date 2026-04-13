use std::fs;
use std::path::Path;

fn watch_dir_recursive(path: &Path) {
    if !path.exists() {
        return;
    }

    println!("cargo:rerun-if-changed={}", path.display());

    for entry in fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let entry_path = entry.path();

        if entry_path.is_dir() {
            watch_dir_recursive(&entry_path);
        } else {
            println!("cargo:rerun-if-changed={}", entry_path.display());
        }
    }
}

fn should_copy_file(src: &Path, dst: &Path) -> bool {
    let Ok(src_meta) = fs::metadata(src) else {
        return false;
    };

    let Ok(dst_meta) = fs::metadata(dst) else {
        return true;
    };

    src_meta.len() != dst_meta.len()
        || src_meta
            .modified()
            .ok()
            .zip(dst_meta.modified().ok())
            .is_none_or(|(src_time, dst_time)| src_time > dst_time)
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("Failed to create directory");

    for entry in fs::read_dir(src).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let src_path = entry.path();
        let dest_path = dst.join(src_path.file_name().unwrap());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path);
        } else if should_copy_file(&src_path, &dest_path) {
            fs::copy(&src_path, &dest_path)
                .unwrap_or_else(|_| panic!("Failed to copy file: {:?}", src_path));
        }
    }
}

fn main() {
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(4)
        .expect("Failed to find target directory");

    let src_csgo_gc = Path::new("csgo_gc");
    let target_csgo_gc = target_dir.join(&profile).join("csgo_gc");

    if src_csgo_gc.exists() {
        watch_dir_recursive(src_csgo_gc);
        copy_dir_recursive(src_csgo_gc, &target_csgo_gc);
    }
}
