use clap::Parser;

// Each flag overrides itself so repeating it (`--github --github`) is
// accepted instead of a "cannot be used multiple times" error.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Print GitHub profile link
    #[arg(long, overrides_with = "github")]
    github: bool,

    /// Print Linkedin profile link
    #[arg(long, overrides_with = "linkedin")]
    linkedin: bool,

    /// Print X (Twitter) profile link
    #[arg(long, alias = "twitter", overrides_with = "x")]
    x: bool,

    /// Print all links
    #[arg(long, overrides_with = "all")]
    all: bool,
}

fn github() {
    println!("https://github.com/sadikkuzu")
}

fn linkedin() {
    println!("https://linkedin.com/in/sadikkuzu")
}

fn x() {
    println!("https://x.com/sadikkuzu_mba")
}

fn main() {
    let args = Args::parse();

    if !(args.github || args.linkedin || args.x || args.all) {
        println!("Aloha, World! This is SADIK KUZU.");
        return;
    }

    if args.all || args.github {
        github()
    }

    if args.all || args.linkedin {
        linkedin()
    }

    if args.all || args.x {
        x()
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
