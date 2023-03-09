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

#### Features

- `time` to enable

#### Examples

- Parse only directives related to the specified `user-agent` in the provided
  file.

```rust
```

- Parse all directives in the provided file.

```rust
```

- Build the new file from the provided directives.

```rust
```

#### Links

- [Request for Comments: 9309](https://www.rfc-editor.org/rfc/rfc9309.txt) on
  RFC-Editor.com
- [Robots.txt](https://en.wikipedia.org/wiki/Robots.txt) on Wikipedia.org
- [Introduction to Robots.txt](https://developers.google.com/search/docs/crawling-indexing/robots/intro)
  on Google.com
- [How Google interprets the robots.txt specification](https://developers.google.com/search/docs/crawling-indexing/robots/robots_txt)
  on Google.com
- [What is a robots.txt file](https://moz.com/learn/seo/robotstxt) on Moz.com

#### Other projects

- [Smerity/texting_robots](https://github.com/Smerity/texting_robots):
  - does not perform the longest match on the user agent directive
  - matches the path `/shark/fish` to the pattern `/fish*$`
