use structopt::StructOpt;
use std::path::{
    Path,
    PathBuf,
};
use std::env;
use std::fs;

#[derive(StructOpt, Debug)]
#[structopt(name = "rna", about = "Mass rename all files with one pattern to a different pattern")]
struct RenameAllOpt{
    #[structopt(name = "Pattern")]
    pattern: String,

    #[structopt(name = "Replacement")]
    replacement: Option<String>,

    #[structopt(short, long)]
    recursive: bool,
}

fn rename_all_at(opt: &RenameAllOpt, path: PathBuf) -> std::io::Result<()>{
    let replacement = &opt.replacement
        .clone()
        .unwrap_or(String::new());
    let replacement = replacement.as_str();
    for entry in fs::read_dir(path.clone())? { // read directory
        let entry = entry?;
        if entry.path().is_dir() && opt.recursive { // if it is a directory
            rename_all_at(opt, entry.path())?; // recurse
        } else {
            let file_name = entry.file_name(); // get file name
            let file_name = file_name.to_string_lossy(); // convert file name to Cow<String>
            if file_name.contains(&opt.pattern){ // if the file name has the pattern
                let new_file_name = file_name.replace( // replace it
                    &opt.pattern,
                    replacement,
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

fn rename_all(opt: &RenameAllOpt) -> std::io::Result<()>{
    let current_dir = env::current_dir()?;
    rename_all_at(opt, current_dir)
}

fn main() -> std::io::Result<()> {
    let opt = RenameAllOpt::from_args();
    rename_all(&opt)
}
