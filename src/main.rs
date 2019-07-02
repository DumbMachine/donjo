#[macro_use]
extern crate structopt;
mod temp;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "donjo",
    about = "A cli for fancy syncing of your content on Github."
)]
/// The help message that will be displayed when passing `--help`.
struct Opt {

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
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    if opt.init == true {
        temp::init(opt.force);
    }
    if opt.sync == true {
        temp::commit();
        temp::push_origin();
    }
    if opt.generate == true {
        temp::generate();
    }
}

