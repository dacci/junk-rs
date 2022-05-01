use anyhow::{anyhow, Error, Result};
use serde::{de, ser, Serialize};
use serde_json::{ser::PrettyFormatter, Serializer, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, clap::Args)]
pub struct Args {
    path: PathBuf,
}

pub fn main(args: &Args) -> Result<()> {
    let f = File::open(&args.path)?;
    let config: BTreeMap<Key, Value> = serde_json::from_reader(f)?;

    let mut f = File::create(&args.path)?;
    let mut s = Serializer::with_formatter(&f, PrettyFormatter::with_indent(b"    "));
    config.serialize(&mut s)?;
    f.write_all(b"\n")?;

    Ok(())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Key {
    Config(String),
    Lang(BTreeSet<String>),
}

impl std::str::FromStr for Key {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            let mut set = BTreeSet::new();

            let mut s = s;
            while let Some(from) = s.find('[') {
                let to = s
                    .find(']')
                    .ok_or_else(|| anyhow!("unmatched opening bracket"))?;
                set.insert(s[(from + 1)..to].to_string());
                s = &s[(to + 1)..];
            }

            Ok(Key::Lang(set))
        } else {
            Ok(Key::Config(s.to_string()))
        }
    }
}

impl<'de> de::Deserialize<'de> for Key {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Key;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a string")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse().map_err(E::custom)
            }
        }

        d.deserialize_str(Visitor)
    }
}

impl ser::Serialize for Key {
    fn serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Key::Config(key) => s.serialize_str(key),
            Key::Lang(set) => s.serialize_str(&format!(
                "[{}]",
                set.iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .join("][")
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens_error, assert_tokens, Token};

    #[test]
    fn serde() {
        assert_tokens(&Key::Config("hoge".to_string()), &[Token::String("hoge")]);
        assert_tokens(
            &Key::Lang(BTreeSet::from(["piyo".to_string(), "hoge".to_string()])),
            &[Token::String("[hoge][piyo]")],
        );

        assert_de_tokens_error::<Key>(
            &[Token::I32(42)],
            "invalid type: integer `42`, expected a string",
        );
        assert_de_tokens_error::<Key>(&[Token::String("[hoge")], "unmatched opening bracket");
    }
}
