use std::fs;
use std::io;

fn main() {
    // to gracefully execute and exit program.
    std::process::exit(execute())
}

fn execute() -> i32 {
    // extract env vars from cli
    let args: Vec<_> = std::env::args().collect();
    // if not enough params exit.
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }
    // get the filename
    let fname = std::path::Path::new(&*args[1]);
    // open the file 
    let file = fs::File::open(&fname).unwrap();

    // using archive reader, read contents
    let mut archive = zip::ZipArchive::new(file).unwrap();

    // keep track of found files
    let mut file_count = 1;
    // extract all files in the archive starting from 0..
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        // set path for extracting files
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue 
        };
        // checking for comments
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }
        // if this is folder
        if (*file.name()).ends_with('/') {
            println!("new directory {:?}" , outpath);
            // recursively create new dirs
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                file_count,
                outpath.display(),
                file.size()
            );
            
            // if this was a standalone file then create an extraction folder
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
            file_count+=1;
        }

        // set permissions for unix 
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode).unwrap());
            }
        }

    }
    0
}
