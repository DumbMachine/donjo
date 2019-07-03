use std::fs;
// use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
extern crate dirs;

// fn main() {
//     let x:bool = directory_fine(Path::new(dirs::document_dir()).join("Typora"));
//     println!("{:#?}",x);
//         let paths = fs::read_dir("/home/dumbmachine/Documents/Typora/").unwrap();

//     for path in paths {
//         println!("Name: {}", path.unwrap().path().display())
//     }
// }

pub fn directory_check(dir: std::path::PathBuf) -> bool {
    if !dir.is_dir() {
        eprintln!("[ERROR] DirectoryNotFoundError: {}", dir.display());
        false
    } else {
        if fs::read_dir(&dir).unwrap().count() == 0 {
            eprintln!("[ERROR] DirectoryIsEmptyError: {}", dir.display());
            false
        } else {
            for x in fs::read_dir(&dir).unwrap() {
                println!("{:#?}", x.unwrap().file_name());
            }
            true
        }
    }
}

pub fn init(force: bool) {

    // Checking if the file exists

    // Deleting the file
    if force {
        match fs::remove_file(".git") {
            Ok(_) => println!("Deleted the .git folder, reini"),
            Err(_) => println!("Skipping the deletion as the file, doesn;t exist"),
        };
    }

    let init = Command::new("git")
        .arg("init")
        .output()
        .expect("some TING WONG");

    if init.stderr.len() == 0 {
        println!("{:#?}", init);

    } else {
        eprint!("some TING WONG");
    }

    // let add_remote = Command::new("git")
    //     .arg("remote")
    //     .arg("set-url")
    //     .arg("origin")
    //     .arg("git@github.com:DumbMachine/donjo.git")
    //     .output()
    //     .expect("some TING WONG");
    let add_remote = Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg("git@github.com:DumbMachine/donjo.git")
        .output()
        .expect("some TING WONG");
    if init.stderr.len() == 0 {
        println!("{:#?}", add_remote)

    } else {
        eprint!("some TING WONG")
    }

}

pub fn commit() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let current_time: String = (since_the_epoch.as_secs() * 1000
        + since_the_epoch.subsec_nanos() as u64 / 1_000_000)
        .to_string();

    let add_id = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("some TING WONG");
    if add_id.stderr.len() == 0 {
        // No error occured.
        println!("Adding the Files")
    }


    let commit_id = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(current_time)
        .output()
        .expect("some TING WONG");

    let commit_stdout = String::from_utf8(commit_id.stdout).unwrap();

    let mut files_added = String::new();

    // for line in commit_stdout.lines(){
    //     if line.contains("files changed"){
    //         println!("Total Changes: {:#?}", line);
    //     }
    //     if line.contains(".md"){
    //         files_added.push_str(line)
    //     }
    // }

    // println!("{:#?}", commit_stdout.lines());
    // println!("Total Changes: {:#?}", files_added);
    println!("{}", commit_stdout);
}

pub fn push_origin() {
    let push = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .arg("-f")
        .output()
        .expect("some TING WONG");

    let push_stdout = String::from_utf8(push.stdout).unwrap();
    let push_sterr = String::from_utf8(push.stderr).unwrap();
    print!("[SUCESS] {}", push_stdout);
    print!("Error Was Obtrained {}", push_sterr);
}

pub fn generate() {

    let data: String = fs::read_to_string("./src/template.md").expect("Unable to read file");
    // println!("{}", data.len());

    fs::write("./hmm.md", data).expect("Unable to write file");
}

