#![allow(dead_code)]
#![allow(unused_imports)]
use std::num::FpCategory;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Write};
use std::error::Error;

struct FileOrganizer{
    base_directory: PathBuf,
}

impl FileOrganizer {
    fn new(base_path: &str) -> io::Result<Self>{
        let path = PathBuf::from(base_path);

        if !path.exists(){
            fs::create_dir_all(&path)?;
        }
        Ok(FileOrganizer { base_directory: (path)})

    }

    fn create_category(&self, category: &str)-> io::Result<PathBuf>{
        let category_path = self.base_directory.join(category);
        fs::create_dir_all(&category_path)?;
        Ok(category_path)
    }

    fn organize_file(&self, file_path: &Path, category: &str) -> io::Result<PathBuf>{
        let filename = file_path.file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid fillename"))?;

        let category_path = self.create_category(category)?;

        let new_file_path = category_path.join(filename);

        fs::copy(file_path, &new_file_path)?;

        println!("Moved {} to {}", file_path.display(), new_file_path.display());

        Ok(new_file_path)

    }

    fn create_log(&self) -> io::Result<()> {
        let log_path = self.base_directory.join("organiztion_log.txt");
        let mut log_file = File::create(&log_path)?;

        writeln!(log_file, "File Organization Log")?;
        writeln!(log_file, "Base Directory: {}",self.base_directory.display())?;
        Ok(())
    }


}
fn main()-> Result<(), Box<dyn Error>> { 
    let organizer = FileOrganizer::new("./organized_files")?;

    let documents = Path::new("./sample_document.txt");
    let image = Path::new("./sample_image.jpg");

    organizer.organize_file(documents, "Documents")?;
    organizer.organize_file(image, "Images")?;

    organizer.create_log()?;

    println!("File organization complete!");
    Ok(())
}
