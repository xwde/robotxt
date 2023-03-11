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

The implementation of the robots.txt (or URL exclusion) protocol in the Rust
programming language with the support of `crawl-delay`, `sitemap` and universal
`*` match extensions (according to the RFC specification).

### Examples

- parse the `user-agent` in the provided `robots.txt` file:

```rust
use robotxt::Robots;

fn main() {
    let txt = r#"
      User-Agent: foobot
      Allow: /example/
      Disallow: /example/nope.txt
    "#.as_bytes();
    
    let r = Robots::from_slice(txt, "foobot");
    assert!(r.is_match("/example/yeah.txt"));
    assert!(!r.is_match("/example/nope.txt"));
}
```

- build the new `robots.txt` file from provided directives:

> **Note** : the builder is not yet implemented.

```rust
```

### Links

- [Request for Comments: 9309](https://www.rfc-editor.org/rfc/rfc9309.txt) on
  RFC-Editor.com
- [Introduction to Robots.txt](https://developers.google.com/search/docs/crawling-indexing/robots/intro)
  on Google.com
- [How Google interprets Robots.txt](https://developers.google.com/search/docs/crawling-indexing/robots/robots_txt)
  on Google.com
- [What is Robots.txt file](https://moz.com/learn/seo/robotstxt) on Moz.com

### Notes

The parser is based on
[Smerity/texting_robots](https://github.com/Smerity/texting_robots).
