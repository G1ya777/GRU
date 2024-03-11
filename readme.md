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

# Usage examples

- **Example file tree**</br>

```
├── audios
│   ├── hit.mp3
│   ├── other.mp3
│   ├── song.mp3
│   └── test.mp3
├── images
│   ├── image.png
│   ├── img1.png
│   ├── img2.png
│   └── otherimg.png
├── other
│   ├── data
│   │   └── somefile
│   ├── file
│   └── video.mp4
└── videos
    ├── file1.mkv
    ├── file2.mkv
    ├── file.mkv
    └── other_video.mkv
```

- **`rename-gru --clear --main-filename " - main - " --prefix "prefix" -suffix " - suffix" --numbering ./other`**

```
data -------> prefix - main - 1 - suffix
file -------> prefix - main - 2 - suffix
video.mp4 -------> prefix - main - 3 - suffix.mp4

```

- **rename video files, add tags at the end**<br />
  `rename-gru ./videos  --clear --prefix "MTVS" --numbering --suffix "[AWSMUploader] [1080P]""`

```
file.mkv -------> MTVS1[AWSMUploader] [1080P].mkv
file1.mkv -------> MTVS2[AWSMUploader] [1080P].mkv
file2.mkv -------> MTVS3[AWSMUploader] [1080P].mkv
other_video.mkv -------> MTVS4[AWSMUploader] [1080P].mkv

```

- **rename background files, use random ordering**<br />
  `rename-gru ./images --clear --prefix "bg" --numbering --sort-by 3`

```
image.png -------> bg1.png
img2.png -------> bg2.png
otherimg.png -------> bg3.png
img1.png -------> bg4.png

```

- **keep title and add numbering at the start of the file**<br />
  `rename-gru ./images --main-filename " - " -B --numbering -b --sort-by 3`

```
image.png -------> 1 - image.png
otherimg.png -------> 2 - otherimg.png
img2.png -------> 3 - img2.png
img1.png -------> 4 - img1.png

```

- **rename .mp4 file only**<br />
  `rename-gru ./other --clear --prefix "my_video" --numbering --sort-by 3 -t ".mp4"`

```
video.mp4 -------> my_video1.mp4

```

- **rename folders only**<br />
  `rename-gru ./other --only-folders -n`

```
data -------> data1

```

- **append CRC32 checksum at the end of each filename**<br />
  `rename-gru --crc ./videos`

```
file.mkv -------> file [AA98C6D9].mkv
file1.mkv -------> file1 [26022818].mkv
file2.mkv -------> file2 [1B5BD171].mkv
other_video.mkv -------> other_video [23233DEC].mkv

```

- **restore original file names**<br />
  `rename-gru ./videos --restore "./videos/grubcp-2023-10-22-18-17-23.json"`

# Usage

<!-- rename-gru --help output -->
```
A CLI tool to batch rename files in the provided directory

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
          add a main filename (when --clear isn't used it is added after the original filename)
  -B
          add the main filename before the original filename when --clear isn't used
  -n, --numbering
          Add numbering
      --pad <PAD>
          specify padding for numbering, use --no-pad to disable (defaults to number of digits of number of files - 1)
      --no-pad
          
  -b, --before-main-name
          numbering position, can be before main_filename or after main_filename (default)
      --start <START>
          custom numbering start (default is 1)
      --crc
          add crc32 checksum in hex at the end each  filename (folders won't be renamed if this option is used)
      --replace <Replace> <Replace_with>
          replace old String with a new one
  -e, --extension-replace-by <EXTENSION_REPLACE_BY>
          replace the extension of the original title of all files with a new one (eg: .mp3) (also use -t to target files with a certain extension)
      --sort-by <SORT_BY>
          how to sort files, default is 0 (by name), 1 is by modification date, 2 for date created, 3 for random
      --desc
          set file sort to descending
      --only-files
          only modify files and don't modify folders
      --only-folders
          only modify folders and don't modify files
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
          use to preview file names, w/o writing
      --noconfirm
          perform rename w/o asking for y/n
  -h, --help
          Print help
  -V, --version
          Print version
```
<!-- end of rename-gru --help command output -->

# Found a bug or want a new feature ?

- **[Issues](https://github.com/G1ya777/GRU/issues/new)**
- any help would be appreciated.
