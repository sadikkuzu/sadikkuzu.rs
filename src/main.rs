use clap::Parser;

/// SADIK KUZU
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Print GitHub profile link
    #[clap(long, action = clap::ArgAction::Count)]
    github: u8,

    /// Print Linkedin profile link
    #[clap(long, action = clap::ArgAction::Count)]
    linkedin: u8,

    /// Print twitter link
    #[clap(long, action = clap::ArgAction::Count)]
    twitter: u8,

    /// Print all links
    #[clap(long, action = clap::ArgAction::Count)]
    all: u8,
}

impl Args {
    fn argcount(&self) -> u8 {
        // TODO find a better way to do this
        self.github + self.linkedin + self.twitter + self.all
    }
}

fn github() {
    println!("https://github.com/sadikkuzu")
}

fn linkedin() {
    println!("https://linkedin.com/in/sadikkuzu")
}

fn twitter() {
    println!("https://twitter.com/sadikkuzu_mba")
}

fn all() {
    github();
    linkedin();
    twitter();
}

fn main() {
    let args = Args::parse();

    if args.argcount() == 0 {
        println!("Aloha, World! This is SADIK KUZU.");
    } else if args.all > 0 {
        all()
    } else {
        if args.github > 0 {
            github()
        }

        if args.linkedin > 0 {
            linkedin()
        }

        if args.twitter > 0 {
            twitter()
        }
    }
}
