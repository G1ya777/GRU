use std::fs::{self, DirEntry};

pub fn read(
    location: &str,
    sort_by: u8,
    desc: bool,
    only_files: bool,
    only_folders: bool,
) -> Vec<DirEntry> {
    // read the specified directory
    let file_set = fs::read_dir(location).expect("something went wrong with the provided location");

    let file_list = (file_set.collect::<Result<Vec<_>, _>>())
        .expect("failed to read one of the files in provided location");

    let mut filtered_file_list: Vec<DirEntry> = file_list
        .into_iter()
        .filter(|file| !only_files || file.path().is_file())
        .filter(|file| !only_folders || file.path().is_dir())
        .filter(|file| {
            !file
                .file_name()
                .to_str()
                .expect("failed to read one of the files in the specified folder")
                .contains("_grubcp.json")
        })
        .collect();

    // Sort the files by the provided sort_by, 1 is for date modified, 2 is for date created, 0 or any other value is for name
    if sort_by == 1 {
        filtered_file_list.sort_by_cached_key(|file| {
            fs::metadata(file.path())
                .expect("Error when getting files metadata")
                .modified()
                .expect("Error when sorting files")
        });
    } else if sort_by == 2 {
        filtered_file_list.sort_by_cached_key(|file| {
            fs::metadata(file.path())
                .expect("Error when getting files metadata")
                .created()
                .expect("Error when sorting files")
        });
    } else {
        filtered_file_list.sort_by_cached_key(|file| file.file_name())
    }

    // Descending sort if requested
    if desc {
        filtered_file_list.reverse()
    }

    return filtered_file_list;
}
