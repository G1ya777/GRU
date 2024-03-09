# GRU : G Rename Utility

A **CLI tool** to batch rename files in the provided directory

<div align="center">
<img alt="Crates.io Version" src="https://img.shields.io/crates/v/rename-gru">
<img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/rename-gru">
<img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/G1ya777/GRU">
<img alt="Crates.io License" src="https://img.shields.io/crates/l/rename-gru">

</div>

# Key features

- **Backup file generated after every use**
- **Avoids cycles**
- **dry-run flag to preview changes**
- **Detects and uses the correct padding for numbering**
- **Ignore/Include hidden files**
- **Random order**
- **Target only one extension**
- **Append CRC32 checksum in Hexadecimal at the end of each file name**
- **Replace extension**
- **Replace n or all occurrences of a string**
- **Convert Unicode to ASCII representation**
- **All the basic stuff like clear original names, numbering, add prefix, remove n first...**
- **Linux and Windows are supported, it should also work on macOS, but I can't test**

# Install

- **using cargo** : `cargo install rename-gru`
- or build from source `git clone https://github.com/G1ya777/GRU && cd GRU` then `cargo build`
- on Arch Linux install from the AUR the package `rename-gru-git`

# Usage

```txt
Usage: rename-gru [OPTIONS] [LOCATION]

Arguments:
  [LOCATION]  location of the files to rename (defaults to the current location)

Options:
      --restore <RESTORE>

  -c, --clear
          use if you wanna clear the original titles and start naming from scratch (the extension will be kept)
  -p, --prefix <PREFIX>
          Prefix to add for the naming
  -s, --suffix <SUFFIX>
          Suffix to add for the naming
  -m, --main-filename <MAIN_FILENAME>
          add a main filename (when --clear it is added before the original filename)
      --separator <SEPARATOR>
          String to use as a separator (default is " - ")
  -n, --numbering
          Add numbering
      --pad <PAD>
          specify padding for numbering, use --no-pad to disable (defaults to number of digits of number of files - 1)
      --no-pad

  -b, --before-main-name
          numbering position, can be before main_filename or after main_filename (default)
      --start <START>
          custom numbering start (default is 1) [default: 1]
      --crc
          add crc32 checksum in hex at the end of the file (folders won't be renamed if this option is used)
      --replace <Replace> <Replace_with>
          replace old String with a new one
  -e, --extension-replace-by <EXTENSION_REPLACE_BY>
          replace the extension of the original title of all files with a new one (eg: .mp3)
      --extension-to-replace <EXTENSION_TO_REPLACE>
          replace the extension of the original title, restrict to provided extension only (defaults to all files), (eg: .mp4)
      --sort-by <SORT_BY>
          how to sort files, default is 0 (by name), 1 is by modification date, 2 for date created, 3 for random
      --desc
          set file sort to descending
      --only-files

      --only-folders

  -f, --first-n-remove <FIRST_N_REMOVE>
          remove the first n characters from the original title
  -l, --last-n-remove <LAST_N_REMOVE>
          remove the last n characters from the original title
  -r, --remove-all <REMOVE_ALL>
          remove all occurrences of a string from the original title
      --remove-n <REMOVE_N>
          remove n occurrences of a string (defaults to 1) from the original title (use --times to specify the number)
      --times <TIMES>
          use with remove_n to specify a number of occurrences to remove
  -t, --target-extension <TARGET_EXTENSION>
          apply renaming only to files ending with provided extension (eg : .jpg)
      --no-bcp
          disable backing up the original filenames
      --no-temp-rename
          disable temporary renaming (used to avoid cycles)
      --incl-hidden
          also rename hidden files (turn off by default)
      --to-ascii
          convert utf-8 to ascii
      --dry-run

  -h, --help
          Print help
  -V, --version
          Print version
```

# Usage examples

- **rename video files, add tags at the end**<br />
  `rename-gru ~/myTvShow --clear --prefix "MTVS" --numbering --suffix "[AWSMUploader] [1080P]"`

- **rename background files, use random ordering**<br />
  `rename-gru ~/myBackgrounds --clear --prefix "bg" --numbering --sort-by 3`

- **keep title and add numbering at the start of the file**<br />
  `rename-gru ~/myBackgrounds --numbering -b --sort-by 3`

- **rename .mp3 file only**<br />
  `rename-gru ~/myMusic --clear --prefix "song" --numbering --sort-by 3 -t ".mp3"`

- **append CRC32 checksum at the end of each filename**<br />
  `rename-gru --crc ~/myAwsmFiles`

- **restore original file names**<br />
  `rename-gru ~/myMusic --restore "~/myMusic/grubcp-2023-10-22-18-17-23.json"`

# Found a bug or want a new feature ?

- **[Issues](https://github.com/G1ya777/GRU/issues/new)**
- any help would be appreciated.
