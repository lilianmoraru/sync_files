use ssh2::Session;
use std::ffi::OsString;

pub fn get_session<IP, U, P>(ip: IP, user: U, pass: P) -> Session
    where IP: ToSocketAddrs,
          U: AsRef<OsString>,
          P: AsRef<OsString>
{
    let tcp = TcpStream::connect(ip).unwrap();
    let mut session = Session::new().unwrap();
    session.handshake(&tcp).unwrap();
    session.userauth_agent(user.as_ref()).unwrap();

    assert!(session.authenticated());

    session
}
