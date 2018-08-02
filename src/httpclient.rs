extern crate hyper;
extern crate hyper_tls;

extern crate serde;
extern crate serde_json;
extern crate futures;

use hyper::{Body, Request};
use hyper::Client;
use hyper_tls::HttpsConnector;
use hyper::rt::{Future, Stream};


pub struct HttpClient {

}

impl HttpClient {
    pub fn make_request<T>(method: &str, url: &str) -> impl Future<Item = T, Error = hyper::Error>
    where 
        T: serde::de::DeserializeOwned, 
    {
        let client = HttpClient::create_client();
        
        let req = HttpClient::build_request(method, url);
    
        client.request(req)
              .and_then(|res| {
                  println!("Got response: {}", res.status());
                  res.into_body().concat2()
               })
               .and_then(move |body| {
                   let s = String::from_utf8(body.to_vec()).unwrap();
                   let ds = serde_json::from_str(&s).unwrap();
                   Ok(ds)
               })
    }

    fn create_client() -> hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body> {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        Client::builder().build::<_, hyper::Body>(https)
    }

    fn build_request(method: &str, url: &str) -> hyper::Request<hyper::Body> {
        Request::builder()
            .method(method)
            .uri(url)
            .header("user-agent", "newsy")
            .body(Body::empty())
            .unwrap()
    }
}