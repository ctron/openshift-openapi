// Generated from definition com.github.openshift.api.authorization.v1.SubjectRulesReviewStatus

/// SubjectRulesReviewStatus is contains the result of a rules check
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubjectRulesReviewStatus {
    /// EvaluationError can appear in combination with Rules.  It means some error happened during evaluation that may have prevented additional rules from being populated.
    pub evaluation_error: Option<String>,

    /// Rules is the list of rules (no particular sort) that are allowed for the subject
    pub rules: Vec<crate::api::authorization::v1::PolicyRule>,
}

impl<'de> serde::Deserialize<'de> for SubjectRulesReviewStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_evaluation_error,
            Key_rules,
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
                            "evaluationError" => Field::Key_evaluation_error,
                            "rules" => Field::Key_rules,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubjectRulesReviewStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SubjectRulesReviewStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_evaluation_error: Option<String> = None;
                let mut value_rules: Option<Vec<crate::api::authorization::v1::PolicyRule>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_evaluation_error => value_evaluation_error = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rules => value_rules = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SubjectRulesReviewStatus {
                    evaluation_error: value_evaluation_error,
                    rules: value_rules.ok_or_else(|| serde::de::Error::missing_field("rules"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "SubjectRulesReviewStatus",
            &[
                "evaluationError",
                "rules",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SubjectRulesReviewStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SubjectRulesReviewStatus",
            1 +
            self.evaluation_error.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.evaluation_error {
            serde::ser::SerializeStruct::serialize_field(&mut state, "evaluationError", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "rules", &self.rules)?;
        serde::ser::SerializeStruct::end(state)
    }
}
