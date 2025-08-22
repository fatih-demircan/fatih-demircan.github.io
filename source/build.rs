use serde_json;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;

fn main() {
    let mut files: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    for f in fs::read_dir("public\\logos")
        .unwrap()
        .into_iter()
        .map(|f| f.unwrap().path())
    {
        if f.is_dir() {
            let k = f.file_stem().unwrap().to_string_lossy().to_string();
            let mut v: BTreeMap<String, String> = BTreeMap::new();
            let paths = fs::read_dir(f)
                .unwrap()
                .into_iter()
                .map(|f| f.unwrap().path());
            for path in paths {
                let name = path.file_stem().unwrap().to_string_lossy().to_string();
                let fpath = path.to_string_lossy().to_string();
                v.insert(name, fpath);
            }
            files.insert(k, v);
        }
    }

    let json = serde_json::to_string_pretty(&files).unwrap();
    let mut file = fs::File::create("public\\logos.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
    println!("cargo:rerun-if-changed=public\\logos");
}
