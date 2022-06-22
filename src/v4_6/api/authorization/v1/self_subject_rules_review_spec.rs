// Generated from definition com.github.openshift.api.authorization.v1.SelfSubjectRulesReviewSpec

/// SelfSubjectRulesReviewSpec adds information about how to conduct the check
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelfSubjectRulesReviewSpec {
    /// Scopes to use for the evaluation.  Empty means "use the unscoped (full) permissions of the user/groups". Nil means "use the scopes on this request".
    pub scopes: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for SelfSubjectRulesReviewSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_scopes,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "scopes" => Field::Key_scopes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SelfSubjectRulesReviewSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SelfSubjectRulesReviewSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_scopes: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_scopes => value_scopes = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SelfSubjectRulesReviewSpec {
                    scopes: value_scopes.ok_or_else(|| serde::de::Error::missing_field("scopes"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "SelfSubjectRulesReviewSpec",
            &[
                "scopes",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SelfSubjectRulesReviewSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SelfSubjectRulesReviewSpec",
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "scopes", &self.scopes)?;
        serde::ser::SerializeStruct::end(state)
    }
}
