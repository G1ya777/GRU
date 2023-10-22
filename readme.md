# GRU : G Rename Utility

A **cli tool** to batch rename all the files in the provided directory

# Key features

- **Backup file generated after every use**
- **Avoids cycles**
- **dry-run flags to preview changes**
- **Detects and uses the correct padding for numbering**
- **Ignore/Include hidden files**
- **Random order**
- **Target only one extension**
- **Replace extension**
- **Replace n or all occurrences of a string**
- **All the basic stuff like clear og names, numbering, add prefix, remove n first...** for an exhaustive list of the options run `rename-gru --help`

# Install

- **using crago** : `cargo install rename-gru`
- or build from source `git clone https://github.com/G1ya777/GRU && cd GRU` then `cargo build`
- or use one of the binaries in the release section

# Usage examples

- **rename video files, add tags at the end**<br />
  `rename-gru ~/myTvShow --clear --prefix "MTVS-" --numbering --suffix "\ \[AWSMUploader\]\ \[1080P\]"`

- **rename background files, use random ordering**<br />
  `rename-gru ~/myBackgrounds --clear --prefix "bg-" --numbering --sort-by 3`

- **rename .mp3 file only**<br />
  `rename-gru ~/myMusic --clear --prefix "song-" --numbering --sort-by 3 -t ".mp3"`

- **restore original file names**<br />
  `rename-gru ~/myMusic --restore "~/myMusic/grubcp-2023-10-22-18-17-23.json"`

# Found a bug or want a new feature ?

- **`https://github.com/G1ya777/GRU/issues/new`**
- any help would be appreciated.
