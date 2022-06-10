fn main() {
    use std::io::prelude::*;
    use std::net::{TcpStream};
    use ssh2::Session;
    use std::path::Path;

    // Connect to 1st SSH server
    let tcp = TcpStream::connect("143.244.190.188:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("root", "tesTer123s").unwrap();

    // Connect to 2nd SSH server
    let tcp1 = TcpStream::connect("143.244.190.188:22").unwrap();
    let mut sess1 = Session::new().unwrap();
    sess1.set_tcp_stream(tcp1);
    sess1.handshake().unwrap();
    sess1.userauth_password("root", "tesTer123s").unwrap();


    let mut channel = sess.channel_session().unwrap();
    channel.exec(r#"find ./check -name "*.csv" -type "f" -size -400b -delete"#).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    let mut channel = sess.channel_session().unwrap();
    channel.exec(r#"find check -type f"#).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    // println!("{}", s);
    let mut char_vec = s.split("\n");

    // char_vec.delete(1);
    for c in char_vec {
        if c == "" { continue; }

        println!("moving file > {}", c);
        let (mut remote_file, stat) = sess.scp_recv(Path::new(c)).unwrap();
        println!("remote file size: {}", stat.size());
        let mut contents = Vec::new();
        remote_file.read_to_end(&mut contents).unwrap();
        // Close the channel and wait for the whole content to be tranferred
        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();


        let mut remote_file = sess1.scp_send(Path::new(c),
                                             0o644, 10, None).unwrap();
        remote_file.write(&contents).unwrap();

        // Close the channel and wait for the whole content to be tranferred
        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();
    }

    channel.wait_close();
    // println!("{}", channel.exit_status().unwrap());
}
