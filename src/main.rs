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

fn rename_ext_at(opt: &RenameExtOpt, path: PathBuf) -> std::io::Result<()>{
    let new_ext = &opt.new_ext
        .clone()
        .unwrap_or(String::new());
    let new_ext = new_ext.as_str();
    for entry in fs::read_dir(path.clone())? { // read directory
        let entry = entry?;
        if entry.path().is_dir() && opt.recursive { // if it is a directory
            rename_ext_at(opt, entry.path())?; // recurse
        } else {
            let file_name = entry.file_name(); // get file name
            let file_name = file_name.to_string_lossy(); // convert file name to Cow<String>
            if file_name.ends_with(&opt.old_ext){ // if the file name ends with extension
                let new_file_name = file_name.replace( // replace it
                    &opt.old_ext,
                    new_ext,
                    );
                let mut old_path = path.clone();
                old_path.push(Path::new(&*file_name));
                let mut new_path = path.clone();
                new_path.push(new_file_name);
                fs::rename(old_path, new_path)?; // rename the file
            }
        }
    }
    Ok(())
}

fn rename_ext(opt: &RenameExtOpt) -> std::io::Result<()>{
    let current_dir = env::current_dir()?;
    rename_ext_at(opt, current_dir)
}

fn main() -> std::io::Result<()> {
    let opt = RenameExtOpt::from_args();
    rename_ext(&opt)
}
