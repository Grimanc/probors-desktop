use std::fs;
use std::path::Path;

fn main() {
    // Ensure a default RGBA PNG icon exists so tauri_build won't panic looking for it.
    let icon_path = Path::new("icons/icon.png");
    if let Err(e) = fs::create_dir_all("icons") {
        panic!("failed to create icons dir: {}", e);
    }

    // If icon exists, ensure it's valid RGBA PNG. Only re-save when not already RGBA
    // to avoid overwriting the file (which would trigger the dev watcher and cause a rebuild loop).
    if icon_path.exists() {
        match image::open(icon_path) {
            Ok(img) => {
                // Only write back if we had to convert (e.g. RGB, grayscale); skip if already RGBA.
                if img.as_rgba8().is_none() {
                    let rgba = img.to_rgba8();
                    if let Err(e) = rgba.save(icon_path) {
                        eprintln!("warning: failed to re-save icon as RGBA: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("warning: failed to open existing icon: {}, will overwrite with default", e);
                let img = image::RgbaImage::from_pixel(16, 16, image::Rgba([0, 0, 0, 0]));
                if let Err(e) = img.save(icon_path) {
                    panic!("failed to write default icon: {}", e);
                }
            }
        }
    } else {
        // Create a 16x16 transparent RGBA PNG
        let img = image::RgbaImage::from_pixel(16, 16, image::Rgba([0, 0, 0, 0]));
        if let Err(e) = img.save(icon_path) {
            panic!("failed to save icon: {}", e);
        }
    }

    tauri_build::build()
}

