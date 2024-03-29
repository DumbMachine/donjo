mod utils;
use dirs;
use std::path::Path;
use structopt::clap::Shell;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(
    name = "donjo",
    about = "A cli for fancy syncing of your content on Github."
)]
/// The help message that will be displayed when passing `--help`.
struct Opt {
    /// Custom location
    #[structopt(short = "c", long = "custom")]
    custom: bool,

    /// Path of the location
    #[structopt(
        short = "p",
        long = "path",
        parse(from_os_str),
        // required_ifs = ("force")
    )]
    path: std::path::PathBuf,

    /// Should -f argument be used with git.
    #[structopt(short = "f", long = "force")]
    force: bool,

    /// Initialize the donjo cli in the directory.
    #[structopt(short = "i", long = "init", conflicts_with = "sync")]
    init: bool,

    /// Sync the notes with Github
    #[structopt(short = "s", long = "sync")]
    sync: bool,

    /// Generate a template.
    #[structopt(short = "g", long = "generate")]
    generate: bool,
}

fn main() {

    Opt::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");
    let opt = Opt::from_args();

    let base_dir = if opt.path.to_str().unwrap() == "default" {
        let documents_dir = dirs::document_dir().unwrap();
        let dir = Path::new(&documents_dir).join("Typora").to_path_buf();
        dir

    } else {
        if opt.path.exists() {
            let dir = Path::new(&opt.path).to_path_buf();
            dir

        } else {
            panic!("DirectoryNotFoundError at : {}", opt.path.display())
        }
    };

    println!("\nLocation: {}", base_dir.display());
    if utils::directory_check(&base_dir) {

        if opt.init == true {
            // let path = Path::new(".").to_path_buf();
            let mut files = Vec::new();
            utils::readme(&base_dir, &mut files);
            utils::init(&base_dir, opt.force);

        }
        if opt.sync == true {
            let mut files = Vec::new();
            utils::readme(&base_dir, &mut files);
            utils::sync(&base_dir, opt.force);
        }
        if opt.generate == true {
            utils::generate(&base_dir);
        }
    }

}

