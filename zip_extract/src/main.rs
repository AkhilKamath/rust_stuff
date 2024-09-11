use std::io;
use std::fs::{self, File};
use clap::Parser;
use std::path::Path;
use zip::ZipArchive;

#[derive(Parser, Debug)]
#[command(about, version, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String
}

fn main() -> io::Result<()> {

    let args = Args::parse();


    // get the input path from args
    let input_path = Path::new(&args.input);

    // create a file Reader
    let input_zip_file = File::open(&input_path)?;

    // zip archive
    let mut archive = ZipArchive::new(input_zip_file)?;

    // iterate through every archive entry to write it in file system
    for i in 0..archive.len() {
        
        // create a mutable file reader of the ith file
        let mut i_file = archive.by_index(i)?;

        let i_out_file_path = match i_file.enclosed_name() {
            Some(path) => path,
            None => continue
        };

        // limited scope to print file comment
        {
            let comment = i_file.comment();
            println!("{i}: has comment {comment}");
        }

        if i_file.is_dir() {
            if !i_out_file_path.exists() {
                fs::create_dir_all(&i_out_file_path)?;
                println!("{i}: Created dir at path {}", i_out_file_path.display());
            }
        } else {
            let mut i_out_file = File::create(&i_out_file_path)?;

            if let Some(i_out_file_parent_path) = i_out_file_path.parent() {
                if !i_out_file_parent_path.exists() {
                    fs::create_dir_all(&i_out_file_parent_path)?;
                }
            }

            io::copy(&mut i_file, &mut i_out_file)?;
            println!("{i}: Done extrctinf file {} of size {}", i_out_file_path.display(), i_file.size());
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(i_file_mode) = i_file.unix_mode() {
                fs::set_permissions(&i_out_file_path, fs::Permissions::from_mode(i_file_mode))?;
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