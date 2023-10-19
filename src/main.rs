use std::{fs::DirEntry, vec};

use clap::Parser;

//modules
mod backup;
mod clap_tags;
mod get_filenames;
mod process_filenames;
mod read_files;
mod write_new_filenames;

fn main() {
    let args = clap_tags::Args::parse();

    let filelist: Vec<DirEntry> = read_files::read(
        &args.location,
        args.sort_by,
        args.desc,
        args.only_files,
        args.only_folders,
        args.incl_hidden && args.restore == "",
    );
    if args.restore != "" {
        backup::restore(args.restore.clone(), &filelist);
    }
    let filenames: Vec<String> =
        get_filenames::get(&filelist, args.incl_hidden && args.restore == "");

    if !filenames.is_empty() {
        if &args.restore == "" {
            let new_filenames = process_filenames::process(&args, &filenames);
            if !args.no_bcp {
                backup::backup(&filenames, &args.location)
            }
            //temprary renaming
            if !args.no_temp_rename {
                let mut temp_filenames: Vec<String> = vec![];
                for i in 0..new_filenames.len() {
                    let temp_filename = "temp-".to_string() + &i.to_string();
                    temp_filenames.push(temp_filename)
                }
                write_new_filenames::write(&args.location, &filenames, &temp_filenames);
                write_new_filenames::write(&args.location, &temp_filenames, &new_filenames);
            } else {
                write_new_filenames::write(&args.location, &filenames, &new_filenames);
            }
        }
    } else {
        println!("the specified folder is empty")
    }
}
