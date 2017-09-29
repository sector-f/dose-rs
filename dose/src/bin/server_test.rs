extern crate tokio_core;
use tokio_core::reactor::Core;

extern crate tokio_io;
use tokio_io::{io, AsyncRead};

extern crate futures;
use futures::{Future, Stream};

extern crate tokio_uds;
use tokio_uds::UnixListener;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let socket = UnixListener::bind("./socket", &handle).unwrap();

    let server = socket.incoming().for_each(|(stream, _addr)| {
        let (reader, writer) = stream.split();
        let copy = io::copy(reader, writer).then(|_| Ok(()));
        handle.spawn(copy);
        Ok(())
    });

    let _ = core.run(server);
}
