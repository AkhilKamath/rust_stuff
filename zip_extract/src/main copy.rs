use std::fs::{self, File};
use std::io::{self, BufReader};
use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let input_path = Path::new(&args.input);
    let input_file = File::open(input_path)?;
    let input_reader = BufReader::new(input_file);

    let mut archive = zip::ZipArchive::new(input_reader)?;

    // print the archive

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let output_path = match file.enclosed_name() {
            Some(path) => path,
            None => continue
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment {comment}");
            }
        }

        if file.is_dir() {
            println!("File {i} is a dir and will be extracted to '{}'", output_path.display());
            fs::create_dir_all(&output_path)?;
        } else {
            println!("File {i} is a file of size {} and will be extracted to '{}'", file.size(), output_path.display());

            if let Some(parent_path) = output_path.parent() {
                if !parent_path.exists() {
                    fs::create_dir_all(parent_path)?;
                }
            }

            let mut out_file = File::create(&output_path)?;
            io::copy(&mut file, &mut out_file)?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&output_path, fs::Permissions::from_mode(mode))?;
            }
        }
        #[cfg(not(unix))]
        {
            // Use a different method for non-Unix systems
            // Or return an error if not supported
            Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "Setting permissions with mode is not supported on this platform"))
        }
    }

    Ok(())
}