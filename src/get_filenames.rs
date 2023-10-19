use std::fs::DirEntry;

pub fn get(filtered_file_list: &Vec<DirEntry>, _incl_hidden: bool) -> Vec<String> {
    // Return the names of the filelist
    #[cfg(windows)]
    //the filtering is done in the read_files module on windows
    let final_file_list = filtered_file_list
        .iter()
        .map(|file| {
            file.file_name()
                .to_str()
                .expect("failed to read one of the files in the specified folder")
                .to_owned()
        })
        .collect();

    #[cfg(unix)]
    let final_file_list = filtered_file_list
        .iter()
        .map(|file| {
            file.file_name()
                .to_str()
                .expect("failed to read one of the files in the specified folder")
                .to_owned()
        })
        .filter(|filename| _incl_hidden || !is_hidden(filename))
        .collect();

    return final_file_list;
}

#[cfg(unix)]
pub fn is_hidden(filename: &str) -> bool {
    filename
        .chars()
        .nth(0)
        .expect("failed check to get hidden files")
        == '.'
}
