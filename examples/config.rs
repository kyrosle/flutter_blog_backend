use flutter_blog_backend::core::git_book_manager::config::{
    read_book_config::BookConfigs, read_server_config::ServerConfigs, ReadConfig,
};

fn main() {
    println!("--- Root Path and Execute Application Path ---");
    println!("current path: {:#?}", std::env::current_dir());
    println!(
        "current executable application path: {:#?}",
        std::env::current_exe()
    );
    println!("--- Path Type : absolute line ---");
    let books_config_absolute = BookConfigs::config_read_from_yaml_path(
        r#"C:\Development\flutter_blog_backend\test-config\books-config.yml"#,
    )
    .unwrap();
    let servers_config_absolute = ServerConfigs::config_read_from_yaml_path(
        r#"C:\Development\flutter_blog_backend\test-config\server-config.yml"#,
    )
    .unwrap();
    println!("books_configs_absolute: \n{:#?}", books_config_absolute);
    println!("servers_configs_absolute: \n{:#?}", servers_config_absolute);

    println!("--- Path Type : relative line ---");

    let books_config_relative =
        BookConfigs::config_read_from_yaml_path(r#"test-config\books-config.yml"#).unwrap();
    let servers_config_relative =
        ServerConfigs::config_read_from_yaml_path(r#"test-config\server-config.yml"#).unwrap();
    println!("books_configs: \n{:#?}", books_config_relative);
    println!("servers_configs: \n{:#?}", servers_config_relative);
}
