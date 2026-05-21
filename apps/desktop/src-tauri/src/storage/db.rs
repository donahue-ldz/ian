use std::{io, path::Path};

use rusqlite::Connection;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn open(app_dir: &Path) -> io::Result<Self> {
        let path = app_dir.join("ian.db");
        let connection =
            Connection::open(path).map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

        Ok(Self { connection })
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }
}
