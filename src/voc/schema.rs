use crate::types::{IRI, URI};
use crate::voc::Vocabulary;

/// `SchemaVoc` contains constants of the Schema.org vocabulary.
pub struct SchemaVoc;

impl Vocabulary for SchemaVoc {
  type Prefix = IRI;
  type Full = IRI;

  fn prefix() -> Self::Prefix {
    IRI::from("schema:")
  }

  fn full() -> Self::Full {
    IRI::from("https://schema.org/")
  }
}

/*
enum Number {
  Float(f64),
  Integer(isize),
}

enum Boolean {
  True,
  False,
}

enum Class {

}

enum Date {
  Date,
  Time,
  DateTime,
}

enum Property {

}

enum SchemaTypes {
  // DateTypes.
  Boolean(bool),
  Text(IRI),
  URL(URI),
  Number,
  Date,

  Class,
  Property,
}


/// The basic data types such as Integers, Strings, etc.
const DataType: IRI = prefix + "DataType";
/// Boolean: True or False.
const Boolean: IRI = prefix + "Boolean";
/// The boolean value true.
const True: IRI = prefix + "True";
/// The boolean value false.
const False: IRI = prefix + "False";
/// Data type: Text.
const Text: IRI = prefix + "Text";
/// Data type: URL.
const URL: IRI = prefix + "URL";
/// Data type: Number.
const Number: IRI = prefix + "Number";
/// Data type: Integer.
const Integer: IRI = prefix + "Integer";
/// Data type: Floating number.
const Float: IRI = prefix + "Float";
/// A date value in ISO 8601 date format.
const Date: IRI = prefix + "Date";
/// A point in time recurring on multiple days in the
/// form hh:mm:ss[Z|(+|-)hh:mm].
const Time: IRI = prefix + "Time";
/// A combination of date and time of day in the form
/// [-]CCYY-MM-DDThh:mm:ss[Z|(+|-)hh:mm] (see Chapter 5.4 of ISO 8601).
const DateTime: IRI = prefix + "DateTime";


/// A class, also often called a 'Type'; equivalent to rdfs:Class.
const Class: IRI = prefix + "Class";
/// A property, used to indicate attributes and relationships
/// of some Thing; equivalent to rdf:Property.
const Property: IRI = prefix + "Property";

/// The name of the item.
const Name: IRI = prefix + "name";
/// The URL of the item property.
const UrlProp: IRI = prefix + "url";
*/
