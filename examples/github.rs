use flutter_blog_backend::core::{git_book_manager::manager::github_manager::BOOKS_MANAGER, SPath};
use tempfile::tempdir;

fn main() {
    // config path is set under the workspace
    // let book_configs =
    //     BookConfigs::config_read_from_yaml_path("test-config/books-config.yml").unwrap();

    // set the path later
    let temp_dir = tempdir().unwrap();
    let dir = temp_dir.path().to_str().unwrap();
    let dir = SPath::new(dir);

    let git_book_manager = &mut BOOKS_MANAGER.lock().unwrap();
    git_book_manager.set_storage_path(dir.clone());
    git_book_manager.hook_repository(dir).unwrap();

    // execute powershell command to show tempdir directories and files
    // command_ls(dir.to_str().unwrap());

    // println!("{:#?}", git_book_manager);
    git_book_manager.iter_run().unwrap();
}
