use std::{process::{ Command }, io::Write};

fn main() {
    let server = "xxx";
    let mut child = Command::new("ssh").arg(server).spawn().ok().expect("ssh connect failed.");
    let _ = {
        let the_stdid_stream = child.stdin.as_mut().expect("could not get mutable stream.");
        the_stdid_stream.write(b"xxx");
        the_stdid_stream.flush();
    };
    child.wait();
}
