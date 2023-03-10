## xwde: robotxt

[![Build Status][action-badge]][action-url]
[![Crate Docs][docs-badge]][docs-url]
[![Crate Version][crates-badge]][crates-url]

[action-badge]: https://img.shields.io/github/actions/workflow/status/xwde/robotxt/build.yaml?branch=main&label=build&logo=github&style=for-the-badge
[action-url]: https://github.com/xwde/robotxt/actions/workflows/build.yaml
[crates-badge]: https://img.shields.io/crates/v/robotxt.svg?logo=rust&style=for-the-badge
[crates-url]: https://crates.io/crates/robotxt
[docs-badge]: https://img.shields.io/docsrs/robotxt?logo=Docs.rs&style=for-the-badge
[docs-url]: http://docs.rs/robotxt

> **Warning** : The library is in active development. Expect breaking changes.

The implementation of the robots.txt protocol (or URL exclusion protocol) in
Rust programming language with the support of `crawl-delay`, `sitemap`, `host`
and universal `*` match extensions (according to the RFC specification).

#### Examples

- Parse the set of directives related to the specific `user-agent` in the
  provided `robots.txt` file.

```rust
use robotxt::Robots;

fn main() {
    let txt = r#"
      User-Agent: foobot
      Allow: /example/
      Disallow: /example/nope.txt
    "#;
    
    let r = Robots::from_string(txt, "foobot");
    assert!(r.is_match("/example/yeah.txt"));
    assert!(!r.is_match("/example/nope.txt"));
}
```

#### Links

- [Request for Comments: 9309](https://www.rfc-editor.org/rfc/rfc9309.txt) on
  RFC-Editor.com
- [Introduction to Robots.txt](https://developers.google.com/search/docs/crawling-indexing/robots/intro)
  on Google.com
- [How Google interprets Robots.txt](https://developers.google.com/search/docs/crawling-indexing/robots/robots_txt)
  on Google.com
- [What is Robots.txt file](https://moz.com/learn/seo/robotstxt) on Moz.com

#### Notes

The parser is based on:
[Smerity/texting_robots](https://github.com/Smerity/texting_robots) with
following differences:

- finds the longest match on the `user-agent` directives i.e. the user-agent
  `robotxt` matches the `user-agent: robot` directive.
- fixes patterns with both wildcards i.e. the path `/shark/fish` does not match
  the pattern `/fish*$` anymore.
- sorts patterns by `.len()` and `.is_allowed()` to provide better performance.
