use crate::value::Value;

// *************** Visitor *****************
pub trait EnumAccess {}

pub trait Visitor {
    type Value;

    fn visit_enum<A>(self, a: A) -> Self::Value
    where
        A: EnumAccess;

    fn visit_string(self, v: String) -> Self::Value;
}

struct ValueVisitor;

impl Visitor for ValueVisitor {
    type Value = Value;

    fn visit_string(self, v: String) -> Self::Value {
        unimplemented!()
    }

    fn visit_enum<A>(self, a: A) -> Self::Value
    where
        A: EnumAccess,
    {
        panic!()
    }
}

// *************** Deserialize *****************
///
/// #[derive(Deserialize, PartialEq, Debug)]
/// struct Person {
///     first_name: String,
///     last_name: String,
///     own_business: bool,
///     address: Option<String>
/// }

pub trait Deserialize {
    fn deserialize<D>(deserializer: D) -> Self
    where
        D: Deserializer;
}

impl Deserialize for Value {
    fn deserialize<D>(deserializer: D) -> Self
    where
        D: Deserializer,
    {
        let variants = vec!["Literal", "Number", "Array", "RsonMap"];
        deserializer.deserialize_enum("Value", variants.as_slice(), ValueVisitor)
    }
}

// *************** Deserializer *****************

pub trait Deserializer {
    fn deserialize_string<V>(self, v: V) -> V::Value
    where
        V: Visitor;

    fn deserialize_enum<V>(self, name: &str, variants: &[&str], v: V) -> V::Value
    where
        V: Visitor;
}

struct StringDeserializer {
    value: String,
}

impl Deserializer for StringDeserializer {
    fn deserialize_string<V>(self, v: V) -> V::Value
    where
        V: Visitor,
    {
        v.visit_string(self.value)
    }

    fn deserialize_enum<V>(self, name: &str, variants: &[&str], v: V) -> V::Value
    where
        V: Visitor,
    {
        unimplemented!()
    }
}

struct ValueDeserializer;

impl Deserializer for ValueDeserializer {
    fn deserialize_string<V>(self, v: V) -> V::Value
    where
        V: Visitor,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(self, name: &str, variants: &[&str], v: V) -> V::Value
    where
        V: Visitor,
    {
        panic!()
    }
}
