use clap::{App, Arg};

mod flags;

fn list() {
    println!("\n Flag Value     Flag Description");
    println!(" -------------------------------------------------");
    println!(" lgbtq          Show the six colour lgbtq flag.");
    println!("                Show the six colour lgbtq flag.");
    println!(" agender        Show the agender flag.");
    println!(" aromantic      Show the aromantic flag.");
    println!(" asexual        Show the asexual flag.");
    println!(" bisexual       Show the bisexual flag.");
    println!(" demisexual     Show the demisexual flag.");
    println!(" genderfluid    Show the genderfluid flag.");
    println!(" genderqueer    Show the genderqueer flag.");
    println!(" intersex       Show the intersex flag.");
    println!(" lesbian        Show the traditional lesbian flag.");
    println!(" lesbian-comm   Show the lesbian community flag.");
    println!(" non-binary     Show the non-binary flag.");
    println!(" pansexual      Show the panexual flag.");
    println!(" polysexual     Show the polysexual flag.");
    println!(" transgender    Show the transgender flag.\n");
}
fn main() {
    let cmd_args = App::new("Terminal Flags")
                .version("1.0.1")
                .author("Matthew Limb <matt.limb17@gmail.com>")
                .about("Print coloured flags in the terminal.")
                .arg(Arg::with_name("FLAG")
                    .value_name("FLAG")
                    .help("Name of the flag to show.")
                    .takes_value(true)
                )
            .get_matches();
            
    match cmd_args.value_of("FLAG").unwrap_or("lgbtq") {
        "lgbtq" => println!("{}", flags::lgbtq()),
        "agender" => println!("{}", flags::agender()),
        "aromantic" => println!("{}", flags::aromantic()),
        "asexual" => println!("{}", flags::asexual()),
        "bisexual" => println!("{}", flags::bi()),
        "demisexual" => println!("{}", flags::demi()),
        "genderfluid" => println!("{}", flags::genderfluid()),
        "genderqueer" => println!("{}", flags::genderqueer()),
        "intersex" => println!("{}", flags::intersex()),
        "lesbian" => println!("{}", flags::lesbian()),
        "lesbian-comm" => println!("{}", flags::lesbian_comm()),
        "non-binary" => println!("{}", flags::non_binary()),
        "pansexual" => println!("{}", flags::pan()),
        "polysexual" => println!("{}", flags::polysexual()),
        "transgender" => println!("{}", flags::trans()),
        "list" => list(),
        other => println!("No flag by the name: {}. Please choose a valid flag.", other),
    }
}