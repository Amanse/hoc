use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum Methods {
    Get,
    Post,
    Delete,
    Put,
}

impl Display for Methods {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Methods::Get => write!(f, "get"),
            Methods::Post => write!(f, "post"),
            Methods::Put => write!(f, "put"),
            Methods::Delete => write!(f, "delete"),
        }
    }
}

pub fn make_request(url: String, method: Methods, if_json: bool, pretty_p: bool) -> String{
 match method {
     Methods::Get => make_get(url, if_json, pretty_p),
     Methods::Post => make_post(url, if_json),
     Methods::Delete => make_delete(url, if_json),
     Methods::Put => make_put(url, if_json),
 }  

}

fn make_get(url: String, if_json: bool, pretty_p: bool) -> String {
    let req = ureq::get(&url).call().unwrap();

    let st = req.into_string().unwrap();
    if pretty_p {
    jsonxf::pretty_print(&st).unwrap()
    } else {
        st
    }
}
     
fn make_post(url: String, if_json: bool) -> String {
    todo!()

}
fn make_delete(url: String, if_json: bool) -> String {
    todo!()
}

fn make_put(url: String, if_json: bool) -> String {
    todo!()
}
