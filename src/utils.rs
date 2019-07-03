use std::process::Command;
// use std::
use std::env;
use std::fs;
use std::path::Path;
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

pub fn directory_check(dir: &std::path::PathBuf) -> bool {

    if fs::read_dir(dir).unwrap().count() == 0 {
        eprintln!("[ERROR] DirectoryIsEmptyError at : {}", dir.display());
        false
    } else {
        true
        // for x in fs::read_dir(&dir).unwrap() {
        //     println!("{:#?}", x.unwrap().file_name());
        // }
    }
}

pub fn init(location: &std::path::PathBuf, force: bool) {

    // Checking if the file exists

    env::set_current_dir(&location).unwrap();

    if force {
        match fs::remove_dir_all(Path::new(location).join(".git")) {
            Ok(_) => println!("     Deleted the .git folder"),
            Err(_) => println!("    Unable to delete the folder."),
        };
    }

    let init = Command::new("git")
        .arg("init")
        // .arg(location.to_str().unwrap())
        .output()
        .expect("some TING WONG");

    let init_stdout = String::from_utf8(init.stdout).unwrap();
    if init_stdout.contains("Reinitialized") {
        eprintln!(
            "       [ERROR] git repo exists at : {}    Add -f/--force argument to reinitialize",
            location.to_str().unwrap()
        );
        return;
    } else {
        println!("[GIT OUTPUT] {}", init_stdout)
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

    println!("CD: {:#?}", add_remote);

    // println!("{}", String::from_utf8(add_remote.stdout).unwrap());

    // let cd = Command::new("cd")
    //     .arg("/home/dumbmachine/Documents/Typora")
    //     .output()
    //     .expect("Asdasd");

    // println!("CD: {:#?}", cd);

    // let _ls = Command::new("ls").output().expect("Asdasdasdasdasdasd");


    // println!("LS: {:#?}", _ls);


    // println!("REMOTE {:#?}", add_remote);


    // if init.stderr.len() == 0 {
    //     println!("{:#?}", add_remote)

    // } else {
    //     eprint!("some TING WONG")
    // }

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
