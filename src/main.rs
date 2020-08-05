use clap::load_yaml;
use clap::App;
use dolls::elf::ElfDescriptor;
use dolls::Parse;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let rel_path = matches.value_of("INPUT").unwrap();
    let mut abs_path = std::env::current_dir().unwrap();
    abs_path.push(rel_path);

    let mut file = std::fs::File::open(abs_path).unwrap();
    let descriptor = ElfDescriptor::parse(&mut file).unwrap();

    print!("{:#?}", descriptor);
}
