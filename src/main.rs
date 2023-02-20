use clap::Parser;
use crate::lister::RunID;

mod lister;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Either CHAPTER.SECTION or NAME of exercise to run
    #[arg()]
    arg: String,
}


fn main() {
    let arg = Args::parse().arg;
    let runner_list = lister::Lister::new();

    let id = if arg.contains('.') {
        let args: Vec<&str> = arg.split('.').collect();
        let chapter = args[0].parse::<i32>().expect("Invalid argument including string but not convertable to ints");
        let section = args[1].parse::<i32>().expect("Invalid argument including string but not convertable to ints");

        RunID::Numeric { chapter, section }
    } else {
        RunID::Named(arg)
    };

    let run_result = runner_list.launch(id);

    match run_result {
        Ok(_) => { println!("========= DONE ==========") }
        Err(_) => {
            println!("ID not found");
            println!("usage: <arg>");
            println!("arg: CHAPTER.SECTION or NAME");
            println!();
            println!("Available Options:");
            println!("{}", runner_list);
        }
    }
}
