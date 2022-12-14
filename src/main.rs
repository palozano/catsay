use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::{self, Read};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String, // [1]

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the animal picture from the specified file
    catfile: Option<std::path::PathBuf>,

    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let mut message = String::new(); // options.message;

    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    if message.to_lowercase() == "woof" {
        eprint!("A cat shouldn't bark like a dog.")
    }

    let eye = if options.dead { "x" } else { "o" };

    match &options.catfile {
        Some(path) => {
            let animal_template = std::fs::read_to_string(path)
                .with_context(|_| format!("could not read file {:?}", path))?;
            let animal_picture = animal_template.replace("{eye}", eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", &animal_picture);
        }
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!(" \\");
            println!("   /\\_/\\");
            println!("  ( {eye} {eye} )", eye = eye.red().bold());
            println!(" =(  I  )=");
        }
    }
    Ok(())
}
