use crate::clap_tags::Args;
pub fn process(args: &Args, filenames: &Vec<String>) -> Vec<String> {
    let mut new_filenames: Vec<String> = vec![];
    let mut count: u32 = args.start;
    let default_pad =
        u8::try_from(filenames.len().to_string().len()).expect("error with number of files");
    for filename in filenames.iter() {
        let (subname, mut extension): (&str, &str) = get_file_extension(&filename);

        let mut new_filename = subname.to_string();

        if args.target_extension != "" {
            if extension != args.target_extension {
                new_filename += &extension;
                continue;
            }
        }

        if args.clear {
            new_filename = String::new()
        }
        if args.first_n_remove != 0 {
            let index: usize = usize::from(args.first_n_remove);
            new_filename = new_filename[index..].to_string();
        }

        if args.last_n_remove != 0 {
            let index: usize = usize::from(new_filename.len() - usize::from(args.last_n_remove));
            new_filename = new_filename[..index].to_string();
        }

        if args.remove_all != "" {
            let occurance = &args.remove_all;
            new_filename = new_filename.replace(occurance, "");
        } else if args.remove_n != "" {
            let occurance = &args.remove_n;
            new_filename = new_filename.replacen(occurance, "", usize::from(args.times));
        }

        if args.main_filename != "" {
            new_filename += &args.main_filename;
        }
        if args.numbering && args.before_main_name {
            new_filename =
                pad_number(default_pad, args.pad, args.no_pad, count) + "-" + &new_filename;
        }
        if args.prefix != "" {
            new_filename = args.prefix.clone() + &new_filename;
        }

        if args.numbering && !args.before_main_name {
            new_filename =
                new_filename + "-" + &pad_number(default_pad, args.pad, args.no_pad, count);
        }

        if args.extension_replace_by != "" {
            if args.extension_to_replace == "" || args.extension_to_replace == extension {
                extension = &args.extension_replace_by;
            }
        }
        if args.suffix != "" {
            new_filename += &args.suffix;
        }

        new_filename += &extension;

        //last
        count += 1;
        new_filenames.push(new_filename)
    }
    return new_filenames;
}

fn get_file_extension(filename: &str) -> (&str, &str) {
    if filename.contains(".") {
        let extension: usize = filename.rfind(".").expect(
            "something is off with one of the file names! couldn't figure out the extension.",
        );
        let subname: &str = &filename[..extension];
        let extension: &str = &filename[extension..];
        return (subname, extension);
    } else {
        return (filename, "");
    }
}

fn pad_number(default_pad: u8, pad: u8, no_pad: bool, count: u32) -> String {
    let mut pad_value = pad;
    if pad_value == 0 {
        pad_value = default_pad
    } else if no_pad {
        pad_value = 0
    }
    let number = format!("{:0width$}", &count, width = usize::from(pad_value));
    return number;
}
