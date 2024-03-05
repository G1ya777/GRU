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

    let file_list: Vec<DirEntry> = read_files::read(
        &args.location,
        args.sort_by,
        args.desc,
        args.only_files,
        args.only_folders,
        args.incl_hidden && args.restore == "",
    );

    if args.restore != "" {
        backup::restore(args.restore.clone(), &file_list);
    }
    let filenames: Vec<String> =
        get_filenames::get(&file_list, args.incl_hidden && args.restore == "");

    if !filenames.is_empty() {
        let mut crc_list: Vec<String> = Vec::new();
        if args.crc {
            crc_list = read_files::get_crc_list(&file_list);
        }
        if &args.restore == "" {
            let new_filenames = process_filenames::process(&args, &filenames, &crc_list);
            if args.dry_run {
                for filename in &new_filenames {
                    println!("{filename}")
                }
            }
            if !args.no_bcp && !args.dry_run {
                backup::backup(&filenames, &args.location)
            }
            //temporary renaming
            if !args.no_temp_rename {
                let mut temp_filenames: Vec<String> = vec![];
                for i in 0..new_filenames.len() {
                    let temp_filename = "temp-".to_string() + &i.to_string();
                    temp_filenames.push(temp_filename)
                }
                if !args.dry_run {
                    write_new_filenames::write(&args.location, &filenames, &temp_filenames);
                    write_new_filenames::write(&args.location, &temp_filenames, &new_filenames);
                }
            } else {
                if !args.dry_run {
                    write_new_filenames::write(&args.location, &filenames, &new_filenames);
                }
            }
        }
    } else {
        println!("the specified folder is empty")
    }
}
