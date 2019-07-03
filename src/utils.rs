use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn directory_check(dir: &std::path::PathBuf) -> bool {

    if fs::read_dir(dir).unwrap().count() == 0 {
        eprintln!("[ERROR] DirectoryIsEmptyError at : {}", dir.display());
        false
    } else {
        true
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
        // println!("{} {}", "[GIT OUTPUT]".green(), init_stdout)
    }

    let _add_remote = Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg("git@github.com:DumbMachine/donjo-example.git")
        .output()
        .expect("some TING WONG");

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

    println!("[GIT OUTPUT]:");
    git_printer(commit_stdout);
}

pub fn push_origin(force: bool) {
    let push = if force {
        Command::new("git")
            .arg("push")
            .arg("origin")
            .arg("master")
            .arg("-f")
            .output()
            .expect("some TING WONG")
    } else {
        Command::new("git")
            .arg("push")
            .arg("origin")
            .arg("master")
            .output()
            .expect("some TING WONG")
    };

    // let push_stdout = String::from_utf8(push.stdout).unwrap();
    let push_sterr = String::from_utf8(push.stderr).unwrap();

    if push_sterr.contains("failed") {
        eprintln!("[ERROR] UnableToPush. Try adding --force to overcome this. ");
    } else {
        println!("Succesfullly Pushed");
    }
}

pub fn sync(location: &std::path::PathBuf, force: bool) {
    env::set_current_dir(&location).unwrap();
    commit();
    push_origin(force);
}

pub fn generate(location: &std::path::PathBuf) {
    let data: String =
        fs::read_to_string(location.join("template.md")).expect("Unable to read file");

    fs::write(location.join("from-template.md"), data).expect("Unable to write file");
}

pub fn git_printer(string: std::string::String) {
    for line in string.lines() {
        println!("              {}", line);
    }
}

pub fn get_files(location: std::path::PathBuf) {
    let mut files: Vec<String> = [].to_vec();
    for item in fs::read_dir(location).unwrap() {
        let item = item.unwrap();
        if item.file_name().into_string().unwrap().contains(".md") {
            // println!("!");
            files.push(item.file_name().into_string().unwrap());
        }
        // }
    }
    // for item in fs::read_dir(location).unwrap() {
    //     let item = item.unwrap();
    //     if item.path().is_file() {
    //         println!("{:#?}", item);
    //     }
    // }
    println!("{:#?}", files);

}
fn main() {
    let base = Path::new("/home/dumbmachine/Documents/Typora").to_path_buf();
    get_files(base);
}
