// Generated from definition com.github.openshift.api.apps.v1.DeploymentConfigRollbackSpec

/// DeploymentConfigRollbackSpec represents the options for rollback generation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentConfigRollbackSpec {
    /// From points to a ReplicationController which is a deployment.
    pub from: k8s_openapi::api::core::v1::ObjectReference,

    /// IncludeReplicationMeta specifies whether to include the replica count and selector.
    pub include_replication_meta: bool,

    /// IncludeStrategy specifies whether to include the deployment Strategy.
    pub include_strategy: bool,

    /// IncludeTemplate specifies whether to include the PodTemplateSpec.
    pub include_template: bool,

    /// IncludeTriggers specifies whether to include config Triggers.
    pub include_triggers: bool,

    /// Revision to rollback to. If set to 0, rollback to the last revision.
    pub revision: Option<i64>,
}

impl<'de> serde::Deserialize<'de> for DeploymentConfigRollbackSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_from,
            Key_include_replication_meta,
            Key_include_strategy,
            Key_include_template,
            Key_include_triggers,
            Key_revision,
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
                            "from" => Field::Key_from,
                            "includeReplicationMeta" => Field::Key_include_replication_meta,
                            "includeStrategy" => Field::Key_include_strategy,
                            "includeTemplate" => Field::Key_include_template,
                            "includeTriggers" => Field::Key_include_triggers,
                            "revision" => Field::Key_revision,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentConfigRollbackSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DeploymentConfigRollbackSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_from: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_include_replication_meta: Option<bool> = None;
                let mut value_include_strategy: Option<bool> = None;
                let mut value_include_template: Option<bool> = None;
                let mut value_include_triggers: Option<bool> = None;
                let mut value_revision: Option<i64> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_from => value_from = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_include_replication_meta => value_include_replication_meta = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_include_strategy => value_include_strategy = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_include_template => value_include_template = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_include_triggers => value_include_triggers = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_revision => value_revision = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentConfigRollbackSpec {
                    from: value_from.ok_or_else(|| serde::de::Error::missing_field("from"))?,
                    include_replication_meta: value_include_replication_meta.ok_or_else(|| serde::de::Error::missing_field("includeReplicationMeta"))?,
                    include_strategy: value_include_strategy.ok_or_else(|| serde::de::Error::missing_field("includeStrategy"))?,
                    include_template: value_include_template.ok_or_else(|| serde::de::Error::missing_field("includeTemplate"))?,
                    include_triggers: value_include_triggers.ok_or_else(|| serde::de::Error::missing_field("includeTriggers"))?,
                    revision: value_revision,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeploymentConfigRollbackSpec",
            &[
                "from",
                "includeReplicationMeta",
                "includeStrategy",
                "includeTemplate",
                "includeTriggers",
                "revision",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DeploymentConfigRollbackSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentConfigRollbackSpec",
            5 +
            self.revision.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "from", &self.from)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "includeReplicationMeta", &self.include_replication_meta)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "includeStrategy", &self.include_strategy)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "includeTemplate", &self.include_template)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "includeTriggers", &self.include_triggers)?;
        if let Some(value) = &self.revision {
            serde::ser::SerializeStruct::serialize_field(&mut state, "revision", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
