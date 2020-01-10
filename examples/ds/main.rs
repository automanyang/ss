extern crate bytes;
extern crate futures;
extern crate futures_codec;
extern crate async_std;

// --

use futures::{executor};

fn main() {
    executor::block_on(async move {
        use async_std::os::unix::net::UnixStream;
        use async_std::prelude::*;

        let mut stream = UnixStream::connect("/tmp/socket").await.unwrap();
        // let r = stream.write_all(b"hello world").await;
        // dbg!("{:?}", r);

        let mut response = Vec::new();
        let r = stream.read_to_end(&mut response).await;
        dbg!("{:?}", r.unwrap());
        dbg!("buf: {:?}", response);

        Ok::<(), std::io::Error>(())
    }).unwrap()

}
