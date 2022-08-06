 // use std::fmt::Display;

use clap::clap_derive::ArgEnum;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Methods {
    Get,
    Post,
    Delete,
    Put,
}

struct CRequest {
    if_json:bool,
    raw_js:bool,
    auth_tuple: (String, String),
    req: ureq::Request
}

impl CRequest {
   
    fn new(url: String, method: Methods, if_json: bool, raw_js: bool, auth_tuple: (String, String)) -> CRequest {
        let req: ureq::Request;
        req = match method {
            Methods::Get => ureq::get(&url),
            Methods::Post => ureq::post(&url),
            Methods::Delete => ureq::delete(&url),
            Methods::Put => ureq::put(&url),
        };
        CRequest {if_json:if_json, raw_js: raw_js, auth_tuple: auth_tuple, req: req }
    }
    
    fn handle_auth_tuple(&mut self) -> &mut CRequest {

        let (key, value) = &self.auth_tuple;
        if key == "" && value == "" {
            self
        } else {
            if key != "" && value == "" {
                panic!("Auth header value is needed with key")
            } else if key == "" && value != "" {
                self.req = self.req.clone().set("Authorization", value.as_str());
                self
            } else {
                self.req = self.req.clone().set(key.as_str(), value.as_str());
                self
            }
        }

    }

    fn  handle_json_head(&mut self ) -> &mut CRequest {
        if self.if_json {
            self.req = self.req.clone().set("Content-Type", "application/json");
            self
        } else {
            self
        }
    }

    fn make_req(&self) -> String {
        let res = self.req.clone().call().unwrap();
        if !self.raw_js {
            jsonxf::pretty_print(&res.into_string().unwrap()).unwrap()
        } else {
            res.into_string().unwrap()
        }
    } 

}

pub fn make_request(url: String, method: Methods, if_json: bool, raw_js: bool, auth_tuple: (String, String)) -> String{
    CRequest::new(url, method, if_json, raw_js, auth_tuple).handle_json_head().handle_auth_tuple().make_req()
}

// fn make_get(url: String, if_json: bool, raw_js: bool) -> String {
//     let req = ureq::get(&url);
//     let st = req.call().unwrap().into_string().unwrap();
//     if !raw_js {
//         jsonxf::pretty_print(&st).unwrap()
//     } else {
//         st.to_string()
//     }
// }
//     
// fn make_post(url: String, if_json: bool) -> String {
//     todo!()
//
// }
// fn make_delete(url: String, if_json: bool, raw_js: bool) -> String {
//     let res;
//     if if_json {
//         let req = ureq::delete(&url).set("Content-Type", "application/json");
//         res = req.call().unwrap();
//     } else {
//         let req = ureq::get(&url);
//         res = req.call().unwrap();
//     }
//     let st = res.into_string().unwrap();
//     if !raw_js {
//         jsonxf::pretty_print(&st).unwrap()
//     } else {
//         st
//     }
// }
//
// fn make_put(url: String, if_json: bool) -> String {
//     todo!()
// }
//

// impl Display for Methods {
//      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match &self {
//             Methods::Get => write!(f, "get"),
//             Methods::Post => write!(f, "post"),
//             Methods::Put => write!(f, "put"),
//             Methods::Delete => write!(f, "delete"),
//         }
//     }
// }
