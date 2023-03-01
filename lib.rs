#![forbid(unsafe_code)]
#![feature(once_cell)]

//!
//!
//!

mod build;
mod directive;
mod parse;
mod state;

mod ideas {
    use std::collections::HashMap;
    use url::Url;

    pub struct RobotsRule {}

    pub struct RobotsError {}

    // pub enum RobotsData {
    //     Single(String, Vec<RobotsRule>),
    //     Whole(HashMap<String, Vec<RobotsRule>>),
    // }
    //
    // pub struct Robots {
    //     data: RobotsData,
    //     sitemaps: Vec<Url>,
    //     host: Option<Url>,
    // }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub struct AgentName(String);

    pub struct AgentData {
        rules: Vec<RobotsRule>,
        delay: Option<i32>,
    }

    pub struct Robots {
        data: HashMap<AgentName, AgentData>,
        sitemaps: Vec<Url>,
        host: Option<Url>,
    }
}

mod builder {
    use std::error::Error;
    use std::fmt::{Display, Formatter, Result as FmtResult};
    use std::io::{BufWriter, Error as IoError, Write};
    use url::Url;

    #[derive(Debug)]
    pub enum BuilderError {
        IoError(IoError),
    }

    impl Display for BuilderError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            todo!()
        }
    }

    impl From<IoError> for BuilderError {
        fn from(error: IoError) -> Self {
            Self::IoError(error)
        }
    }

    impl Error for BuilderError {}

    #[derive(Debug)]
    pub enum BuilderRule {
        Allow(String),
        Disallow(String),
        CrawlDelay(i32),
    }

    impl Display for BuilderRule {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            match &self {
                Self::Allow(_) => todo!(),
                Self::Disallow(_) => todo!(),
                Self::CrawlDelay(_) => todo!(),
            }
        }
    }

    pub struct BuilderData {}

    pub struct Builder {}

    impl Builder {
        pub fn build<W: Write>(writer: W) -> Result<(), BuilderError> {
            let mut writer = BufWriter::new(writer);
            Ok(writer.flush()?)
        }

        pub fn with_rule(&mut self, agent: &str, rule: BuilderRule) {
            todo!()
        }

        pub fn with_host(&mut self, host: Url) {
            todo!()
        }

        pub fn with_sitemap(&mut self, sitemap: Url) {
            todo!()
        }
    }
}

mod parser {
    use std::io::{BufReader, Read};

    pub trait Parser<R> {}

    pub struct RobotsParser<R: Read> {
        reader: BufReader<R>,
    }

    impl<R: Read> RobotsParser<R> {
        pub fn new(reader: R) -> Self {
            let reader = BufReader::new(reader);
            Self { reader }
        }

        pub fn next() {}
    }
}
