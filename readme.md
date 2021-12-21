A crate for flexible version numbers, mostly inspired by [Ruby's Gem::Version](https://ruby-doc.org/stdlib-3.0.3/libdoc/rubygems/rdoc/Gem/Version.html). This crate provides a flexible versioning structure which is compatible with [Semantic Versioning](https://semver.org/), and some other obscure versioning schemas used in the wild.

## Syntax
Versions are defined as components separated by dots, minus or plus signs. Each component might be either a number, or an alphabetic string. 

| Version string | Canonical representation |
|:---------------|:-------------------------|
| `0`            | `0`                      |
| `0.9`          | `0.9`                    |
| `0.9a`         | `0.9.a`                  |
| `0.9.a`        | `0.9.a`                  |
| `1.0`          | `1.0`                    |
| `1.0.5.4-b.3`  | `1.0.5.4.b.3`            |

## Ordering
Versions have a total order, which is mainly determined by the order of numeric components. Alphabetic components are inferior to all numeric components, and lexicographic between themselves. When comparing Versions, missing trailing components are treated as zero.

### Example
The following versions are ordered from lower to higher. Equal versions are presented in the same line.
1. `0.9`
2. `1.0.a.2`
3. `1.0.b1`
4. `1.0`, `1`, `1.0.0.0`
6. `1.0.1`
