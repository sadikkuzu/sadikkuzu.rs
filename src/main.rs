use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, args_override_self = true)]
struct Args {
    /// Print GitHub profile link
    #[arg(long)]
    github: bool,

    /// Print Linkedin profile link
    #[arg(long)]
    linkedin: bool,

    /// Print twitter link
    #[arg(long)]
    twitter: bool,

    /// Print all links
    #[arg(long)]
    all: bool,
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

fn main() {
    let args = Args::parse();

    if !(args.github || args.linkedin || args.twitter || args.all) {
        println!("Aloha, World! This is SADIK KUZU.");
        return;
    }

    if args.all || args.github {
        github()
    }

    if args.all || args.linkedin {
        linkedin()
    }

    if args.all || args.twitter {
        twitter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Args::command().debug_assert();
    }
}
