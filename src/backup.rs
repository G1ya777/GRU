use chrono::prelude::*;
use file_id::FileId;
use std::{
    collections::HashMap,
    fs::{self, DirEntry, File},
    io::{BufReader, BufWriter},
};

pub fn backup(filenames_list: &Vec<String>, location: &String) {
    let backup_path = location.to_owned()
        + "/gru_"
        + &(Utc::now().to_string().replace(" ", "_"))
        + "_grubcp.json";
    let file: File = File::create(backup_path).expect("error when creating file for backup!");

    let mut backup_data: Vec<String> = vec![];
    for filename in filenames_list.iter() {
        let filename = filename.to_owned();
        let id = file_id::get_file_id(location.clone() + "/" + &filename)
            .expect("failed to get the id of one of the files.");
        let id_string;
        match id {
            FileId::HighRes { file_id, .. } => id_string = file_id.to_string(),
            FileId::Inode { inode_number, .. } => id_string = inode_number.to_string(),
            FileId::LowRes { file_index, .. } => id_string = file_index.to_string(),
        }

        let data = filename + "#" + &id_string;
        backup_data.push(data)
    }

    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &backup_data).expect("error when saving backup data!")
}

pub fn restore(backup_file_location: String, file_list: &Vec<DirEntry>) {
    let backup_file: File =
        File::open(backup_file_location).expect("error when opening backup file!");
    let reader = BufReader::new(backup_file);
    let backup_data: Vec<String> =
        serde_json::from_reader(reader).expect("failed to read backup file");

    let file_dict: HashMap<String, String> = backup_data
        .iter()
        .map(|filename| {
            let mut parts = filename.split('#');
            let file_name = parts
                .next()
                .expect("failed to get file ID from restore file")
                .to_string();
            let file_id = parts
                .next()
                .expect("failed to get filename from restore file")
                .to_string();
            (file_id, file_name)
        })
        .collect();

    for dir_entry in file_list {
        let id = file_id::get_file_id(dir_entry.path())
            .expect("failed to get the id of one of the files.");

        let id_string: String;
        match id {
            FileId::HighRes { file_id, .. } => id_string = file_id.to_string(),
            FileId::Inode { inode_number, .. } => id_string = inode_number.to_string(),
            FileId::LowRes { file_index, .. } => id_string = file_index.to_string(),
        }
        println!("{id_string}");
        let new_file_path = dir_entry.path().with_file_name(
            file_dict
                .get(&id_string)
                .expect("failed to find one of the files listed in the restore file"),
        );

        fs::rename(dir_entry.path(), &new_file_path).expect("failed to rename");
    }
}
