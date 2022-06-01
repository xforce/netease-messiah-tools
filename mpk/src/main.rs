use clap::Parser;
use std::fmt::Debug;

use messiah_mpk::MPKFileReader;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(
        help = "Input .mpkinfo file, we derive all the required files based on that and it's location"
    )]
    mpkinfo_file: String,

    #[clap(help = "Desired output directory to extract files to")]
    out_dir: String,
}

/*
struct MPKInfo {
    int32 version;
    int32 file_count;

    struct MPKInfoFile {
        ushort name_length;
        char name[name_length];
        int32 offset;
        int32 size;
        struct FileFlags {
            BitfieldDisablePadding();
            char is_folder : 1;
            int32 file_number : 31;
        } flags;
    } files[info_header.file_count] <size=MPKInfoFileSize,optimize=false>;
} info_header;
*/

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let args = Args::parse();

    let mpkinfo_file = std::path::Path::new(&args.mpkinfo_file);
    let reader = MPKFileReader::new(mpkinfo_file)?;
    reader.extract_files(args.out_dir)?;

    Ok(())
}
