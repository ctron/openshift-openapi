// Generated from definition com.github.openshift.api.apps.v1.DeploymentConfigStatus

/// DeploymentConfigStatus represents the current deployment state.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentConfigStatus {
    /// AvailableReplicas is the total number of available pods targeted by this deployment config.
    pub available_replicas: i32,

    /// Conditions represents the latest available observations of a deployment config's current state.
    pub conditions: Option<Vec<crate::api::apps::v1::DeploymentCondition>>,

    /// Details are the reasons for the update to this deployment config. This could be based on a change made by the user or caused by an automatic trigger
    pub details: Option<crate::api::apps::v1::DeploymentDetails>,

    /// LatestVersion is used to determine whether the current deployment associated with a deployment config is out of sync.
    pub latest_version: i64,

    /// ObservedGeneration is the most recent generation observed by the deployment config controller.
    pub observed_generation: i64,

    /// Total number of ready pods targeted by this deployment.
    pub ready_replicas: Option<i32>,

    /// Replicas is the total number of pods targeted by this deployment config.
    pub replicas: i32,

    /// UnavailableReplicas is the total number of unavailable pods targeted by this deployment config.
    pub unavailable_replicas: i32,

    /// UpdatedReplicas is the total number of non-terminated pods targeted by this deployment config that have the desired template spec.
    pub updated_replicas: i32,
}

impl<'de> serde::Deserialize<'de> for DeploymentConfigStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_available_replicas,
            Key_conditions,
            Key_details,
            Key_latest_version,
            Key_observed_generation,
            Key_ready_replicas,
            Key_replicas,
            Key_unavailable_replicas,
            Key_updated_replicas,
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
                            "availableReplicas" => Field::Key_available_replicas,
                            "conditions" => Field::Key_conditions,
                            "details" => Field::Key_details,
                            "latestVersion" => Field::Key_latest_version,
                            "observedGeneration" => Field::Key_observed_generation,
                            "readyReplicas" => Field::Key_ready_replicas,
                            "replicas" => Field::Key_replicas,
                            "unavailableReplicas" => Field::Key_unavailable_replicas,
                            "updatedReplicas" => Field::Key_updated_replicas,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentConfigStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DeploymentConfigStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_available_replicas: Option<i32> = None;
                let mut value_conditions: Option<Vec<crate::api::apps::v1::DeploymentCondition>> = None;
                let mut value_details: Option<crate::api::apps::v1::DeploymentDetails> = None;
                let mut value_latest_version: Option<i64> = None;
                let mut value_observed_generation: Option<i64> = None;
                let mut value_ready_replicas: Option<i32> = None;
                let mut value_replicas: Option<i32> = None;
                let mut value_unavailable_replicas: Option<i32> = None;
                let mut value_updated_replicas: Option<i32> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_available_replicas => value_available_replicas = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_conditions => value_conditions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_details => value_details = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_latest_version => value_latest_version = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_observed_generation => value_observed_generation = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_ready_replicas => value_ready_replicas = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_unavailable_replicas => value_unavailable_replicas = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_updated_replicas => value_updated_replicas = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentConfigStatus {
                    available_replicas: value_available_replicas.ok_or_else(|| serde::de::Error::missing_field("availableReplicas"))?,
                    conditions: value_conditions,
                    details: value_details,
                    latest_version: value_latest_version.ok_or_else(|| serde::de::Error::missing_field("latestVersion"))?,
                    observed_generation: value_observed_generation.ok_or_else(|| serde::de::Error::missing_field("observedGeneration"))?,
                    ready_replicas: value_ready_replicas,
                    replicas: value_replicas.ok_or_else(|| serde::de::Error::missing_field("replicas"))?,
                    unavailable_replicas: value_unavailable_replicas.ok_or_else(|| serde::de::Error::missing_field("unavailableReplicas"))?,
                    updated_replicas: value_updated_replicas.ok_or_else(|| serde::de::Error::missing_field("updatedReplicas"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeploymentConfigStatus",
            &[
                "availableReplicas",
                "conditions",
                "details",
                "latestVersion",
                "observedGeneration",
                "readyReplicas",
                "replicas",
                "unavailableReplicas",
                "updatedReplicas",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DeploymentConfigStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentConfigStatus",
            6 +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.details.as_ref().map_or(0, |_| 1) +
            self.ready_replicas.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "availableReplicas", &self.available_replicas)?;
        if let Some(value) = &self.conditions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.details {
            serde::ser::SerializeStruct::serialize_field(&mut state, "details", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "latestVersion", &self.latest_version)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", &self.observed_generation)?;
        if let Some(value) = &self.ready_replicas {
            serde::ser::SerializeStruct::serialize_field(&mut state, "readyReplicas", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", &self.replicas)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "unavailableReplicas", &self.unavailable_replicas)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "updatedReplicas", &self.updated_replicas)?;
        serde::ser::SerializeStruct::end(state)
    }
}
