use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version = "0.1.0", about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    #[arg(long, default_value_t = String::new(), conflicts_with_all = RESTORE_CONFLICTS,hide_default_value = true)]
    pub restore: String,

    ///location of the files to rename (defaults to the current location)
    #[arg(index=1, default_value_t = String::from("."),hide_default_value = true)]
    pub location: String,

    ///use if you wanna clear the original titles and start naming from scratch (the extension will be kept)
    #[arg(short, long, default_value_t = false, group = "clear_group")]
    pub clear: bool,

    /// Prefix to add for the naming
    #[arg(short, long, default_value_t = String::new(),hide_default_value = true)]
    pub prefix: String,

    /// Suffix to add for the naming
    #[arg(short, long, default_value_t = String::new(),hide_default_value = true)]
    pub suffix: String,

    /// add a main filename (when --clear isn't used it is added after the original filename)
    #[arg(short, long, default_value_t = String::new(),hide_default_value = true)]
    pub main_filename: String,

    /// add the main filename before the original filename when --clear isn't used
    #[arg(short = 'B', default_value_t = false)]
    pub main_before: bool,

    /// Add numbering
    #[arg(short, long, default_value_t = false)]
    pub numbering: bool,

    /// specify padding for numbering, use --no-pad to disable (defaults to number of digits of number of files - 1)
    #[arg(long, default_value_t = 0, hide_default_value = true)]
    pub pad: u8,
    //disable padding
    #[arg(long, default_value_t = false)]
    pub no_pad: bool,
    /// numbering position, can be before main_filename or after main_filename (default)
    #[arg(short, long, default_value_t = false)]
    pub before_main_name: bool,
    /// custom numbering start (default is 1)
    #[arg(long, default_value_t = 1, hide_default_value = true)]
    pub start: u32,

    ///add crc32 checksum in hex at the end each  filename (folders won't be renamed if this option is used)
    #[arg(long, default_value_t = false)]
    pub crc: bool,

    ///replace old String with a new one
    #[arg(long, num_args = 2, value_names=["Replace","Replace_with"], hide_default_value = true)]
    pub replace: Vec<String>,

    ///replace the extension of the original title of all files with a new one (eg: .mp3) (also use -t to target files with a certain extension)
    #[arg(short, long, default_value_t = String::new(),hide_default_value = true)]
    pub extension_replace_by: String,

    ///how to sort files, default is 0 (by name), 1 is by modification date, 2 for date created, 3 for random.
    #[arg(long, default_value_t = 0, hide_default_value = true)]
    pub sort_by: u8,

    ///set file sort to descending
    #[arg(long, default_value_t = false)]
    pub desc: bool,

    ///only modify files and don't modify folders
    #[arg(long, default_value_t = false, group = "file_or_folder_group")]
    pub only_files: bool,

    ///only modify folders and don't modify files
    #[arg(long, default_value_t = false, group = "file_or_folder_group")]
    pub only_folders: bool,

    ///remove the first n characters from the original title
    #[arg(
        short,
        long,
        default_value_t = 0,
        group = "clear_group",
        hide_default_value = true
    )]
    pub first_n_remove: u8,

    ///remove the last n characters from the original title
    #[arg(
        short,
        long,
        default_value_t = 0,
        group = "clear_group",
        hide_default_value = true
    )]
    pub last_n_remove: u8,

    ///remove all occurrences of a string from the original title
    #[arg(short, long, default_value_t = String::new(),groups=["remove_group", "remove_n_group"],hide_default_value = true)]
    pub remove_all: String,

    ///remove n occurrences of a string (defaults to 1) from the original title (use --times to specify the number)
    #[arg(long, default_value_t = String::new(), groups=["remove_group","clear_group"],hide_default_value = true)]
    pub remove_n: String,
    ///use with remove_n to specify a number of occurrences to remove
    #[arg(long, default_value_t = 1, groups = ["remove_n_group", "clear_group"],hide_default_value = true)]
    pub times: u8,

    ///apply renaming only to files ending with provided extension (eg : .jpg)
    #[arg(short, long, default_value_t = String::new(),hide_default_value = true)]
    pub target_extension: String,

    /// disable backing up the original filenames
    #[arg(long, default_value_t = false)]
    pub no_bcp: bool,

    /// disable temporary renaming (used to avoid cycles)
    #[arg(long, default_value_t = false)]
    pub no_temp_rename: bool,

    /// also rename hidden files (turn off by default)
    #[arg(long, default_value_t = false)]
    pub incl_hidden: bool,

    /// convert utf-8 to ascii
    #[arg(long, default_value_t = false)]
    pub to_ascii: bool,

    /// use to preview file names, w/o writing.
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// perform rename w/o asking for y/n
    #[arg(long, default_value_t = false)]
    pub noconfirm: bool,
}

// used for disabling when restore
const RESTORE_CONFLICTS: [&str; 27] = [
    "clear",
    "prefix",
    "suffix",
    "main_filename",
    "numbering",
    "pad",
    "no_pad",
    "before_main_name",
    "start",
    "replace",
    "extension_replace_by",
    "sort_by",
    "desc",
    "only_files",
    "only_folders",
    "first_n_remove",
    "last_n_remove",
    "remove_all",
    "remove_n",
    "times",
    "crc",
    "target_extension",
    "no_bcp",
    "incl_hidden",
    "dry_run",
    "to_ascii",
    "noconfirm",
];
