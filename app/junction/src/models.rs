use derivative::Derivative;
use std::error::Error;
use std::fmt;

#[derive(Derivative, Eq, PartialEq)]
#[derivative(Debug, Default)]
pub struct Broker {
    hostname: String,

    #[derivative(Default(value = "9092"))]
    port: u16,
}

#[derive(Debug)]
struct BrokerError(String);

impl fmt::Display for BrokerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid broker: {}", self.0)
    }
}

impl Error for BrokerError {}

impl Broker {
    pub fn to_connection_string(self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }

    pub fn from_connection_string(str: String) -> Result<Broker, Box<dyn Error>> {
        let mut components = str.split(":");
        let hostname= match components.next() {
            Some(hostname) => String::from(hostname),
            None => return Err(Box::new(BrokerError(String::from("No hostname specified"))))
        };
        let port: u16 = if let Some(port_string) = components.next() {
            port_string.parse()?
        } else {
            9092
        };
        Ok(Broker {
            hostname,
            port
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_connection_string() {
        let broker = Broker {
            hostname: String::from("localhost"),
            port: 8080
        };
        assert_eq!(broker.to_connection_string(), "localhost:8080")
    }

    #[test]
    fn test_from_connection_string() {
        let actual = Broker::from_connection_string(String::from("localhost:8080"))
            .expect("Invalid broker");

        let expected = Broker {
            hostname: String::from("localhost"),
            port: 8080
        };
        assert_eq!(expected, actual)
    }
}