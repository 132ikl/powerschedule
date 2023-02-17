use std::fmt::Display;

use serde::{de::Error as DeError, Deserialize, Deserializer};

#[derive(Debug)]
pub enum Error {
    ConvertError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct ClassCode {
    subject: String,
    course: u16,
}

impl ClassCode {
    pub fn new(subject: String, course: u16) -> Self {
        return ClassCode { subject, course };
    }
}

#[derive(Debug, Deserialize)]
pub struct ClassCodes(Vec<ClassCode>);

#[derive(Debug, Deserialize)]
pub struct Class {
    name: ClassCode,
    credits: u8,
    #[serde(deserialize_with = "empty_string_is_none")]
    prerequisites: Option<ClassCodes>,
    #[serde(deserialize_with = "empty_string_is_none")]
    corequisites: Option<ClassCodes>,
}

impl Class {
    pub fn name(&self) -> String {
        format!("{}{}", self.name.subject, self.name.course)
    }
}

fn empty_string_is_none<'de, D>(deserializer: D) -> Result<Option<ClassCodes>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.is_empty() {
        true => Ok(None),
        false => Ok(Some(
            s.try_into()
                .map_err(|e: Error| DeError::custom(e.to_string()))?,
        )),
    }
}

impl TryFrom<String> for ClassCode {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 6 {
            return Err(Error::ConvertError(format!(
                "Class code {} must be 6 characters long",
                value
            )));
        }

        let (subject, code) = value.split_at(3);
        let course: u16 = code.parse().map_err(|_| -> Error {
            Error::ConvertError(format!("Failed to convert course code {} to integer", code))
        })?;
        Ok(ClassCode::new(subject.to_string(), course))
    }
}

impl TryFrom<String> for ClassCodes {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let classes: Vec<ClassCode> = value
            .split("|")
            .map(|x| x.to_string().try_into())
            .try_collect::<Vec<ClassCode>>()?;
        Ok(Self(classes))
    }
}
