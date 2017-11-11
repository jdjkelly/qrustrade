extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate serde_json;

use std::env;
use std::io::{self, Write};

use serde_json::{Value, Error};

use futures::Future;
use futures::stream::Stream;

use hyper::{Client, Chunk, Uri};
use hyper_tls::HttpsConnector;

fn get(uri: Uri) -> Client::FutureResponse {
    let mut core = tokio_core::reactor::Core::new()?;
    let handle = core.handle();
    let httpsConnector = HttpsConnector::new(4, &handle)?;

    let client = Client::configure()
        .connector(httpsConnector)
        .build(&handle);

    return client.get(uri);
}

fn exchange(refresh_token: &str) -> String {
    let uri = format!("https://login.questrade.com/oauth2/token?grant_type=refresh_token&refresh_token={refresh_token}", refresh_token = refresh_token).parse::<hyper::Uri>()?;

    let request = get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().concat2().and_then(move |body: Chunk| {
            let v: Value = serde_json::from_slice(&body).unwrap();
            println!("{}", v);
            Ok(())
        })
    });

    core.run(request).unwrap();
}

fn main() {
  let refreshToken = match env::args().nth(1) {
      Some(refreshToken) => refreshToken,
      None => {
          println!("Usage: client <refresh_token>");
          return;
      }
  };

  exchange(refreshToken);
}
