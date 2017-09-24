// Copyright 2017 Michael Dilger
// See COPYRIGHT file for details

extern crate url;
extern crate hyper;

use hyper::server::Request;

pub fn get_query_param<'a>(param: &'a str, request: &Request) -> Option<String>
{
    if let Some(q) = request.query() {
        if let Some(v) = ::url::form_urlencoded::parse(q.as_bytes())
            .find(|&(ref n,_)| n==param)
        {
            return Some(v.1.into_owned())
        }
    }
    None
}
