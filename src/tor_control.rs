use std::iter;

use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::str;

use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

use std::ops::DerefMut;
use std::sync::mpsc::{self, channel, Receiver, RecvError, Sender};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

// BufStream:
extern crate bufstream;
use bufstream::BufStream;

extern crate futures;
use futures::{Async, Poll, Stream};

//============================================================================//
// Errors:                                                                    //
//============================================================================//

pub trait TCErrBase: From<io::Error> {
    fn unknown_error() -> Self;

    //fn is_unknown() -> bool;
}

pub trait TCStatusedError: From<u32> + TCErrBase {}

fn description_unknown() -> &'static str {
    "Tor Control replied with unknown response"
}

macro_rules! impl_err_base {
    ($typ: ident) => {
        impl TCErrBase for $typ {
            fn unknown_error() -> Self {
                $typ::UnknownResponse
            }
        }

        impl From<io::Error> for $typ {
            fn from(err: io::Error) -> Self {
                $typ::IoError(err)
            }
        }

        impl Display for $typ {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Debug::fmt(self, f)
            }
        }
    };
}

macro_rules! impl_err_statused {
    ($typ: ident) => {
        impl_err_base!($typ);
        impl TCStatusedError for $typ {}
        impl From<u32> for $typ {
            fn from(err: u32) -> Self {
                use $typ::*;
                err.try_into().map(TorError).unwrap_or(UnknownResponse)
            }
        }
    };
}

//============================================================================//
// Errors / TCEventsError:                                                    //
//============================================================================//

type AsyncNotify = (u32, bool, String);
type OptReader<T> = Option<BufReader<T>>;
type PEOptReader<'a, T> = PoisonError<MutexGuard<'a, OptReader<T>>>;

#[derive(Debug)]
pub enum TCEventsError {
    UnknownResponse,
    IoError(io::Error),
    SendError(mpsc::SendError<AsyncNotify>),
    PoisonError,
}

impl_err_base!(TCEventsError);

impl From<mpsc::SendError<AsyncNotify>> for TCEventsError {
    fn from(err: mpsc::SendError<AsyncNotify>) -> Self {
        TCEventsError::SendError(err)
    }
}

impl<'a, T> From<PEOptReader<'a, T>> for TCEventsError {
    fn from(_: PEOptReader<'a, T>) -> Self {
        TCEventsError::PoisonError
    }
}

//============================================================================//
// Errors / TCErrorKind:                                                      //
//============================================================================//

/// The kinds of errors that TorCP can issue as specified in `4. Replies` in the
/// TorCP specification. Note that codes `250`, `251` and `651` are not errors.
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

/// Conversions from `TCErrorKind` into the actual error code as specified in
/// `4. Replies` in the TorCP specification.
impl Into<u32> for TCErrorKind {
    fn into(self) -> u32 {
        use TCErrorKind::*;
        match self {
            ResourceExhausted => 451,
            SyntaxErrorProtocol => 500,
            UnrecognizedCmd => 510,
            UnimplementedCmd => 511,
            SyntaxErrorCmdArg => 512,
            UnrecognizedCmdArg => 513,
            AuthRequired => 514,
            BadAuth => 515,
            UnspecifiedTorError => 550,
            InternalError => 551,
            UnrecognizedEntity => 552,
            InvalidConfigValue => 553,
            InvalidDescriptor => 554,
            UnmanagedEntity => 555,
        }
    }
}

/// Conversions from error codes into as specified in
/// `4. Replies` in the TorCP specification.
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

fn description_kind(kind: &TCErrorKind) -> &str {
    use TCErrorKind::*;
    match *kind {
        ResourceExhausted => "Tor Control: Resource exhausted",
        SyntaxErrorProtocol => "Tor Control: Syntax error: protocol",
        UnrecognizedCmd => "Tor Control: Unrecognized command",
        UnimplementedCmd => "Tor Control: Unimplemented command",
        SyntaxErrorCmdArg => "Tor Control: Syntax error in command argument",
        UnrecognizedCmdArg => "Tor Control: Unrecognized command argument",
        AuthRequired => "Tor Control: Authentication required",
        BadAuth => "Tor Control: Bad authentication",
        UnspecifiedTorError => "Tor Control: Unspecified Tor error",
        InternalError => "Tor Control: Internal error",
        UnrecognizedEntity => "Tor Control: Unrecognized entity",
        InvalidConfigValue => "Tor Control: Invalid configuration value",
        InvalidDescriptor => "Tor Control: Invalid descriptor",
        UnmanagedEntity => "Tor Control: Unmanaged entity",
    }
}

//============================================================================//
// Errors / TCErrorAuth:                                                      //
//============================================================================//

/// The types of errors that can come as a result of interacting with TorCP.
#[derive(Debug)]
pub enum TCError {
    /// Wraps [`io:Error`](https://doc.rust-lang.org/std/io/struct.Error.html).
    IoError(io::Error),
    /// Indicates an unknown error code.
    UnknownResponse,
    /// Wraps **error** status codes that TorCP replies with.
    /// `250` and `251` are not errors, and thus is an `Ok(_)`.
    TorError(TCErrorKind),
}

impl_err_statused!(TCError);

impl Error for TCError {
    fn description(&self) -> &str {
        match *self {
            TCError::IoError(ref e) => e.description(),
            TCError::UnknownResponse => description_unknown(),
            TCError::TorError(ref kind) => description_kind(kind),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            TCError::IoError(ref e) => Some(e),
            _ => None,
        }
    }
}

//============================================================================//
// Internal / Reading utilities                                               //
//============================================================================//

/// Does a bunch of `write_all(...)` on a Write.
macro_rules! try_write {
    ( $s:expr, $( $x:expr ),* ) => {
        $( $s.write_all( $x )?; )*
    };
}

/// Writes end of line and flushes on a Write.
fn write_end<W: Write, E: TCErrBase>(w: &mut W) -> Result<(), E> {
    try_write!(w, b"\r\n");
    w.flush()?;
    Ok(())
}

/// Combines `try_write!(...)` and `write_end` on a Write.
macro_rules! try_wend {
    ( $w:ty, $e:ty, $s:expr ) => { write_end::<$w, $e>(&mut $s)? };
    ( $w:ty, $e:ty, $s:expr, $( $x:expr ),* ) => {
        try_write!( $s, $($x),* );
        try_wend!( $w, $e, $s );
    };
}

macro_rules! try_wendtc {
    ( $s:expr ) => { write_end::<Self::Writer, Self::Error>(&mut $s)? };
    ( $s:expr, $( $x:expr ),* ) => {
        try_write!( $s, $($x),* );
        try_wendtc!( $s );
    };
}

/// Writes an iterator of byte arrays to the stream, separated by whitespace.
fn write_many<W, Is, E>(writer: &mut W, items: Is) -> Result<usize, E>
where
    W: Write,
    Is: IntoIterator,
    Is::Item: AsRef<[u8]>,
    E: TCErrBase,
{
    let mut c = 0;
    for item in items.into_iter() {
        try_write!(writer, b" ", item.as_ref());
        c += 1;
    }
    Ok(c)
}

//============================================================================//
// Internal / Reading utilities                                               //
//============================================================================//

/// Converts a "string" to a status code, or fails with `UnknownResponse`.
fn read_status<E: TCErrBase>(line: &str) -> Result<u32, E> {
    (&line[0..3]).parse().map_err(|_| E::unknown_error())
}

fn read_is_end<E: TCErrBase>(line: &str) -> Result<bool, E> {
    // Act upon separator:
    match &line[3..4] {
        // Meaning: this is the last line to read.
        " " => Ok(true),
        // We have more lines to read.
        "+" | "-" => Ok(false),
        _ => Err(E::unknown_error()),
    }
}

fn read_line<'b, R: BufRead, E: TCErrBase>(
    stream: &mut R,
    buf: &'b mut String,
) -> Result<(u32, bool, &'b str), E> {
    // Read a line and make sure we have at least 3 (status) + 1 (sep) bytes.
    if stream.read_line(buf)? < 4 {
        return Err(E::unknown_error());
    }
    let (buf_s, msg) = buf.split_at(4);
    let status = read_status::<E>(&buf_s)?;
    let is_end = read_is_end::<E>(&buf_s)?;
    Ok((status, is_end, msg))
}

/// Handles a status code, 250 or 251 is Ok, otherwise error.
fn handle_code<E: TCStatusedError>(status: u32) -> Result<(), E> {
    match status {
        250 | 251 => Ok(()),
        status => Err(status.into()),
    }
}

/// Reads a status code, if `250` -> `Ok(())`, otherwise -> error.
fn read_ok_sync<R: BufRead, E: TCStatusedError>(read: &mut R) -> Result<(), E> {
    let mut buf = String::new();
    let (status, end, _) = read_line::<R, E>(read, &mut buf)?;
    if end {
        handle_code(status)
    } else {
        Err(E::unknown_error())
    }
}

/// Reads one or many reply lines as specified in `2.3`.
/// Terminates early on status code other than `250`.
fn read_lines_sync<R, E>(read: &mut R) -> Result<Vec<String>, E>
where
    R: BufRead,
    E: TCStatusedError,
{
    let mut rls: Vec<String> = Vec::with_capacity(1);
    let mut buf = String::new();
    loop {
        {
            let (status, end, msg) = read_line::<R, E>(read, &mut buf)?;
            handle_code::<E>(status)?;
            rls.push(msg.trim_end().to_owned());
            if end {
                break;
            }
        }
        buf.clear();
    }

    Ok(rls)
}

//============================================================================//
// Traits needed for backends:                                                //
//============================================================================//

pub trait TryClone
where
    Self: Sized,
{
    fn try_clone(&self) -> io::Result<Self>;
}

//============================================================================//
// API Traits:                                                                //
//============================================================================//

pub trait Connectable
where
    Self: Sized,
{
    type Error;
    fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, Self::Error>;
}

pub trait IsAuth {
    /// Returns true if we are authenticated.
    fn is_auth(&self) -> bool;
}

pub trait IsAsync {
    /// Returns true if we are in async mode.
    fn is_async(&self) -> bool;
}

pub trait TorLimited {
    type Writer: Write;
    type Error: TCStatusedError;

    #[doc(hidden)]
    fn into_writer(self) -> Self::Writer;

    #[doc(hidden)]
    fn writer(&mut self) -> &mut Self::Writer;

    #[doc(hidden)]
    fn read_ok(&mut self) -> Result<(), Self::Error>;

    #[doc(hidden)]
    /// Reads one or many reply lines as specified in `2.3`.
    /// Terminates early on status code other than `250`.
    fn read_lines(&mut self) -> Result<Vec<String>, Self::Error>;

    /// Tells the server to hang up on this controller connection as specified
    /// in `3.18. QUIT`
    fn quit(self) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        let mut w = self.into_writer();
        try_write!(w, b"QUIT");
        write_end(&mut w)
    }

    // 3.21. PROTOCOLINFO
    fn protocol_info(&mut self) -> Result<Vec<String>, Self::Error> {
        try_wend!(Self::Writer, Self::Error, self.writer(), b"PROTOCOLINFO 1");
        self.read_lines()
    }
}

pub trait TorControl: TorLimited {
    #[doc(hidden)]
    /// Executes a simple "one-shot" command expecting a 250 OK reply back.
    fn command<P>(&mut self, cmd: P) -> Result<(), Self::Error>
    where
        P: AsRef<[u8]>,
    {
        try_wendtc!(self.writer(), cmd.as_ref());
        self.read_ok()
    }

    #[doc(hidden)]
    /// Used for `setconf` and `resetconf`.
    fn xsetconf<K, V>(&mut self, cmd: &[u8], kw: K, val: Option<V>) -> Result<(), Self::Error>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        {
            let mut writer = self.writer();
            try_wendtc!(writer, cmd.as_ref(), b" ", kw.as_ref());
            if let Some(value) = val {
                try_write!(writer, b" = ", value.as_ref());
            }
            try_wendtc!(writer);
        }

        self.read_ok()
    }

    /// Sets a configuration as specified in `3.1. SETCONF`.
    ///
    /// It sets the configuration variable specified by `kw` to `value`
    /// when `val == Some(value)` is given. Otherwise, on `None`,
    /// it is reset to `0` or `NULL`.
    fn setconf<K, V>(&mut self, kw: K, val: Option<V>) -> Result<(), Self::Error>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        self.xsetconf(b"SETCONF", kw, val)
    }

    /// Sets a configuration as specified in `3.2. RESETCONF`.
    ///
    /// Behaves as [`setconf`] in every respect except for what happens when
    /// `val == None`. In that case, the configuration variable specified by
    /// `kw` is reset to the default value.
    ///
    /// [`setconf`]: struct.TorControl.html#method.setconf
    fn resetconf<K, V>(&mut self, kw: K, val: Option<V>) -> Result<(), Self::Error>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        self.xsetconf(b"RESETCONF", kw, val)
    }

    /// Gets a configuration as specified in `3.3. GETCONF`.
    ///
    /// Requests the value(s) of a configuration variable specified by keys `kws`.
    /// If any key does not correspond to a valid variable, an error is "thrown".
    ///
    /// # Examples
    ///
    /// Let's assume that we have `torrc` file that includes, among other things:
    ///
    /// ```text
    /// SOCKSPolicy accept 127.0.0.1
    /// SOCKSPolicy reject *
    /// HashedControlPassword 16:1E4D6C2977B2413E60A8563914D60B3F5D6888929178436A0AA23D5176
    /// ControlPort 9051
    /// ```
    ///
    /// In this case, we try:
    ///
    /// ```rust
    /// use tor_control::TorControl;
    /// let mut tc = TorControl::connect("127.0.0.1:9051").unwrap();
    /// tc.auth(Some("\"password\"")).unwrap();
    /// println!("{:?}", tc.getconf(vec!["SOCKSPolicy", "Nickname"]).unwrap());
    /// ```
    ///
    /// Which would print out:
    ///
    /// ```text
    /// ["SocksPolicy=accept 127.0.0.1", "SocksPolicy=reject *", "Nickname"]
    /// ```
    fn getconf<Ks>(&mut self, kws: Ks) -> Result<Vec<String>, Self::Error>
    where
        Ks: IntoIterator,
        Ks::Item: AsRef<[u8]>,
    {
        {
            // Format is:
            // "GETCONF" 1*(SP keyword) CRLF
            // Write the command:
            let mut writer = self.writer();
            try_write!(writer, b"GETCONF");

            // Write all keywords to get for:
            write_many::<Self::Writer, Ks, Self::Error>(&mut writer, kws)?;
            try_wendtc!(writer);
        }

        self.read_lines()
    }

    /// Gets a configuration as specified in `3.9. GETINFO`.
    fn getinfo<Ks>(&mut self, kws: Ks) -> Result<Vec<String>, Self::Error>
    where
        Ks: IntoIterator,
        Ks::Item: AsRef<[u8]>,
    {
        {
            // Format is:
            // "GETINFO" 1*(SP keyword) CRLF
            // Write the command:
            let mut writer = self.writer();
            try_write!(writer, b"GETINFO");

            // Write all keywords to get for:
            write_many::<Self::Writer, Ks, Self::Error>(&mut writer, kws)?;
            try_wendtc!(writer);
        }

        self.read_lines()
    }

    /// Gets a configuration as specified in `3.3. GETCONF`.
    ///
    /// Behaves like [`getconf`] except that it takes only one key.
    ///
    /// Also, if the variable is set to its default state, `None` is returned.
    /// If it is not, the value(s) are returned as [`String`]s.
    /// Note that the [`String`]s only include everything after `=`.
    ///
    /// # Examples
    ///
    /// Let's assume that we have `torrc` file that includes, among other things:
    ///
    /// ```text
    /// SOCKSPolicy accept 127.0.0.1
    /// SOCKSPolicy reject *
    /// HashedControlPassword 16:1E4D6C2977B2413E60A8563914D60B3F5D6888929178436A0AA23D5176
    /// ControlPort 9051
    /// ```
    ///
    /// In this case, we try:
    ///
    /// ```
    /// use tor_control::TorControl;
    /// let mut tc = TorControl::connect("127.0.0.1:9051").unwrap();
    /// tc.auth(Some("\"password\"")).unwrap();
    /// println!("{:?}", tc.getconf0("SOCKSPolicy").unwrap());
    /// println!("{:?}", tc.getconf0("Nickname").unwrap());
    /// ```
    ///
    /// Which would print out:
    ///
    /// ```text
    /// Some(["accept 127.0.0.1", "reject *"])
    /// None
    /// ```
    ///
    /// [`getconf`]: struct.TorControl.html#method.getconf
    /// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
    fn getconf0<K>(&mut self, kw: K) -> Result<Option<Vec<String>>, Self::Error>
    where
        K: AsRef<[u8]>,
    {
        // Read variables:
        let lines = self.getconf(iter::once(kw))?;

        // Strip everything before = in reply lines, and if it wasn't found,
        // indicate that we found the default value by returning None.
        let mut retr = Vec::with_capacity(lines.len());
        for line in lines {
            match line.rfind('=') {
                None => return Ok(None),
                Some(idx) => retr.push(line[idx + 1..].into()),
            }
        }
        Ok(Some(retr))
    }

    /// Issues a `SAVECONF` command as specified in `3.6. SAVECONF`.
    fn saveconf(&mut self) -> Result<(), Self::Error> {
        self.command(b"SAVECONF")
    }

    /// Issues a `NEWNYM` signal as specified in `3.7. SIGNAL`.
    fn newnym(&mut self) -> Result<(), Self::Error> {
        self.command(b"SIGNAL NEWNYM")
    }

    /// Issues a `CLEARDNSCACHE` signal as specified in `3.7. SIGNAL`.
    fn clear_dns_cache(&mut self) -> Result<(), Self::Error> {
        self.command(b"SIGNAL CLEARDNSCACHE")
    }

    /// Issues a `HEARTBEAT` signal as specified in `3.7. SIGNAL`.
    fn heartbeat(&mut self) -> Result<(), Self::Error> {
        self.command(b"SIGNAL HEARTBEAT")
    }

    /// Issues a `RELOAD` signal as specified in `3.7. SIGNAL`.
    fn reload(&mut self) -> Result<(), Self::Error> {
        // same as: HUP
        self.command(b"SIGNAL RELOAD")
    }

    /// Issues a `SHUTDOWN` signal as specified in `3.7. SIGNAL`.
    fn shutdown(&mut self) -> Result<(), Self::Error> {
        // same as: INT
        self.command(b"SIGNAL SHUTDOWN")
    }

    /// Issues a `HALT` signal as specified in `3.7. SIGNAL`.
    fn halt(&mut self) -> Result<(), Self::Error> {
        // same as: TERM
        self.command(b"SIGNAL HALT")
    }

    /// Issues a `DUMP` signal as specified in `3.7. SIGNAL`.
    fn dump(&mut self) -> Result<(), Self::Error> {
        // same as: USR1
        self.command(b"SIGNAL DUMP")
    }

    /// Issues a `DEBUG` signal as specified in `3.7. SIGNAL`.
    fn debug(&mut self) -> Result<(), Self::Error> {
        // same as: USR2
        self.command(b"SIGNAL DEBUG")
    }
}

//============================================================================//
// TcpStream implementation:                                                  //
//============================================================================//

impl Connectable for TcpStream {
    type Error = io::Error;
    fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, Self::Error> {
        Self::connect(addr)
    }
}

impl TryClone for TcpStream {
    fn try_clone(&self) -> io::Result<Self> {
        self.try_clone()
    }
}

//============================================================================//
// TCNoAuth                                                                   //
//============================================================================//

pub struct TCNoAuth<T: Read + Write>(BufStream<T>);

impl<T: Read + Write> IsAuth for TCNoAuth<T> {
    fn is_auth(&self) -> bool {
        false
    }
}

impl<T: Read + Write> IsAsync for TCNoAuth<T> {
    fn is_async(&self) -> bool {
        false
    }
}

impl<T: Read + Write> TorLimited for TCNoAuth<T> {
    type Writer = BufStream<T>;
    type Error = TCError;

    fn into_writer(self) -> Self::Writer {
        self.0
    }

    fn writer(&mut self) -> &mut Self::Writer {
        &mut self.0
    }

    fn read_ok(&mut self) -> Result<(), TCError> {
        read_ok_sync(&mut self.0)
    }

    fn read_lines(&mut self) -> Result<Vec<String>, TCError> {
        read_lines_sync(&mut self.0)
    }
}

impl<T> Connectable for TCNoAuth<T>
where
    T: Connectable<Error = io::Error> + Read + Write,
{
    type Error = TCError;
    fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, Self::Error> {
        Ok(TCNoAuth::new(T::connect(addr)?))
    }
}

impl<T: Read + Write> TCNoAuth<T> {
    /// Constructs an interface to TorCP given the backing stream of type `T`,
    /// which is most often a `TcpStream`.
    pub fn new(stream: T) -> Self {
        TCNoAuth(BufStream::new(stream))
    }

    /// Authenticates to TorCP as specified in `3.5. AUTHENTICATE`.
    ///
    /// If no password is required, `mpass == None`, otherwise `Some("<pass>")`.
    pub fn auth<P>(self, mpass: Option<P>) -> Result<TCAuth<T>, TCError>
    where
        P: AsRef<[u8]>,
    {
        let mut stream = self.0;

        if let Some(pass) = mpass {
            try_wend!(
                BufStream<T>,
                TCError,
                stream,
                b"AUTHENTICATE ",
                pass.as_ref()
            );
        } else {
            try_wend!(BufStream<T>, TCError, stream, b"AUTHENTICATE");
        }

        read_ok_sync::<BufStream<T>, TCError>(&mut stream)?;

        Ok(TCAuth(stream))
    }
}

//============================================================================//
// TCAuth:                                                                    //
//============================================================================//

pub struct TCAuth<T: Read + Write>(BufStream<T>);

impl<T: Read + Write> IsAuth for TCAuth<T> {
    fn is_auth(&self) -> bool {
        true
    }
}

impl<T: Read + Write> IsAsync for TCAuth<T> {
    fn is_async(&self) -> bool {
        false
    }
}

impl<T: Read + Write> TorLimited for TCAuth<T> {
    type Writer = BufStream<T>;
    type Error = TCError;

    fn into_writer(self) -> Self::Writer {
        self.0
    }

    fn writer(&mut self) -> &mut Self::Writer {
        &mut self.0
    }

    fn read_ok(&mut self) -> Result<(), TCError> {
        read_ok_sync(&mut self.0)
    }

    fn read_lines(&mut self) -> Result<Vec<String>, TCError> {
        read_lines_sync(&mut self.0)
    }
}

impl<T: Read + Write> TorControl for TCAuth<T> {}

//============================================================================//
// SETEVENTS, 3.4                                                             //
//============================================================================//

pub type Event = String;

type StealableReader<T> = Arc<Mutex<OptReader<T>>>;

pub struct TCEvents<T: Read + Write> {
    reader: StealableReader<T>,
    sync_tx: Sender<AsyncNotify>,
}

fn setevents_common<Es, Err, W>(writer: &mut W, extended: bool, events: Es) -> Result<(), Err>
where
    Es: IntoIterator,
    Es::Item: AsRef<[u8]>,
    Err: TCErrBase,
    W: Write,
{
    // Format is:
    // "SETEVENTS" [SP "EXTENDED"] *(SP EventCode) CRLF
    // EventCode = 1*(ALPHA / "_")

    // Write the command:
    try_write!(writer, b"SETEVENTS");

    // Extended mode or not?
    if extended {
        try_write!(writer, b" EXTENDED");
    }

    // Subscribe to all events & check if we're OK:
    write_many::<W, Es, Err>(writer, events)?;
    write_end::<W, Err>(writer)
}

impl<T: Read + Write> Stream for TCEvents<T> {
    type Item = String;
    type Error = TCEventsError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.reader.lock()?.deref_mut() {
            &mut None => Ok(Async::Ready(None)),
            &mut Some(ref mut reader) => {
                let mut buf = String::new();
                let (status, end, msg) = read_line::<BufReader<T>, Self::Error>(reader, &mut buf)?;

                if status == 650 {
                    Ok(Async::Ready(Some(msg.to_owned())))
                } else {
                    self.sync_tx.send((status, end, msg.to_owned()))?;
                    Ok(Async::NotReady)
                }
            }
        }
    }
}
