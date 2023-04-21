mod file_system;
use crate::file_system::FileSystem;
use crate::file_system::dir::file::{CustomError, File, FileType};

fn main() -> Result<(), CustomError>{
    //1) create an empty fs
    let _my_empty_fs = FileSystem::new();

    //2) create a fs starting from a specified directory
    let mut my_fs = FileSystem::from_dir("my_fs")?;
    println!("File System CREATED from Directory 'my_fs':\n{}", my_fs.get_root());

    //3) make a dir in the specified path
    my_fs.mk_dir("my_fs\\folder0_0\\folder_new")?;
    println!("File System Directory ADDED 'my_fs\\folder0_0\\folder_new':\n{}", my_fs.get_root());

    //3) delete the dir specified by the path
    my_fs.rm_dir("my_fs\\folder0_0\\folder_new")?;
    println!("File System Directory DELETED 'my_fs\\folder0_0\\folder_new':\n{}", my_fs.get_root());

    //4) create a file from a specified one
    let mut test_file = File::default();
    test_file.set_name("test_file.txt".to_string());
    test_file.set_content(vec![22; 1000]);
    test_file.set_creation_time(123456789);
    test_file.set_type_(FileType::Text);
    my_fs.mk_dir("my_fs\\folder0_0\\test_folder")?;
    my_fs.new_file("my_fs\\folder0_0\\test_folder\\test_file.txt", test_file)?;
    println!("File System File CREATED 'my_fs\\folder0_0\\test_folder\\test_file.txt':\n{}", my_fs.get_root());

    //6) get the file specified by the path
    let file_found = my_fs.get_file("my_fs\\folder0_0\\test_folder\\test_file.txt");
    match file_found {
        Some(file) => println!("File System File GOT:\n{}", file),
        None => println!("File not Found!\n")
    }

    //5) remove the file specified by the path
    my_fs.rm_file("my_fs\\folder0_0\\test_folder\\test_file.txt")?;
    println!("File System File DELETED 'my_fs\\folder0_0\\test_folder\\test_file.txt':\n{}", my_fs.get_root());

    //7) query the fs (eventually with different queries)
    let queries = vec!["name:file0_0.txt", "content:test queries"];
    let res = my_fs.search(&queries);
    println!("{}", res);

    Ok(())
}
