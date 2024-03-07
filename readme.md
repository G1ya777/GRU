# GRU : G Rename Utility

A **CLI tool** to batch rename files in the provided directory

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
- **All the basic stuff like clear original names, numbering, add prefix, remove n first...** for an exhaustive list of the options run `rename-gru --help`

# Install

- **using cargo** : `cargo install rename-gru`
- or build from source `git clone https://github.com/G1ya777/GRU && cd GRU` then `cargo build`
- on Arch Linux install from the AUR the package `rename-gru-git`

# Usage examples

- **rename video files, add tags at the end**<br />
  `rename-gru ~/myTvShow --clear --prefix "MTVS" --numbering --suffix "[AWSMUploader] [1080P]"`

- **rename background files, use random ordering**<br />
  `rename-gru ~/myBackgrounds --clear --prefix "bg" --numbering --sort-by 3`

- **keep title and add numbering at the start of the file**
  `rename-gru ~/myBackgrounds --numbering -b --sort-by 3`

- **rename .mp3 file only**<br />
  `rename-gru ~/myMusic --clear --prefix "song" --numbering --sort-by 3 -t ".mp3"`

- **append CRC32 checksum at the end of each file**<br />
  `rename-gru --crc ~/myAwsmFiles`

- **restore original file names**<br />
  `rename-gru ~/myMusic --restore "~/myMusic/grubcp-2023-10-22-18-17-23.json"`

# Found a bug or want a new feature ?

- **[Issues](https://github.com/G1ya777/GRU/issues/new)**
- any help would be appreciated.
