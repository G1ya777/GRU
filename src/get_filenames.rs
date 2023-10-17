use std::fs::DirEntry;

pub fn get(filtered_file_list: &Vec<DirEntry>) -> Vec<String> {
    // Return the names of the filelist
    let final_file_list = filtered_file_list
        .iter()
        .map(|file| {
            file.file_name()
                .to_str()
                .expect("failed to read one of the files in the specified folder")
                .to_owned()
        })
        .collect();

    return final_file_list;
}
