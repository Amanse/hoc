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
    #[clap(short='k', long, value_parser)]
    auth_header_key: Option<String>,
    #[clap(short='v', long, value_parser)]
    auth_header_val: Option<String>,
}

fn main() {
    let cli = Args::parse();
    let mut auth_tuple: (String, String) = ("".to_string(), "".to_string());

    match cli.auth_header_key {
        Some(k) => {
            match cli.auth_header_val {
                Some(v) => {auth_tuple = (k, v)},
                None => panic!("Need auth header value if key is given")
            }
        },
        None => {}
    }
    

    let method: Methods = {
        match cli.method {
            Some(v) => v,
            None => Methods::Get,
        }
    };

    let resp = make_request(cli.url, method, cli.json_header, cli.raw_json, auth_tuple);
    println!("{}", resp);

}
