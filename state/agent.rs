use std::string::FromUtf8Error;

///
#[derive(Debug, Clone)]
pub struct UserAgent {
    vec: Vec<u8>,
    utf8: String,

    best_vec: Option<Vec<u8>>,
    best_utf8: Option<String>,
}

impl UserAgent {
    ///
    pub fn new(agent: &[u8]) -> Result<Self, FromUtf8Error> {
        let utf8 = String::from_utf8(agent.to_vec())?;
        let utf8 = Self::normalize(utf8.as_str());
        let vec = agent.to_vec();
        Ok(Self {
            vec,
            utf8,
            best_vec: None,
            best_utf8: None,
        })
    }

    ///
    pub fn utf8(&self) -> &String {
        &self.utf8
    }

    ///
    fn normalize(agent: &str) -> String {
        agent.trim().to_lowercase()
    }
}
