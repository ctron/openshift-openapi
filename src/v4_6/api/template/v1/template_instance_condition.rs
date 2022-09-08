// Generated from definition com.github.openshift.api.template.v1.TemplateInstanceCondition

/// TemplateInstanceCondition contains condition information for a TemplateInstance.
#[derive(Clone, Debug, PartialEq)]
pub struct TemplateInstanceCondition {
    /// LastTransitionTime is the last time a condition status transitioned from one state to another.
    pub last_transition_time: k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,

    /// Message is a human readable description of the details of the last transition, complementing reason.
    pub message: String,

    /// Reason is a brief machine readable explanation for the condition's last transition.
    pub reason: String,

    /// Status of the condition, one of True, False or Unknown.
    pub status: String,

    /// Type of the condition, currently Ready or InstantiateFailure.
    pub type_: String,
}

impl<'de> serde::Deserialize<'de> for TemplateInstanceCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_last_transition_time,
            Key_message,
            Key_reason,
            Key_status,
            Key_type_,
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
                            "lastTransitionTime" => Field::Key_last_transition_time,
                            "message" => Field::Key_message,
                            "reason" => Field::Key_reason,
                            "status" => Field::Key_status,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TemplateInstanceCondition;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TemplateInstanceCondition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_last_transition_time: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_message: Option<String> = None;
                let mut value_reason: Option<String> = None;
                let mut value_status: Option<String> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_last_transition_time => value_last_transition_time = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_message => value_message = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_reason => value_reason = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_status => value_status = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_type_ => value_type_ = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TemplateInstanceCondition {
                    last_transition_time: value_last_transition_time.ok_or_else(|| serde::de::Error::missing_field("lastTransitionTime"))?,
                    message: value_message.ok_or_else(|| serde::de::Error::missing_field("message"))?,
                    reason: value_reason.ok_or_else(|| serde::de::Error::missing_field("reason"))?,
                    status: value_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    type_: value_type_.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "TemplateInstanceCondition",
            &[
                "lastTransitionTime",
                "message",
                "reason",
                "status",
                "type",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for TemplateInstanceCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TemplateInstanceCondition",
            5,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "lastTransitionTime", &self.last_transition_time)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "message", &self.message)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "reason", &self.reason)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "status", &self.status)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        serde::ser::SerializeStruct::end(state)
    }
}
