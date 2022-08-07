 // use std::fmt::Display;

use std::collections::HashMap;

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
    post_data: Option<HashMap<String, String>>,
    req: ureq::Request
}

impl CRequest {
   
    fn new(
        url: String,
        method: Methods,
        if_json: bool,
        raw_js: bool,
        auth_tuple: (String, String),
        post_data: Option<HashMap<String, String>>
        ) -> CRequest {
        let req: ureq::Request;
        req = match method {
            Methods::Get => ureq::get(&url),
            Methods::Post => ureq::post(&url),
            Methods::Delete => ureq::delete(&url),
            Methods::Put => ureq::put(&url),
        };
        CRequest {
            if_json:if_json,
            raw_js: raw_js,
            auth_tuple: auth_tuple,
            req: req,
            post_data: post_data, 
        }
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
        let res = match &self.post_data {
            Some(v) => {
               self.req.clone().send_json(v).unwrap()
                },
            None => {
                self.req.clone().call().unwrap()
            }
            };
        if !self.raw_js {
            jsonxf::pretty_print(&res.into_string().unwrap()).unwrap()
        } else {
            res.into_string().unwrap()
        }
    } 

    }

pub fn make_request(
    url: String,
    method: Methods,
    if_json: bool,
    raw_js: bool,
    auth_tuple: (String, String),
    post_data: Option<HashMap<String,String>>
    ) -> String{
    CRequest::new(
        url,
        method,
        if_json,
        raw_js,
        auth_tuple,
        post_data,
        ).handle_json_head().handle_auth_tuple().make_req()
}

