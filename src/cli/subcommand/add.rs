use crate::cli::OPTIONS;
use crate::os::{get_file_name, move_file, symlink};
use std::error::Error;
use std::ffi::OsString;
use std::path::PathBuf;
use structopt::StructOpt;

/// Move a file to dotfiles directory, replacing it with a symlink
#[derive(Debug, StructOpt)]
pub struct Add {
    /// A file to move to your dotfiles
    #[structopt(required = true)]
    file: PathBuf,

    /// Store the file with a given name in the dotfiles directory
    #[structopt(long = "with_name", short = "o")]
    with_name: Option<OsString>,
}

impl Add {
    pub fn perform(&self) -> Result<(), Box<dyn Error>> {
        let dest = OPTIONS
            .dotfiles_dir()
            .with_file_name(match &self.with_name {
                Some(name) => name,
                None => get_file_name(&self.file)?,
            });

        if dest.exists() {
            print_warn!("File `{}` already exists", dest.display());
            if !confirm!("Replace it?"; true) {
                return Ok(());
            }
            print_info!("Replacing `{}` with a newly added file", dest.display());
        } else {
            print_info!("Moving `{}` to your dotfiles", dest.display());
        }
        move_file(&self.file, &dest)?;

        print_info!(
            "Creating symlink `{}` -> `{}`",
            self.file.display(),
            dest.display()
        );
        symlink(dest.canonicalize()?, &self.file)?;
        Ok(())
    }
}