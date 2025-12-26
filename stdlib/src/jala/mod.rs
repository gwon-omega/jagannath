//! Jāla - Networking (जाल)
//!
//! Network I/O operations.

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket, SocketAddr, ToSocketAddrs};

/// TCP listener (Śravaṇa - श्रवण)
pub struct Shravana {
    inner: TcpListener,
}

impl Shravana {
    /// Bind to address (बन्धन - bandhana)
    pub fn bandhana<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        Ok(Self {
            inner: TcpListener::bind(addr)?
        })
    }

    /// Accept connection (स्वीकार - svīkāra)
    pub fn svikara(&self) -> io::Result<(Sambandha, SocketAddr)> {
        let (stream, addr) = self.inner.accept()?;
        Ok((Sambandha { inner: stream }, addr))
    }

    /// Local address (स्थानीय - sthānīya)
    pub fn sthaniya(&self) -> io::Result<SocketAddr> {
        self.inner.local_addr()
    }
}

/// TCP connection (Sambandha - सम्बन्ध)
pub struct Sambandha {
    inner: TcpStream,
}

impl Sambandha {
    /// Connect to address (योजय - yojaya)
    pub fn yojaya<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        Ok(Self {
            inner: TcpStream::connect(addr)?
        })
    }

    /// Read data (पठ - paṭha)
    pub fn patha(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }

    /// Write data (लिख - likha)
    pub fn likha(&mut self, data: &[u8]) -> io::Result<usize> {
        self.inner.write(data)
    }

    /// Flush (प्रवाह - pravāha)
    pub fn pravaha(&mut self) -> io::Result<()> {
        self.inner.flush()
    }

    /// Local address (स्थानीय - sthānīya)
    pub fn sthaniya(&self) -> io::Result<SocketAddr> {
        self.inner.local_addr()
    }

    /// Remote address (दूरस्थ - dūrastha)
    pub fn durastha(&self) -> io::Result<SocketAddr> {
        self.inner.peer_addr()
    }
}

/// UDP socket (Svataṃtra-Jāla - स्वतन्त्रजाल)
pub struct SvatantraJala {
    inner: UdpSocket,
}

impl SvatantraJala {
    /// Bind to address (बन्धन - bandhana)
    pub fn bandhana<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        Ok(Self {
            inner: UdpSocket::bind(addr)?
        })
    }

    /// Send to address (प्रेषय - preṣaya)
    pub fn preshaya<A: ToSocketAddrs>(&self, buf: &[u8], addr: A) -> io::Result<usize> {
        self.inner.send_to(buf, addr)
    }

    /// Receive data (प्राप्नोति - prāpnoti)
    pub fn prapnoti(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.inner.recv_from(buf)
    }
}
