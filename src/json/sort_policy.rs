use crate::serde_helper::{invariable, string};
use anyhow::{anyhow, Error, Result};
use indexmap::IndexMap;
use serde::de;
use serde::ser::{self, Serialize, SerializeMap};
use serde_json::{ser::PrettyFormatter, Serializer};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, clap::Args)]
pub struct Args {
    files: Vec<PathBuf>,
}

pub fn main(args: Args) -> Result<()> {
    for file in &args.files {
        let f = File::open(file)?;
        let policy: Policy = serde_json::from_reader(f)?;

        let f = File::create(file)?;
        let mut serializer = Serializer::with_formatter(f, PrettyFormatter::with_indent(b"    "));
        policy.serialize(&mut serializer)?;

        let mut f = serializer.into_inner();
        f.write_all(b"\n")?;
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Arn {
    partition: String,
    service: String,
    region: String,
    account: String,
    resource: String,
}

impl FromStr for Arn {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("arn:") {
            let mut parts = s.splitn(6, ':');
            parts.next().unwrap(); // arn

            let partition = parts
                .next()
                .ok_or_else(|| anyhow!("missing partition part"))?;
            let service = parts
                .next()
                .ok_or_else(|| anyhow!("missing service part"))?;
            let region = parts.next().ok_or_else(|| anyhow!("missing region part"))?;
            let account = parts
                .next()
                .ok_or_else(|| anyhow!("missing account part"))?;
            let resource = parts
                .next()
                .ok_or_else(|| anyhow!("missing resource part"))?;

            Ok(Self {
                partition: partition.to_string(),
                service: service.to_string(),
                region: region.to_string(),
                account: account.to_string(),
                resource: resource.to_string(),
            })
        } else if s.len() == 12 && s.chars().all(|c| ('0'..='9').contains(&c)) {
            Ok(Self {
                partition: "aws".to_string(),
                service: "iam".to_string(),
                region: "".to_string(),
                account: s.to_string(),
                resource: "root".to_string(),
            })
        } else {
            Err(anyhow!("illegal ARN: `{s}`"))
        }
    }
}

impl fmt::Display for Arn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "arn:{}:{}:{}:{}:{}",
            self.partition, self.service, self.region, self.account, self.resource
        )
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct Policy {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "invariable::deserialize"
    )]
    statement: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Statement {
    sid: Option<String>,
    effect: Effect,
    principal: Option<Principal>,
    not_principal: Option<Principal>,
    action: BTreeSet<Action>,
    not_action: BTreeSet<Action>,
    resource: BTreeSet<Resource>,
    not_resource: BTreeSet<Resource>,
    condition: IndexMap<String, IndexMap<String, Vec<String>>>,
}

impl PartialOrd for Statement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match PartialOrd::partial_cmp(&self.sid, &other.sid) {
            Some(Ordering::Equal) => {}
            cmp => return cmp,
        };

        match PartialOrd::partial_cmp(&self.effect, &other.effect) {
            Some(Ordering::Equal) => {}
            cmp => return cmp,
        };

        match PartialOrd::partial_cmp(&self.principal, &other.principal) {
            Some(Ordering::Equal) => {}
            cmp => return cmp,
        };

        match PartialOrd::partial_cmp(&self.not_principal, &other.not_principal) {
            Some(Ordering::Equal) => {}
            cmp => return cmp,
        };

        match PartialOrd::partial_cmp(&self.action, &other.action) {
            Some(Ordering::Equal) => {}
            cmp => return cmp,
        };

        match PartialOrd::partial_cmp(&self.not_action, &other.not_action) {
            Some(Ordering::Equal) => {}
            cmp => return cmp,
        };

        match PartialOrd::partial_cmp(&self.resource, &other.resource) {
            Some(Ordering::Equal) => {}
            cmp => return cmp,
        };

        PartialOrd::partial_cmp(&self.not_resource, &other.not_resource)
    }
}

impl Ord for Statement {
    fn cmp(&self, other: &Self) -> Ordering {
        match Ord::cmp(&self.sid, &other.sid) {
            Ordering::Equal => {}
            cmp => return cmp,
        };

        match Ord::cmp(&self.effect, &other.effect) {
            Ordering::Equal => {}
            cmp => return cmp,
        };

        match Ord::cmp(&self.principal, &other.principal) {
            Ordering::Equal => {}
            cmp => return cmp,
        };

        match Ord::cmp(&self.not_principal, &other.not_principal) {
            Ordering::Equal => {}
            cmp => return cmp,
        };

        match Ord::cmp(&self.action, &other.action) {
            Ordering::Equal => {}
            cmp => return cmp,
        };

        match Ord::cmp(&self.not_action, &other.not_action) {
            Ordering::Equal => {}
            cmp => return cmp,
        };

        match Ord::cmp(&self.resource, &other.resource) {
            Ordering::Equal => {}
            cmp => return cmp,
        };

        Ord::cmp(&self.not_resource, &other.not_resource)
    }
}

impl<'de> de::Deserialize<'de> for Statement {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Statement;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                enum Field {
                    Sid,
                    Effect,
                    Principal,
                    NotPrincipal,
                    Action,
                    NotAction,
                    Resource,
                    NotResource,
                    Condition,
                }

                impl<'de> de::Deserialize<'de> for Field {
                    fn deserialize<D: de::Deserializer<'de>>(
                        deserializer: D,
                    ) -> Result<Self, D::Error> {
                        struct Visitor;

                        impl<'de> de::Visitor<'de> for Visitor {
                            type Value = Field;

                            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                                formatter.write_str("an identifier")
                            }

                            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                                match v {
                                    "Sid" => Ok(Field::Sid),
                                    "Effect" => Ok(Field::Effect),
                                    "Principal" => Ok(Field::Principal),
                                    "NotPrincipal" => Ok(Field::NotPrincipal),
                                    "Action" => Ok(Field::Action),
                                    "NotAction" => Ok(Field::NotAction),
                                    "Resource" => Ok(Field::Resource),
                                    "NotResource" => Ok(Field::NotResource),
                                    "Condition" => Ok(Field::Condition),
                                    field => Err(de::Error::unknown_field(
                                        field,
                                        &[
                                            "Sid",
                                            "Effect",
                                            "Principal",
                                            "NotPrincipal",
                                            "Action",
                                            "NotAction",
                                            "Resource",
                                            "NotResource",
                                            "Condition",
                                        ],
                                    )),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(Visitor)
                    }
                }

                let mut sid = None;
                let mut effect = None;
                let mut principal = None;
                let mut not_principal = None;
                let mut action = None;
                let mut not_action = None;
                let mut resource = None;
                let mut not_resource = None;
                let mut condition = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        Field::Sid => {
                            if sid.is_some() {
                                return Err(de::Error::duplicate_field("Sid"));
                            }
                            sid = map.next_value().map(Some)?;
                        }
                        Field::Effect => {
                            if effect.is_some() {
                                return Err(de::Error::duplicate_field("Effect"));
                            }
                            effect = map.next_value().map(string::unwrap).map(Some)?;
                        }
                        Field::Principal => {
                            if principal.is_some() {
                                return Err(de::Error::duplicate_field("Principal"));
                            }
                            principal = map.next_value().map(Some)?;
                        }
                        Field::NotPrincipal => {
                            if not_principal.is_some() {
                                return Err(de::Error::duplicate_field("NotPrincipal"));
                            }
                            not_principal = map.next_value().map(Some)?;
                        }
                        Field::Action => {
                            if action.is_some() {
                                return Err(de::Error::duplicate_field("Action"));
                            }
                            action = map
                                .next_value()
                                .map(invariable::unwrap)
                                .map(|v: Vec<_>| v.into_iter().map(string::unwrap).collect())
                                .map(Some)?;
                        }
                        Field::NotAction => {
                            if not_action.is_some() {
                                return Err(de::Error::duplicate_field("NotAction"));
                            }
                            not_action = map
                                .next_value()
                                .map(invariable::unwrap)
                                .map(|v: Vec<_>| v.into_iter().map(string::unwrap).collect())
                                .map(Some)?;
                        }
                        Field::Resource => {
                            if resource.is_some() {
                                return Err(de::Error::duplicate_field("Resource"));
                            }
                            resource = map
                                .next_value()
                                .map(invariable::unwrap)
                                .map(|v: Vec<_>| v.into_iter().map(string::unwrap).collect())
                                .map(Some)?;
                        }
                        Field::NotResource => {
                            if not_resource.is_some() {
                                return Err(de::Error::duplicate_field("NotResource"));
                            }
                            not_resource = map
                                .next_value()
                                .map(invariable::unwrap)
                                .map(|v: Vec<_>| v.into_iter().map(string::unwrap).collect())
                                .map(Some)?;
                        }
                        Field::Condition => {
                            if condition.is_some() {
                                return Err(de::Error::duplicate_field("Condition"));
                            }
                            condition = map.next_value().map(condition::unwrap).map(Some)?;
                        }
                    }
                }

                let effect = effect.ok_or_else(|| de::Error::missing_field("Effect"))?;
                let action = action.unwrap_or_default();
                let not_action = not_action.unwrap_or_default();
                let resource = resource.unwrap_or_default();
                let not_resource = not_resource.unwrap_or_default();
                let condition = condition.unwrap_or_default();

                Ok(Statement {
                    sid,
                    effect,
                    principal,
                    not_principal,
                    action,
                    not_action,
                    resource,
                    not_resource,
                    condition,
                })
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}

impl ser::Serialize for Statement {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;

        if let Some(sid) = &self.sid {
            map.serialize_entry("Sid", sid)?;
        }

        map.serialize_entry("Effect", &string::wrap(&self.effect))?;

        if let Some(principal) = &self.principal {
            map.serialize_entry("Principal", principal)?;
        }

        if let Some(not_principal) = &self.not_principal {
            map.serialize_entry("NotPrincipal", not_principal)?;
        }

        if !self.action.is_empty() {
            let action = self.action.iter().map(string::wrap).collect::<Vec<_>>();
            map.serialize_entry("Action", &invariable::wrap(&action))?;
        }

        if !self.not_action.is_empty() {
            let not_action = self.not_action.iter().map(string::wrap).collect::<Vec<_>>();
            map.serialize_entry("NotAction", &invariable::wrap(&not_action))?;
        }

        if !self.resource.is_empty() {
            let resource = self.resource.iter().map(string::wrap).collect::<Vec<_>>();
            map.serialize_entry("Resource", &invariable::wrap(&resource))?;
        }

        if !self.not_resource.is_empty() {
            let not_resource = self
                .not_resource
                .iter()
                .map(string::wrap)
                .collect::<Vec<_>>();
            map.serialize_entry("NotResource", &invariable::wrap(&not_resource))?;
        }

        if !self.condition.is_empty() {
            map.serialize_entry("Condition", &condition::wrap(&self.condition))?;
        }

        map.end()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Effect {
    Allow,
    Deny,
}

impl FromStr for Effect {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Allow" => Ok(Self::Allow),
            "Deny" => Ok(Self::Deny),
            _ => Err(anyhow!("illegal value `{s}`, expected `Allow` or `Deny`")),
        }
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allow => f.write_str("Allow"),
            Self::Deny => f.write_str("Deny"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Principal {
    aws: BTreeSet<Arn>,
    canonical_user: BTreeSet<String>,
    federated: BTreeSet<String>,
    service: BTreeSet<String>,
}

impl<'de> de::Deserialize<'de> for Principal {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Principal;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                enum Field {
                    Aws,
                    CanonicalUser,
                    Federated,
                    Service,
                }

                impl<'de> de::Deserialize<'de> for Field {
                    fn deserialize<D: de::Deserializer<'de>>(
                        deserializer: D,
                    ) -> Result<Self, D::Error> {
                        struct Visitor;

                        impl<'de> de::Visitor<'de> for Visitor {
                            type Value = Field;

                            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                                formatter.write_str("an identifier")
                            }

                            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                                match v {
                                    "AWS" => Ok(Field::Aws),
                                    "CanonicalUser" => Ok(Field::CanonicalUser),
                                    "Federated" => Ok(Field::Federated),
                                    "Service" => Ok(Field::Service),
                                    field => Err(de::Error::unknown_field(
                                        field,
                                        &["AWS", "CanonicalUser", "Federated", "Service"],
                                    )),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(Visitor)
                    }
                }

                let mut aws = None;
                let mut canonical_user = None;
                let mut federated = None;
                let mut service = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        Field::Aws => {
                            if aws.is_some() {
                                return Err(de::Error::duplicate_field("AWS"));
                            }
                            aws = map
                                .next_value()
                                .map(invariable::unwrap)
                                .map(|v: Vec<_>| v.into_iter().map(string::unwrap).collect())
                                .map(Some)?;
                        }
                        Field::CanonicalUser => {
                            if canonical_user.is_some() {
                                return Err(de::Error::duplicate_field("CanonicalUser"));
                            }
                            canonical_user = map.next_value().map(invariable::unwrap).map(Some)?;
                        }
                        Field::Federated => {
                            if federated.is_some() {
                                return Err(de::Error::duplicate_field("Federated"));
                            }
                            federated = map.next_value().map(invariable::unwrap).map(Some)?;
                        }
                        Field::Service => {
                            if service.is_some() {
                                return Err(de::Error::duplicate_field("Service"));
                            }
                            service = map.next_value().map(invariable::unwrap).map(Some)?;
                        }
                    }
                }

                let aws = aws.unwrap_or_default();
                let canonical_user = canonical_user.unwrap_or_default();
                let federated = federated.unwrap_or_default();
                let service = service.unwrap_or_default();

                Ok(Principal {
                    aws,
                    canonical_user,
                    federated,
                    service,
                })
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}

impl ser::Serialize for Principal {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;

        if !self.aws.is_empty() {
            let aws = self.aws.iter().map(string::wrap).collect::<Vec<_>>();
            map.serialize_entry("Aws", &invariable::wrap(&aws))?;
        }

        if !self.canonical_user.is_empty() {
            map.serialize_entry("CanonicalUser", &invariable::wrap(&self.canonical_user))?;
        }

        if !self.federated.is_empty() {
            map.serialize_entry("Federated", &invariable::wrap(&self.federated))?;
        }

        if !self.service.is_empty() {
            map.serialize_entry("Service", &invariable::wrap(&self.service))?;
        }

        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    Any,
    Name(String, String),
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::Any),
            _ => {
                if let Some((service, name)) = s.split_once(':') {
                    Ok(Self::Name(service.to_string(), name.to_string()))
                } else {
                    Err(anyhow!("illegal action name: `{s}`"))
                }
            }
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Any => f.write_str("*"),
            Action::Name(service, name) => write!(f, "{service}:{name}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Resource {
    Any,
    Arn(Arn),
}

impl FromStr for Resource {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::Any),
            _ => s.parse().map(Self::Arn),
        }
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Any => f.write_str("*"),
            Self::Arn(arn) => arn.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_arn() {
        assert_eq!(
            "arn:aws:sns:us-east-1:123456789012:example-sns-topic-name"
                .parse::<Arn>()
                .unwrap(),
            Arn {
                partition: "aws".to_string(),
                service: "sns".to_string(),
                region: "us-east-1".to_string(),
                account: "123456789012".to_string(),
                resource: "example-sns-topic-name".to_string(),
            }
        );
        assert_eq!(
            "123456789012".parse::<Arn>().unwrap(),
            Arn {
                partition: "aws".to_string(),
                service: "iam".to_string(),
                region: "".to_string(),
                account: "123456789012".to_string(),
                resource: "root".to_string(),
            }
        );

        assert_eq!(
            "hoge".parse::<Arn>().unwrap_err().to_string(),
            "illegal ARN: `hoge`"
        );
        assert_eq!(
            "12345678901A".parse::<Arn>().unwrap_err().to_string(),
            "illegal ARN: `12345678901A`"
        );
    }

    #[test]
    fn test_serde_effect() {
        assert_eq!("Allow".parse::<Effect>().unwrap(), Effect::Allow);
        assert_eq!("Deny".parse::<Effect>().unwrap(), Effect::Deny);

        assert_eq!(
            "hoge".parse::<Effect>().unwrap_err().to_string(),
            "illegal value `hoge`, expected `Allow` or `Deny`",
        );
    }

    #[test]
    fn test_serde_action() {
        assert_eq!("*".parse::<Action>().unwrap(), Action::Any);
        assert_eq!(
            "s3:GetObject".parse::<Action>().unwrap(),
            Action::Name("s3".to_string(), "GetObject".to_string()),
        );

        assert_eq!(
            "hoge".parse::<Action>().unwrap_err().to_string(),
            "illegal action name: `hoge`"
        );
    }

    #[test]
    fn test_serde_resource() {
        assert_eq!("*".parse::<Resource>().unwrap(), Resource::Any);
        assert_eq!(
            "arn:aws:iam::123456789012:user/johndoe"
                .parse::<Resource>()
                .unwrap(),
            Resource::Arn(Arn {
                partition: "aws".to_string(),
                service: "iam".to_string(),
                region: "".to_string(),
                account: "123456789012".to_string(),
                resource: "user/johndoe".to_string(),
            }),
        );

        assert_eq!(
            "12345678901A".parse::<Resource>().unwrap_err().to_string(),
            "illegal ARN: `12345678901A`",
        );
    }
}

mod condition {
    use super::*;

    pub fn unwrap(d: Deserialize) -> IndexMap<String, IndexMap<String, Vec<String>>> {
        d.0
    }

    pub struct Deserialize(IndexMap<String, IndexMap<String, Vec<String>>>);

    impl<'de> de::Deserialize<'de> for Deserialize {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            struct Visitor;

            impl<'de> de::Visitor<'de> for Visitor {
                type Value = IndexMap<String, IndexMap<String, Vec<String>>>;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a map")
                }

                fn visit_map<A: de::MapAccess<'de>>(
                    self,
                    mut map: A,
                ) -> Result<Self::Value, A::Error> {
                    let mut m = IndexMap::new();

                    while let Some(key) = map.next_key()? {
                        let value = map.next_value().map(operator::unwrap)?;
                        m.insert(key, value);
                    }

                    Ok(m)
                }
            }

            deserializer.deserialize_map(Visitor).map(Deserialize)
        }
    }

    pub fn wrap(inner: &IndexMap<String, IndexMap<String, Vec<String>>>) -> Serialize {
        Serialize(inner)
    }

    pub struct Serialize<'a>(&'a IndexMap<String, IndexMap<String, Vec<String>>>);

    impl ser::Serialize for Serialize<'_> {
        fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut map = serializer.serialize_map(Some(self.0.len()))?;

            for (key, value) in self.0 {
                map.serialize_entry(key, &operator::wrap(value))?;
            }

            map.end()
        }
    }

    mod operator {
        use super::*;

        pub fn unwrap(d: Deserialize) -> IndexMap<String, Vec<String>> {
            d.0
        }

        pub struct Deserialize(IndexMap<String, Vec<String>>);

        impl<'de> de::Deserialize<'de> for Deserialize {
            fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct Visitor;

                impl<'de> de::Visitor<'de> for Visitor {
                    type Value = IndexMap<String, Vec<String>>;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a map")
                    }

                    fn visit_map<A: de::MapAccess<'de>>(
                        self,
                        mut map: A,
                    ) -> Result<Self::Value, A::Error> {
                        let mut m = IndexMap::new();

                        while let Some(key) = map.next_key()? {
                            let value = map.next_value().map(invariable::unwrap)?;
                            m.insert(key, value);
                        }

                        Ok(m)
                    }
                }

                deserializer.deserialize_map(Visitor).map(Deserialize)
            }
        }

        pub fn wrap(inner: &IndexMap<String, Vec<String>>) -> Serialize {
            Serialize(inner)
        }

        pub struct Serialize<'a>(&'a IndexMap<String, Vec<String>>);

        impl ser::Serialize for Serialize<'_> {
            fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let mut map = serializer.serialize_map(Some(self.0.len()))?;

                for (key, value) in self.0 {
                    map.serialize_entry(key, &invariable::wrap(value))?;
                }

                map.end()
            }
        }
    }
}
