extern crate bytes;
extern crate futures;
extern crate futures_codec;
extern crate async_std;

// --

// use bytes::Bytes;
use futures::{executor};
// use futures_codec::{Decoder, Encoder, Framed, FramedRead, FramedWrite};
// use std::io::Error;

fn main() {
    executor::block_on(async move {
        use async_std::os::unix::net::UnixListener;
        use async_std::prelude::*;

        let listener = UnixListener::bind("/tmp/socket").await;
        println!("{:?}", listener);
        let listener = listener.unwrap();
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let mut stream = stream.unwrap();
            stream.write_all(b"hello world").await.unwrap();
        }
    })

}
