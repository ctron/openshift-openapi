// Generated from definition com.github.openshift.api.authorization.v1.PolicyRule

/// PolicyRule holds information that describes a policy rule, but does not contain information about who the rule applies to or which namespace the rule applies to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PolicyRule {
    /// APIGroups is the name of the APIGroup that contains the resources.  If this field is empty, then both kubernetes and origin API groups are assumed. That means that if an action is requested against one of the enumerated resources in either the kubernetes or the origin API group, the request will be allowed
    pub api_groups: Vec<String>,

    /// AttributeRestrictions will vary depending on what the Authorizer/AuthorizationAttributeBuilder pair supports. If the Authorizer does not recognize how to handle the AttributeRestrictions, the Authorizer should report an error.
    pub attribute_restrictions: Option<k8s_openapi::apimachinery::pkg::runtime::RawExtension>,

    /// NonResourceURLsSlice is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path This name is intentionally different than the internal type so that the DefaultConvert works nicely and because the ordering may be different.
    pub non_resource_urls: Option<Vec<String>>,

    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
    pub resource_names: Option<Vec<String>>,

    /// Resources is a list of resources this rule applies to.  ResourceAll represents all resources.
    pub resources: Vec<String>,

    /// Verbs is a list of Verbs that apply to ALL the ResourceKinds and AttributeRestrictions contained in this rule.  VerbAll represents all kinds.
    pub verbs: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for PolicyRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_groups,
            Key_attribute_restrictions,
            Key_non_resource_urls,
            Key_resource_names,
            Key_resources,
            Key_verbs,
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
                            "apiGroups" => Field::Key_api_groups,
                            "attributeRestrictions" => Field::Key_attribute_restrictions,
                            "nonResourceURLs" => Field::Key_non_resource_urls,
                            "resourceNames" => Field::Key_resource_names,
                            "resources" => Field::Key_resources,
                            "verbs" => Field::Key_verbs,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PolicyRule;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PolicyRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_api_groups: Option<Vec<String>> = None;
                let mut value_attribute_restrictions: Option<k8s_openapi::apimachinery::pkg::runtime::RawExtension> = None;
                let mut value_non_resource_urls: Option<Vec<String>> = None;
                let mut value_resource_names: Option<Vec<String>> = None;
                let mut value_resources: Option<Vec<String>> = None;
                let mut value_verbs: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_groups => value_api_groups = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_attribute_restrictions => value_attribute_restrictions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_non_resource_urls => value_non_resource_urls = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_names => value_resource_names = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_verbs => value_verbs = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PolicyRule {
                    api_groups: value_api_groups.ok_or_else(|| serde::de::Error::missing_field("apiGroups"))?,
                    attribute_restrictions: value_attribute_restrictions,
                    non_resource_urls: value_non_resource_urls,
                    resource_names: value_resource_names,
                    resources: value_resources.ok_or_else(|| serde::de::Error::missing_field("resources"))?,
                    verbs: value_verbs.ok_or_else(|| serde::de::Error::missing_field("verbs"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PolicyRule",
            &[
                "apiGroups",
                "attributeRestrictions",
                "nonResourceURLs",
                "resourceNames",
                "resources",
                "verbs",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PolicyRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PolicyRule",
            3 +
            self.attribute_restrictions.as_ref().map_or(0, |_| 1) +
            self.non_resource_urls.as_ref().map_or(0, |_| 1) +
            self.resource_names.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroups", &self.api_groups)?;
        if let Some(value) = &self.attribute_restrictions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "attributeRestrictions", value)?;
        }
        if let Some(value) = &self.non_resource_urls {
            serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceURLs", value)?;
        }
        if let Some(value) = &self.resource_names {
            serde::ser::SerializeStruct::serialize_field(&mut state, "resourceNames", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "resources", &self.resources)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        serde::ser::SerializeStruct::end(state)
    }
}
