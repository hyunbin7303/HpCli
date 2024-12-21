#[path = "../src/file_helper.rs"] mod file_helper;

#[cfg(test)]
mod tests {
    use std::{fs::{self, File}, io::{self, Write}};

    use tempfile::TempDir;

    use crate::file_helper;

    #[test]
    fn test_get_files_in_empty_folder() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_str().unwrap();

        let files = file_helper::get_files_in_folder(path)?;
        assert!(files.is_empty());

        Ok(())
    }
    #[test]
    fn test_get_files_in_folder_with_files() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_str().unwrap();

        // Create some test files
        File::create(temp_dir.path().join("file1.txt"))?.write_all(b"test content")?;
        File::create(temp_dir.path().join("file2.txt"))?.write_all(b"test content")?;

        let files = file_helper::get_files_in_folder(path)?;
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|f| f.file_name().unwrap() == "file1.txt"));
        assert!(files.iter().any(|f| f.file_name().unwrap() == "file2.txt"));

        Ok(())
    }

    #[test]
    fn test_get_files_in_folder_with_subdirectories() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        let path = temp_dir.path().to_str().unwrap();

        // Create a file and a subdirectory
        File::create(temp_dir.path().join("file1.txt"))?.write_all(b"test content")?;
        fs::create_dir(temp_dir.path().join("subdir"))?;

        let files = file_helper::get_files_in_folder(path)?;
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|f| f.file_name().unwrap() == "file1.txt"));
        assert!(files.iter().any(|f| f.file_name().unwrap() == "subdir"));

        Ok(())
    }

    #[test]
    fn test_get_files_in_nonexistent_folder_return_error() {
        let result = file_helper::get_files_in_folder("/path/that/does/not/exist");

        assert!(result.is_err());
    }

}