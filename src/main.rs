use structopt::StructOpt;
use std::path::{
    Path,
    PathBuf,
};
use std::env;
use std::fs;

#[derive(StructOpt, Debug)]
#[structopt(name = "rename_ext", about = "Mass rename all files with an extension to another extension")]
struct RenameExtOpt{
    #[structopt(name = "OLD EXTENSION")]
    old_ext: String,

    #[structopt(name = "NEW EXTENSION")]
    new_ext: Option<String>,

    #[structopt(short, long)]
    recursive: bool,
}

fn rename_ext(opt: RenameExtOpt) -> std::io::Result<()>{
    let current_dir = env::current_dir()?;
    let new_ext = &opt.new_ext
        .unwrap_or(String::new());
    let new_ext = new_ext.as_str();
    for entry in fs::read_dir(current_dir.clone())? {
        let file_name = entry?.file_name();
        let file_name = file_name.to_string_lossy();
        if file_name.ends_with(&opt.old_ext){
            let new_file_name = file_name.replace(
                &opt.old_ext,
                new_ext,
                );
            let mut old_path = current_dir.clone();
            old_path.push(Path::new(&*file_name));
            let mut new_path = current_dir.clone();
            new_path.push(new_file_name);
            fs::rename(old_path, new_path)?;
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let opt = RenameExtOpt::from_args();
    let opt = dbg!(opt);
    rename_ext(opt)
}
