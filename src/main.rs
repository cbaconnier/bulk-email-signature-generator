mod csv_reader;
mod html_generator;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn prog() -> Option<String> {
    std::env::args_os().next().and_then(|p| p.into_string().ok())
}

fn help() {
    println!("Usage: {} [OPTION]... [FILE]...", prog().unwrap_or_default());
    println!("Generate a contact list from a CSV file.");
    println!();
    println!("  -f, --csv FILE");
    println!("  -t, --template FILE");
    println!("  -h, --help");
    println!("      --version");
    println!("  -q, --quiet");
    println!("  -v, --verbose");
    println!();
}

fn main() {
    let mut csv_path: String = String::from("contacts.csv");
    let mut template_path: String = String::from("template.html");
    let mut output_path: String = String::from("output");

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => {
                help();
                std::process::exit(0);
            },
            "--version" => {
                println!("{} {}", prog().unwrap_or_default(), VERSION);
                std::process::exit(0);
            }
            "-q" | "--quiet" => {
                println!("Quiet mode is not supported yet.");
                std::process::exit(0);
            }
            "-v" | "--verbose" => {
                println!("Verbose mode is not supported yet.");
                std::process::exit(0);
            }
            "-f" | "--csv" => {
                if let Some(arg_csv) = args.next() {
                    csv_path = arg_csv;
                } else {
                    panic!("No value specified for parameter --csv.");
                }
            }
            "-t" | "--template" => {
                if let Some(arg_template) = args.next() {
                    template_path = arg_template;
                } else {
                    panic!("No value specified for parameter --template.");
                }
            }
            "-o" | "--output" => {
                if let Some(arg_output) = args.next() {
                    output_path = arg_output;
                } else {
                    panic!("No value specified for parameter --output.");
                }
            }
            _ => {
                if arg.starts_with('-') {
                    println!("Unkown argument {}", arg);
                } else {
                    println!("Unkown positional argument {}", arg);
                }
            }
        }
    }

    let csv = csv_reader::CsvReader::new(csv_path);
    let html = html_generator::HtmlGenerator::new(
        template_path,
        csv.records(),
        output_path
    );

    html.generate();
}
