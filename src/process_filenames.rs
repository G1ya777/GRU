use crate::clap_tags::Args;
use any_ascii::any_ascii;

pub fn process(args: &Args, filenames: &Vec<String>, crc_list: &Vec<String>) -> Vec<String> {
    let mut new_filenames: Vec<String> = vec![];
    let mut count: u32 = args.start;
    let default_pad =
        u8::try_from(filenames.len().to_string().len()).expect("error with number of files");
    for (index, filename) in filenames.iter().enumerate() {
        let (sub_name, mut extension): (&str, &str) = get_file_extension(&filename);

        let mut new_filename = sub_name.to_string();

        if args.target_extension != "" {
            if extension != args.target_extension {
                new_filename += &extension;
                continue;
            }
        }

        if args.to_ascii {
            new_filename = any_ascii(&filename)
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
            let occurrence = &args.remove_all;
            new_filename = new_filename.replace(occurrence, "");
        } else if args.remove_n != "" {
            let occurrence = &args.remove_n;
            new_filename = new_filename.replacen(occurrence, "", usize::from(args.times));
        }

        if args.replace.len() == 2 {
            new_filename = new_filename.replace(&args.replace[0], &args.replace[1]);
        }

        if args.main_filename != "" {
            new_filename += &args.main_filename;
        }
        if args.numbering && args.before_main_name {
            new_filename = pad_number(default_pad, args.pad, args.no_pad, count)
                + &args.separator
                + &new_filename;
        }
        if args.prefix != "" {
            new_filename = args.prefix.clone() + &new_filename;
        }

        if args.numbering && !args.before_main_name {
            new_filename = new_filename
                + &args.separator
                + &pad_number(default_pad, args.pad, args.no_pad, count);
        }

        if args.extension_replace_by != "" {
            if args.extension_to_replace == "" || args.extension_to_replace == extension {
                extension = &args.extension_replace_by;
            }
        }
        if args.suffix != "" {
            new_filename += &args.suffix;
        }
        if args.crc {
            new_filename = new_filename + " [" + &crc_list[index] + "]";
        }
        if extension != "" {
            new_filename += extension;
        }

        //last
        count += 1;
        new_filenames.push(new_filename)
    }
    new_filenames
}

fn get_file_extension(filename: &str) -> (&str, &str) {
    if filename.contains(".") {
        let extension: usize = filename.rfind(".").expect(
            "something is off with one of the file names! couldn't figure out the extension.",
        );
        let sun_name: &str = &filename[..extension];
        let extension: &str = &filename[extension..];
        return (sun_name, extension);
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
    number
}
