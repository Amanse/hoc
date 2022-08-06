mod request;

use clap::Parser;
use request::Methods;

use crate::request::make_request;

#[derive(Parser)]
#[clap(version,about,long_about="High opinionated curl")]
struct  Args {
    #[clap(short,long,value_parser)]
    url: String,
    #[clap(arg_enum ,value_parser)]
    method: Option<Methods>,
    #[clap(short, long, action)]
    raw_json: bool,
    #[clap(short, long, action=clap::ArgAction::SetTrue)]
    json_header: bool,
}

fn main() {
    let cli = Args::parse();

    let method: Methods = {
        match cli.method {
            Some(v) => v,
            None => Methods::Get,
        }
    };

    let resp = make_request(cli.url, method, cli.json_header, cli.raw_json);
    println!("{}", resp);

}
