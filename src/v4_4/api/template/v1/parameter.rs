// Generated from definition com.github.openshift.api.template.v1.Parameter

/// Parameter defines a name/value variable that is to be processed during the Template to Config transformation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Parameter {
    /// Description of a parameter. Optional.
    pub description: Option<String>,

    /// Optional: The name that will show in UI instead of parameter 'Name'
    pub display_name: Option<String>,

    /// From is an input value for the generator. Optional.
    pub from: Option<String>,

    /// generate specifies the generator to be used to generate random string from an input value specified by From field. The result string is stored into Value field. If empty, no generator is being used, leaving the result Value untouched. Optional.
    ///
    /// The only supported generator is "expression", which accepts a "from" value in the form of a simple regular expression containing the range expression "\[a-zA-Z0-9\]", and the length expression "a{length}".
    ///
    /// Examples:
    ///
    /// from             | value ----------------------------- "test\[0-9\]{1}x"  | "test7x" "\[0-1\]{8}"       | "01001100" "0x\[A-F0-9\]{4}"  | "0xB3AF" "\[a-zA-Z0-9\]{8}" | "hW4yQU5i"
    pub generate: Option<String>,

    /// Name must be set and it can be referenced in Template Items using ${PARAMETER_NAME}. Required.
    pub name: String,

    /// Optional: Indicates the parameter must have a value.  Defaults to false.
    pub required: Option<bool>,

    /// Value holds the Parameter data. If specified, the generator will be ignored. The value replaces all occurrences of the Parameter ${Name} expression during the Template to Config transformation. Optional.
    pub value: Option<String>,
}

impl<'de> serde::Deserialize<'de> for Parameter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_description,
            Key_display_name,
            Key_from,
            Key_generate,
            Key_name,
            Key_required,
            Key_value,
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
                            "description" => Field::Key_description,
                            "displayName" => Field::Key_display_name,
                            "from" => Field::Key_from,
                            "generate" => Field::Key_generate,
                            "name" => Field::Key_name,
                            "required" => Field::Key_required,
                            "value" => Field::Key_value,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Parameter;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Parameter")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_description: Option<String> = None;
                let mut value_display_name: Option<String> = None;
                let mut value_from: Option<String> = None;
                let mut value_generate: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_required: Option<bool> = None;
                let mut value_value: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_description => value_description = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_display_name => value_display_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_from => value_from = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_generate => value_generate = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_required => value_required = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Parameter {
                    description: value_description,
                    display_name: value_display_name,
                    from: value_from,
                    generate: value_generate,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    required: value_required,
                    value: value_value,
                })
            }
        }

        deserializer.deserialize_struct(
            "Parameter",
            &[
                "description",
                "displayName",
                "from",
                "generate",
                "name",
                "required",
                "value",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for Parameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Parameter",
            1 +
            self.description.as_ref().map_or(0, |_| 1) +
            self.display_name.as_ref().map_or(0, |_| 1) +
            self.from.as_ref().map_or(0, |_| 1) +
            self.generate.as_ref().map_or(0, |_| 1) +
            self.required.as_ref().map_or(0, |_| 1) +
            self.value.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.description {
            serde::ser::SerializeStruct::serialize_field(&mut state, "description", value)?;
        }
        if let Some(value) = &self.display_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "displayName", value)?;
        }
        if let Some(value) = &self.from {
            serde::ser::SerializeStruct::serialize_field(&mut state, "from", value)?;
        }
        if let Some(value) = &self.generate {
            serde::ser::SerializeStruct::serialize_field(&mut state, "generate", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.required {
            serde::ser::SerializeStruct::serialize_field(&mut state, "required", value)?;
        }
        if let Some(value) = &self.value {
            serde::ser::SerializeStruct::serialize_field(&mut state, "value", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
