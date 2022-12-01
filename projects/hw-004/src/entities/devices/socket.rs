use crate::entities::reportable::Reportable;
use std::fmt::{Display, Formatter, Result};

/// [SocketStatus] enum represents the possible statuses of the
/// smart socket. As for now, its small and simple enum containing
/// only two possible states, but in the future it might be enriched
/// by additional rare-usable statuses.
#[derive(Debug)]
pub enum SocketStatus {
    /// The socket is enabled and it might provide the report about socket
    Enabled,
    /// The socket is disabled and no action might be done on this socket
    /// until the socket will [SocketStatus::Enabled]
    Disabled,
}

/// An obvious implementation of [Display] for the [SocketStatus] enum. I'm
/// wondering why this implementation might not be derived via useful macros
/// from std lib. I need to implement it, or ask someone to guide me on this
/// issue.
impl Display for SocketStatus {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{}", format!("{:?}", self).to_lowercase())
    }
}

/// A representation of the smart socket. Each device entity in this project
/// must have the name and description. Socket entity also has two additional
/// fields such as `power_consumption` and `status`. I guess, there is no need
/// to write it down the meaning of these additional fields
#[derive(Debug)]
pub struct Socket {
    name: String,
    description: Option<String>,
    power_consumption: f32,
    status: SocketStatus,
}

/// An implementation of the Socket struct. All of these methods and functions
/// are super obvious, but I must to describe all of them to make the documentation
/// full and fit the high standards of the rust library docs.
impl Socket {
    /// The associated function for creating a new instance of the smart Socket
    /// It will create an instance with the given name and description. It will
    /// create a clone of each given parameter, so be careful for memory leaks
    /// especially with long name and description. All other fields will be
    /// derived from the [Default] implementation of the Socket struct.
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: Some(description.to_string()),
            ..Self::default()
        }
    }

    /// A getter method for getting the clone of the description of the Socket.
    /// I know it's super weird approach, but let's keet it as is, because I don't
    /// want to deal with references and their lifetimes. Hopefully, I'll have a time
    /// to fix it, but to be honest, it seems to be never.
    pub fn get_description(&self) -> Option<String> {
        self.description.as_ref().cloned()
    }

    /// Method for enabling the socket. It needs a mutable reference and it
    /// overwrites the current socket status.
    pub fn enable(&mut self) {
        self.status = SocketStatus::Enabled
    }

    /// Method for disabling the socket. It needs a mutable reference and it
    /// overwrites the current socket status.
    pub fn disable(&mut self) {
        self.status = SocketStatus::Disabled
    }

    /// As the name of this function claims, it returns the current power consumption
    /// in milli amperes (or whatever it's consumes)
    pub fn get_current_power_consumption(&self) -> f32 {
        rand::random()
    }
}

/// A Default implementation for the Socket struct. All the fields will be set by
/// type default implementation, expect the name of the Socket will have the `Default socket`
/// string. The default status of the Socket will be set to [SocketStatus::Disabled]
impl Default for Socket {
    fn default() -> Self {
        Self {
            name: "Default socket".to_string(),
            description: None,
            power_consumption: 0.0,
            status: SocketStatus::Disabled,
        }
    }
}

/// A detailed information about Socket. It writes out the full information about
/// the Socket regardless the current socket status. It might useful to print
/// the full information about Socket instance.
impl Display for Socket {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let txt = format!(
            "Socket: {}, {}, {}, {}",
            self.name,
            self.description.clone().unwrap_or_default(),
            self.power_consumption,
            self.status
        );

        write!(formatter, "{}", txt)
    }
}

/// A reportable implementation for the Socket struct gives the short and fast report
/// of current status of the socket. It prints out socket name and the status. In the
/// future's implementations it might also has some variety depending on the current status,
/// involving the usage of `power_consumption` field.
impl Reportable for Socket {
    fn report(&self) -> String {
        format!("Socket: {}, Status: {}", self.name, self.status)
    }
}
