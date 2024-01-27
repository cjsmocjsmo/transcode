// use std::fs;
use image::{DynamicImage, ImageFormat};
use walkdir::WalkDir;

pub fn walk_dir(apath: String) {
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            let ext_split = fname.split(".").collect::<Vec<&str>>();
            let ext = ext_split.last().unwrap();
            if ext == &"png" || ext == &"PNG" || ext == &"bmp" {
                print!("{} -> ", fname);
                
            }
        };
    }
    // println!("Total files: {}\n", idx);

    // keeper_vec
}
// ...
