use std::time::Duration;
use url::Url;

use crate::internal::Rule;

///
pub fn try_sitemaps(u: &[u8]) -> Option<Url> {
    let u = String::from_utf8(u.to_vec()).ok()?;
    let u = Url::parse(u.as_str()).ok()?;
    Some(u)
}

///
pub fn try_rule(u: &[u8], allow: bool) -> Option<Rule> {
    let u = String::from_utf8(u.to_vec()).ok()?;
    let u = Rule::new(u.as_str(), allow).ok()?;
    Some(u)
}

///
pub fn try_delay(u: &[u8]) -> Option<Duration> {
    let u = String::from_utf8(u.to_vec()).ok()?;
    let u = u.parse::<f32>().ok()?;
    let u = Duration::try_from_secs_f32(u).ok()?;
    Some(u)
}
