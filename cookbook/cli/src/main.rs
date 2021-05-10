fn main() {
    clap_arg::parse_arg();
    term_theme::it_works();
}

mod clap_arg {
    use clap::{App, Arg};
    pub fn parse_arg() {
        let matches = App::new("My Test Program")
            .version("0.1.0")
            .author("Chaos")
            .about("Teaching argument parsing")
            .arg(
                Arg::with_name("file")
                    .short("f")
                    .long("file")
                    .takes_value(true)
                    .help("a cool file"),
            )
            .arg(
                Arg::with_name("num")
                    .short("n")
                    .long("number")
                    .takes_value(true)
                    .help("Five less than your favorite number"),
            )
            .get_matches();

        let myfile = matches.value_of("file").unwrap_or("input.txt");
        println!("The file passed is :{}", myfile);
        let num_str = matches.value_of("num");
        match num_str {
            None => println!("No idea what your favorite number is"),
            Some(s) => match s.parse::<i32>() {
                Ok(n) => println!("Your favorite number must be {}", n),
                Err(_) => println!("That's not a number! {}", s),
            },
        }
    }
}

mod term_theme {
    use ansi_term::{Colour, Style};
    pub fn it_works() {
        println!(
            "This is {} in color, {} in color and {} in color",
            Colour::Red.bold().paint("red"),
            Colour::Blue.dimmed().paint("blue"),
            Colour::Purple.paint("purple")
        );

        println!("{} and this is not", Style::new().bold().paint("This is blod"));
    }
}