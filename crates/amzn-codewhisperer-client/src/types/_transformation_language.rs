// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `TransformationLanguage`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let transformationlanguage = unimplemented!();
/// match transformationlanguage {
///     TransformationLanguage::Cobol => { /* ... */ },
///     TransformationLanguage::CSharp => { /* ... */ },
///     TransformationLanguage::Java11 => { /* ... */ },
///     TransformationLanguage::Java17 => { /* ... */ },
///     TransformationLanguage::Java21 => { /* ... */ },
///     TransformationLanguage::Java8 => { /* ... */ },
///     TransformationLanguage::Jcl => { /* ... */ },
///     TransformationLanguage::PlI => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `transformationlanguage` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `TransformationLanguage::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `TransformationLanguage::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `TransformationLanguage::NewFeature` is defined.
/// Specifically, when `transformationlanguage` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `TransformationLanguage::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    ::std::clone::Clone,
    ::std::cmp::Eq,
    ::std::cmp::Ord,
    ::std::cmp::PartialEq,
    ::std::cmp::PartialOrd,
    ::std::fmt::Debug,
    ::std::hash::Hash,
)]
pub enum TransformationLanguage {
    #[allow(missing_docs)] // documentation missing in model
    Cobol,
    #[allow(missing_docs)] // documentation missing in model
    CSharp,
    #[allow(missing_docs)] // documentation missing in model
    Java11,
    #[allow(missing_docs)] // documentation missing in model
    Java17,
    #[allow(missing_docs)] // documentation missing in model
    Java21,
    #[allow(missing_docs)] // documentation missing in model
    Java8,
    #[allow(missing_docs)] // documentation missing in model
    Jcl,
    #[allow(missing_docs)] // documentation missing in model
    PlI,
    /// `Unknown` contains new variants that have been added since this code was generated.
    #[deprecated(
        note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants."
    )]
    Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
}
impl ::std::convert::From<&str> for TransformationLanguage {
    fn from(s: &str) -> Self {
        match s {
            "COBOL" => TransformationLanguage::Cobol,
            "C_SHARP" => TransformationLanguage::CSharp,
            "JAVA_11" => TransformationLanguage::Java11,
            "JAVA_17" => TransformationLanguage::Java17,
            "JAVA_21" => TransformationLanguage::Java21,
            "JAVA_8" => TransformationLanguage::Java8,
            "JCL" => TransformationLanguage::Jcl,
            "PL_I" => TransformationLanguage::PlI,
            other => TransformationLanguage::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(
                other.to_owned(),
            )),
        }
    }
}
impl ::std::str::FromStr for TransformationLanguage {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(TransformationLanguage::from(s))
    }
}
impl TransformationLanguage {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            TransformationLanguage::Cobol => "COBOL",
            TransformationLanguage::CSharp => "C_SHARP",
            TransformationLanguage::Java11 => "JAVA_11",
            TransformationLanguage::Java17 => "JAVA_17",
            TransformationLanguage::Java21 => "JAVA_21",
            TransformationLanguage::Java8 => "JAVA_8",
            TransformationLanguage::Jcl => "JCL",
            TransformationLanguage::PlI => "PL_I",
            TransformationLanguage::Unknown(value) => value.as_str(),
        }
    }

    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "COBOL", "C_SHARP", "JAVA_11", "JAVA_17", "JAVA_21", "JAVA_8", "JCL", "PL_I",
        ]
    }
}
impl ::std::convert::AsRef<str> for TransformationLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl TransformationLanguage {
    /// Parses the enum value while disallowing unknown variants.
    ///
    /// Unknown variants will result in an error.
    pub fn try_parse(value: &str) -> ::std::result::Result<Self, crate::error::UnknownVariantError> {
        match Self::from(value) {
            #[allow(deprecated)]
            Self::Unknown(_) => ::std::result::Result::Err(crate::error::UnknownVariantError::new(value)),
            known => Ok(known),
        }
    }
}
impl ::std::fmt::Display for TransformationLanguage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            TransformationLanguage::Cobol => write!(f, "COBOL"),
            TransformationLanguage::CSharp => write!(f, "C_SHARP"),
            TransformationLanguage::Java11 => write!(f, "JAVA_11"),
            TransformationLanguage::Java17 => write!(f, "JAVA_17"),
            TransformationLanguage::Java21 => write!(f, "JAVA_21"),
            TransformationLanguage::Java8 => write!(f, "JAVA_8"),
            TransformationLanguage::Jcl => write!(f, "JCL"),
            TransformationLanguage::PlI => write!(f, "PL_I"),
            TransformationLanguage::Unknown(value) => write!(f, "{}", value),
        }
    }
}
