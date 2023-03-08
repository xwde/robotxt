///
#[derive(Debug, Clone)]
pub struct UserAgent(String);

impl UserAgent {
    ///
    pub fn new(agent: &str) -> Self {
        let agent = Self::normalize(agent);
        Self(agent)
    }

    ///
    pub fn inner(&self) -> &String {
        &self.0
    }

    ///
    fn normalize(agent: &str) -> String {
        agent.trim().to_lowercase()
    }
}
