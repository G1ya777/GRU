use chrono::prelude::*;
use clap::Parser;
use std::io;
use std::panic;
use std::process;
use std::{fs::DirEntry, vec};

//modules
mod backup;
mod clap_tags;
mod get_filenames;
mod process_filenames;
mod read_files;
mod write_new_filenames;

fn main() {
    let args = clap_tags::Args::parse();
    let mut backup_path = String::new();

    let file_list: Vec<DirEntry> = read_files::read(
        &args.location,
        args.sort_by,
        args.desc,
        args.only_files,
        args.only_folders,
        args.incl_hidden && args.restore == "",
        args.crc,
        args.target_extension != "",
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
                for i in 0..filenames.len() {
                    if new_filenames[i] != "" {
                        println!("{} -------> {}", filenames[i], new_filenames[i]);
                    }
                }
            }
            //temporary renaming
            if !args.no_temp_rename {
                let mut temp_filenames: Vec<String> = vec![];
                let mut temp_filename;
                for new_filename in new_filenames.iter() {
                    if new_filename != "" {
                        temp_filename = "temp-".to_string()
                            + &Utc::now()
                                .to_string()
                                .replace(" ", "_")
                                .replace(":", "_")
                                .replace(".", "_");
                    } else {
                        temp_filename = String::from("");
                    }
                    temp_filenames.push(temp_filename)
                }
                let mut sorted_new_filenames: Vec<String> = new_filenames
                    .clone()
                    .into_iter()
                    .filter(|filename| filename != "")
                    .collect();
                sorted_new_filenames.sort_by_cached_key(|filename| filename.to_owned());

                //length of sorted_new_filenames before dedup
                let len_sorted_new_filenames = sorted_new_filenames.len();

                //removing of duplicates
                sorted_new_filenames.dedup();

                if sorted_new_filenames.len() != len_sorted_new_filenames {
                    println!(
                        "Naming conflict detected! Try using -n option. Renaming was aborted."
                    );
                    process::exit(0);
                }

                if !args.dry_run {
                    if !args.noconfirm {
                        for i in 0..filenames.len() {
                            if new_filenames[i] != "" {
                                println!("{} -------> {}", filenames[i], new_filenames[i]);
                            }
                        }
                        println!("Perform renaming ? (y/N)");
                        let mut confirm = String::new();
                        io::stdin()
                            .read_line(&mut confirm)
                            .expect("Failed to read confirmation");
                        confirm = confirm.trim().to_string();
                        if confirm != "y" || confirm != "Y" {
                            println!("Renaming was aborted.");
                            process::exit(0);
                        }
                    }
                    if !args.no_bcp {
                        backup_path = backup::backup(&filenames, &args.location);
                    }
                    write_new_filenames::write(&args.location, &filenames, &temp_filenames);
                    let rename_result = panic::catch_unwind(|| {
                        write_new_filenames::write(&args.location, &temp_filenames, &new_filenames);
                    });
                    match rename_result {
                        Ok(_) => {
                            println!("Renaming performed.")
                        }
                        Err(_) => {
                            println!("triggered");
                            if !args.no_bcp && !args.dry_run {
                                let file_list: Vec<DirEntry> = read_files::read(
                                    &args.location,
                                    args.sort_by,
                                    args.desc,
                                    args.only_files,
                                    args.only_folders,
                                    args.incl_hidden && args.restore == "",
                                    args.crc,
                                    args.target_extension != "",
                                );
                                backup::restore(backup_path, &file_list);
                                println!("a backup restore was attempted");
                            }
                        }
                    }
                }
            } else {
                if !args.dry_run {
                    write_new_filenames::write(&args.location, &filenames, &new_filenames);
                }
            }
        }
    } else {
        println!("The specified folder is empty.")
    }
}
