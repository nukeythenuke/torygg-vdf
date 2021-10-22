use std::{fs::File, path::PathBuf};
fn main() -> Result<(), &'static str> {
    let vdf = PathBuf::from(std::env::var("HOME").map_err(|_| "failed to find home dir")?)
        .join(".steam/root/config/libraryfolders.vdf");
    let kvs =
        torygg_vdf::parse(&mut File::open(vdf).map_err(|_| "failed to open libraryfolders.vdf")?)
            .map_err(|_| "failed to parse libraryfolder.vdf")?;
    for kv in kvs {
        let components = kv.0.components().collect::<Vec<_>>();
        // Keys we want:            🠗
        // libraryfolders/<lib_id>/path
        if let Some(component) = components.get(2) {
            if component.as_os_str() == "path" {
                println!("{}", kv.1);
            }
        }
    }

    Ok(())
}
