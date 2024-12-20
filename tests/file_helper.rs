#[path = "../src/file_helper.rs"] mod file_helper;

#[cfg(test)]
mod tests {
    use std::io;

    use tempfile::TempDir;

    use crate::file_helper;

    // #[test]
    // fn test_filehandler_get_files_in_folder_current_path_success() {
    //     let result: Vec<std::path::PathBuf> = file_helper::get_files_in_folder("./").unwrap();

    // }

    #[test]
    fn test_get_files_in_empty_folder() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_str().unwrap();

        let files = file_helper::get_files_in_folder(path)?;
        assert!(files.is_empty());

        Ok(())
    }

    #[test]
    fn test_get_files_in_nonexistent_folder_return_error() {
        let result = file_helper::get_files_in_folder("/path/that/does/not/exist");

        assert!(result.is_err());
    }

}