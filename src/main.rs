mod utils;

use dirs;
use std::path::Path;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(
    name = "donjo",
    about = "A cli for fancy syncing of your content on Github."
)]
/// The help message that will be displayed when passing `--help`.
struct Opt {

    /// Path of the location
    #[structopt(short = "p", long = "path")]
    path: String,

    /// Should -f argument be used with git.
    #[structopt(short = "f", long = "force")]
    force: bool,

    /// Initialize the donjo cli in the directory.
    #[structopt(short = "i", long = "init")]
    init: bool,

    /// Sync the notes with Github
    #[structopt(short = "s", long = "sync")]
    sync: bool,

    /// Generate a template.
    #[structopt(short = "g", long = "generate")]
    generate: bool,
}

fn main() {
    // let opt = Opt::from_args();
    // println!("{:#?}", opt);
    // if opt.init == true {
    //     utils::init(opt.force);
    // }
    // if opt.sync == true {
    //     utils::commit();
    //     utils::push_origin();
    // }
    // if opt.generate == true {
    //     utils::generate();
    // }
    let documents_dir = dirs::document_dir().unwrap();
    let base_dir = Path::new(&documents_dir).join("Typora");
    println!("{:#?}", base_dir.display());
    println!("{:#?}", utils::directory_check(base_dir));

}

