pub mod invariable {
    use serde::de::{value::*, Deserialize, Deserializer, Error, MapAccess, SeqAccess, Visitor};
    use serde::ser::{Serialize, Serializer};
    use std::fmt;
    use std::marker::PhantomData;

    pub fn deserialize<'de, D, B, T>(deserializer: D) -> Result<B, D::Error>
    where
        D: Deserializer<'de>,
        B: FromIterator<T>,
        T: Deserialize<'de>,
    {
        struct VisitorImpl<B, T>(PhantomData<fn() -> (B, T)>);

        impl<'de, B, T> Visitor<'de> for VisitorImpl<B, T>
        where
            B: FromIterator<T>,
            T: Deserialize<'de>,
        {
            type Value = B;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("deserializable data")
            }

            fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
                Deserialize::deserialize(BoolDeserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_i8<E: Error>(self, v: i8) -> Result<Self::Value, E> {
                Deserialize::deserialize(I8Deserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_i16<E: Error>(self, v: i16) -> Result<Self::Value, E> {
                Deserialize::deserialize(I16Deserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_i32<E: Error>(self, v: i32) -> Result<Self::Value, E> {
                Deserialize::deserialize(I32Deserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
                Deserialize::deserialize(I64Deserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_i128<E: Error>(self, v: i128) -> Result<Self::Value, E> {
                Deserialize::deserialize(I128Deserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_u8<E: Error>(self, v: u8) -> Result<Self::Value, E> {
                Deserialize::deserialize(U8Deserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_u16<E: Error>(self, v: u16) -> Result<Self::Value, E> {
                Deserialize::deserialize(U16Deserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_u32<E: Error>(self, v: u32) -> Result<Self::Value, E> {
                Deserialize::deserialize(U32Deserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
                Deserialize::deserialize(U64Deserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_u128<E: Error>(self, v: u128) -> Result<Self::Value, E> {
                Deserialize::deserialize(U128Deserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_f32<E: Error>(self, v: f32) -> Result<Self::Value, E> {
                Deserialize::deserialize(F32Deserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
                Deserialize::deserialize(F64Deserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_char<E: Error>(self, v: char) -> Result<Self::Value, E> {
                Deserialize::deserialize(CharDeserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                Deserialize::deserialize(StrDeserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_borrowed_str<E: Error>(self, v: &'de str) -> Result<Self::Value, E> {
                Deserialize::deserialize(BorrowedStrDeserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
                Deserialize::deserialize(StringDeserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                Deserialize::deserialize(BytesDeserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_borrowed_bytes<E: Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
                Deserialize::deserialize(BorrowedBytesDeserializer::new(v))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }

            fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
                Deserialize::deserialize(SeqAccessDeserializer::new(seq))
                    .map(|v: Vec<_>| FromIterator::from_iter(v))
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                Deserialize::deserialize(MapAccessDeserializer::new(map))
                    .map(Some)
                    .map(FromIterator::from_iter)
            }
        }

        deserializer.deserialize_any(VisitorImpl(PhantomData))
    }

    pub fn serialize<'a, T, S>(v: &'a T, serializer: S) -> Result<S::Ok, S::Error>
    where
        &'a T: IntoIterator,
        <&'a T as IntoIterator>::Item: Serialize,
        S: Serializer,
    {
        let v = v.into_iter().collect::<Vec<_>>();
        if v.len() == 1 {
            Serialize::serialize(&v[0], serializer)
        } else {
            serializer.collect_seq(v)
        }
    }

    pub fn unwrap<B, T>(d: Wrapper<B, T>) -> B {
        d.0
    }

    pub fn wrap<T>(inner: &T) -> Wrapper<&T, ()> {
        Wrapper(inner, PhantomData)
    }

    pub struct Wrapper<B, T>(B, PhantomData<fn() -> T>);

    impl<'de, B, T> Deserialize<'de> for Wrapper<B, T>
    where
        B: FromIterator<T>,
        T: Deserialize<'de>,
    {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserialize(deserializer).map(|b| Self(b, PhantomData))
        }
    }

    impl<'a, T> Serialize for Wrapper<&'a T, ()>
    where
        &'a T: IntoIterator,
        <&'a T as IntoIterator>::Item: Serialize,
    {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serialize(self.0, serializer)
        }
    }

    #[cfg(test)]
    mod tests {
        use serde_test::*;

        #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
        struct Test {
            #[serde(with = "super")]
            values: Vec<u8>,
        }

        #[test]
        fn test() {
            assert_tokens(
                &Test { values: vec![1] },
                &[
                    Token::Struct {
                        name: "Test",
                        len: 1,
                    },
                    Token::Str("values"),
                    Token::U8(1),
                    Token::StructEnd,
                ],
            );
            assert_tokens(
                &Test { values: vec![1, 2] },
                &[
                    Token::Struct {
                        name: "Test",
                        len: 1,
                    },
                    Token::Str("values"),
                    Token::Seq { len: Some(2) },
                    Token::U8(1),
                    Token::U8(2),
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );
            assert_tokens(
                &Test { values: vec![] },
                &[
                    Token::Struct {
                        name: "Test",
                        len: 1,
                    },
                    Token::Str("values"),
                    Token::Seq { len: Some(0) },
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );
        }
    }
}

pub mod string {
    use serde::de::{Deserialize, Deserializer, Error, Visitor};
    use serde::ser::{Serialize, Serializer};
    use std::fmt;
    use std::marker::PhantomData;
    use std::str::FromStr;

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        T::Err: fmt::Display,
    {
        struct VisitorImpl<T>(PhantomData<fn() -> T>);

        impl<'de, T> Visitor<'de> for VisitorImpl<T>
        where
            T: FromStr,
            T::Err: fmt::Display,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse().map_err(Error::custom)
            }
        }

        deserializer.deserialize_str(VisitorImpl(PhantomData))
    }

    pub fn serialize<T: ToString, S: Serializer>(v: &T, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&v.to_string())
    }

    pub fn unwrap<T>(d: Wrapper<T>) -> T {
        d.0
    }

    pub fn wrap<T: ToString>(inner: &T) -> Wrapper<&T> {
        Wrapper(inner)
    }

    pub struct Wrapper<T>(T);

    impl<'de, T> Deserialize<'de> for Wrapper<T>
    where
        T: FromStr,
        T::Err: fmt::Display,
    {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserialize(deserializer).map(Self)
        }
    }

    impl<T: ToString> Serialize for Wrapper<&T> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serialize(self.0, serializer)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use anyhow::{anyhow, Error};
        use serde_test::*;

        #[derive(Debug, PartialEq)]
        struct Value;

        impl FromStr for Value {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s == "Value" {
                    Ok(Value)
                } else {
                    Err(anyhow!("{s}"))
                }
            }
        }

        impl fmt::Display for Value {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("Value")
            }
        }

        #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
        struct Test {
            #[serde(with = "super")]
            value: Value,
        }

        #[test]
        fn test() {
            assert_tokens(
                &Test { value: Value },
                &[
                    Token::Struct {
                        name: "Test",
                        len: 1,
                    },
                    Token::Str("value"),
                    Token::Str("Value"),
                    Token::StructEnd,
                ],
            );

            assert_de_tokens_error::<Test>(
                &[
                    Token::Struct {
                        name: "Test",
                        len: 1,
                    },
                    Token::Str("value"),
                    Token::Str("Hoge"),
                ],
                "Hoge",
            );
        }
    }
}
