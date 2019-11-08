use crate::prelude::{
    hmac::{Hmac, HmacEngine},
    sha256, FromHex, Hash, HashEngine, ToHex,
};
use bufstream::BufStream;
use rand::Rng;
use std::{
    convert::TryFrom,
    fs::{self, File},
    io::{self, BufRead, Read, Write},
    net::{TcpStream, ToSocketAddrs},
    path::PathBuf,
    str::FromStr,
};

const PROTOCOL_INFO_VERSION: i32 = 1;
const COOKIE_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 32;
static SERVER_KEY: &[u8; 56] = b"Tor safe cookie authentication server-to-controller hash";
static CONTROLLER_KEY: &[u8; 56] = b"Tor safe cookie authentication controller-to-server hash";

#[derive(Debug)]
pub enum TCErrorKind {
    ResourceExhausted,
    SyntaxErrorProtocol,
    UnrecognizedCmd,
    UnimplementedCmd,
    SyntaxErrorCmdArg,
    UnrecognizedCmdArg,
    AuthRequired,
    BadAuth,
    UnspecifiedTorError,
    InternalError,
    UnrecognizedEntity,
    InvalidConfigValue,
    InvalidDescriptor,
    UnmanagedEntity,
}
#[derive(Debug)]
pub enum TCError {
    IoError(io::Error),
    UnknownResponse,
    CannotReadAuthCookie,
    AuthenticationError,
    TorError(TCErrorKind),
}

impl TryFrom<u32> for TCErrorKind {
    type Error = ();
    fn try_from(code: u32) -> Result<Self, ()> {
        use TCErrorKind::*;
        match code {
            451 => Ok(ResourceExhausted),
            500 => Ok(SyntaxErrorProtocol),
            510 => Ok(UnrecognizedCmd),
            511 => Ok(UnimplementedCmd),
            512 => Ok(SyntaxErrorCmdArg),
            513 => Ok(UnrecognizedCmdArg),
            514 => Ok(AuthRequired),
            515 => Ok(BadAuth),
            550 => Ok(UnspecifiedTorError),
            551 => Ok(InternalError),
            552 => Ok(UnrecognizedEntity),
            553 => Ok(InvalidConfigValue),
            554 => Ok(InvalidDescriptor),
            555 => Ok(UnmanagedEntity),
            _ => Err(()),
        }
    }
}

impl From<io::Error> for TCError {
    fn from(err: io::Error) -> Self {
        TCError::IoError(err)
    }
}

type TCResult<T> = Result<T, TCError>;

pub struct TorControl(BufStream<TcpStream>);

#[derive(Debug)]
pub struct AddOnionConfig {
    pub virtual_port: u16,
    pub target_port: u16,
    pub private_key_path: PathBuf,
}

#[derive(Debug)]
pub struct OnionAddr {
    pub port: u16,
    pub onion_service: String,
}

#[derive(Debug)]
struct ProtocolInfo {
    cookiefile: String,
    auth_methods: Vec<String>,
    tor_version: String,
}

impl TorControl {
    pub fn connect(addr: impl ToSocketAddrs) -> TCResult<Self> {
        TorControl(BufStream::new(TcpStream::connect(addr)?)).authenticate()
    }

    pub fn add_v2_onion(&mut self, conf: AddOnionConfig) -> TCResult<OnionAddr> {
        let key_param = match fs::read_to_string(&conf.private_key_path) {
            Ok(key) => key,
            _ => "NEW:RSA1024".into(),
        };
        let port_param = format!("Port={},{}", conf.virtual_port, conf.target_port);
        send_command(
            &mut self.0,
            format!("ADD_ONION {} {}", key_param, port_param),
        )?;
        let response = read_lines(&mut self.0)?.join(" ");
        let mut service_id = "";
        let mut private_key = "";
        for section in response.split(' ') {
            let split: Vec<&str> = section.split('=').collect();
            if split.len() == 2 {
                match split[0] {
                    "ServiceID" => service_id = split[1],
                    "PrivateKey" => private_key = split[1],
                    _ => (),
                }
            }
        }
        if private_key != "" {
            let mut key_file = File::create(conf.private_key_path)?;
            key_file.write_all(private_key.as_bytes())?
        }
        Ok(OnionAddr {
            port: conf.virtual_port,
            onion_service: service_id.to_string() + ".onion",
        })
    }

    fn protocol_info(&mut self) -> TCResult<ProtocolInfo> {
        send_command(
            &mut self.0,
            format!("PROTOCOLINFO {}", PROTOCOL_INFO_VERSION),
        )?;
        let response = read_lines(&mut self.0)?.join(" ");
        let mut cookiefile = "";
        let mut auth_methods = "";
        let mut tor_version = "";
        for section in response.split(' ') {
            let split: Vec<&str> = section.split('=').collect();
            if split.len() == 2 {
                match split[0] {
                    "COOKIEFILE" => cookiefile = split[1],
                    "METHODS" => auth_methods = split[1],
                    "Tor" => tor_version = split[1],
                    _ => (),
                }
            }
        }
        Ok(ProtocolInfo {
            cookiefile: cookiefile.trim_matches('\"').into(),
            auth_methods: auth_methods.split(',').map(|s| s.into()).collect(),
            tor_version: tor_version.trim_matches('\"').into(),
        })
    }

    fn authenticate(mut self) -> TCResult<Self> {
        let auth_cookie = self.get_auth_cookie()?;

        let client_nonce = rand::thread_rng().gen::<[u8; NONCE_LENGTH]>();
        send_command(
            &mut self.0,
            format!("AUTHCHALLENGE SAFECOOKIE {}", <[u8]>::to_hex(&client_nonce)),
        )?;
        let response = read_lines(&mut self.0)?.join(" ");

        let mut serverhash = "";
        let mut servernonce = "";
        for section in response.split(' ') {
            let split: Vec<&str> = section.split('=').collect();
            if split.len() == 2 {
                match split[0] {
                    "SERVERHASH" => serverhash = split[1],
                    "SERVERNONCE" => servernonce = split[1],
                    _ => (),
                }
            }
        }
        let decoded_server_hash = FromStr::from_str(serverhash)
            .expect("Could not decode serverhash during Authentication");
        let decoded_server_nonce: Vec<u8> = FromHex::from_hex(servernonce)
            .expect("Could not decode servernonce during Authentication");

        let mut message = Vec::new();
        message.extend(auth_cookie);
        message.extend(&client_nonce);
        message.extend(decoded_server_nonce);

        let mut server_engine = HmacEngine::<sha256::Hash>::new(SERVER_KEY);
        server_engine.input(&message);
        let computed_server_hash = Hmac::<sha256::Hash>::from_engine(server_engine);
        if computed_server_hash.ne(&decoded_server_hash) {
            return Err(TCError::AuthenticationError);
        }

        let mut client_engine = HmacEngine::<sha256::Hash>::new(CONTROLLER_KEY);
        client_engine.input(&message);
        send_command(
            &mut self.0,
            format!(
                "AUTHENTICATE {}",
                Hmac::<sha256::Hash>::from_engine(client_engine)
            ),
        )?;

        read_lines(&mut self.0).map(|_| self)
    }

    fn get_auth_cookie(&mut self) -> TCResult<Vec<u8>> {
        let info = self.protocol_info()?;
        let mut file_content = Vec::new();
        let length = File::open(info.cookiefile)?.read_to_end(&mut file_content)?;
        if length != COOKIE_LENGTH {
            Err(TCError::CannotReadAuthCookie)
        } else {
            Ok(file_content)
        }
    }
}

#[allow(clippy::write_with_newline)]
fn send_command(writer: &mut impl Write, command: String) -> Result<(), io::Error> {
    write!(writer, "{}\r\n", command)?;
    writer.flush()
}

fn is_last_line(line: &str) -> TCResult<bool> {
    // Act upon separator:
    match &line[3..4] {
        // Meaning: this is the last line to read.
        " " => Ok(true),
        // We have more lines to read.
        "+" | "-" => Ok(false),
        _ => Err(TCError::UnknownResponse),
    }
}

fn parse_status(line: &str) -> TCResult<u32> {
    (&line[0..3]).parse().map_err(|_| TCError::UnknownResponse)
}

fn parse_line<'b>(
    stream: &mut impl BufRead,
    buf: &'b mut String,
) -> TCResult<(u32, bool, &'b str)> {
    // Read a line and make sure we have at least 3 (status) + 1 (sep) bytes.
    if stream.read_line(buf)? < 4 {
        return Err(TCError::UnknownResponse);
    }
    let (buf_s, msg) = buf.split_at(4);
    let status = parse_status(&buf_s)?;
    let is_last_line = is_last_line(&buf_s)?;
    Ok((status, is_last_line, msg))
}
fn read_lines(read: &mut impl BufRead) -> TCResult<Vec<String>> {
    let mut rls: Vec<String> = Vec::with_capacity(1);
    let mut buf = String::new();
    loop {
        {
            let (status, end, msg) = parse_line(read, &mut buf)?;
            handle_code(status)?;
            rls.push(msg.trim_end().to_owned());
            if end {
                break;
            }
        }
        buf.clear();
    }

    Ok(rls)
}

fn handle_code(status: u32) -> TCResult<()> {
    use TCError::*;
    match status {
        250 | 251 => Ok(()),
        status => Err(TCErrorKind::try_from(status)
            .map(TorError)
            .unwrap_or(UnknownResponse)),
    }
}
