use tree::TreeBuilder;
use std::fs;

fn main() {
    let path = ".";
    let mut builder = TreeBuilder::new(path);

    // Read the directory and display the structure
    visit_dirs(path, &mut builder).unwrap();

    let tree = builder.build();
    println!("{}", tree);
}

fn visit_dirs(dir: &str, builder: &mut TreeBuilder) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            builder.add_directory(path.display().to_string());
            visit_dirs(&path.display().to_string(), builder)?;
        } else {
            builder.add_file(path.display().to_string(), 0);
        }
    }
    Ok(())
}
