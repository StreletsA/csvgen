mod generator;
mod cli;

fn main() {
    generator::generate_csv(cli::parse_params());
}
