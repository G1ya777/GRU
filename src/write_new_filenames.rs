use renamore::rename_exclusive;

pub fn write(location: &str, filenames: &Vec<String>, new_filenames: &Vec<String>) {
    for (new_filename, filename) in new_filenames.iter().zip(filenames.iter()) {
        if new_filename != "" {
            // Get the path for the new filename
            let file_path = location.to_string() + "/" + &filename;
            let new_file_path = location.to_string() + "/" + &new_filename;

            // Rename the file to the new path
            rename_exclusive(file_path, &new_file_path)
                .expect("Failed to rename, try removing --no-temp-rename flag");
        }
    }
}
