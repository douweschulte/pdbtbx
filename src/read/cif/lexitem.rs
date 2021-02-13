/// A top level data block for a CIF file
#[derive(Debug, PartialEq)]
pub struct DataBlock {
    /// The name
    pub name: String,
    /// The Items
    pub items: Vec<Item>,
}

/// An Item in a CIF file
#[derive(Debug, PartialEq)]
pub enum Item {
    /// A data item
    DataItem(DataItem),
    /// A saveframe
    SaveFrame(SaveFrame),
}

/// A save frame in a CIF file
#[derive(Debug, PartialEq)]
pub struct SaveFrame {
    /// The name
    pub name: String,
    /// The Data Items
    pub items: Vec<DataItem>,
}

/// A data item, either a Single data item or a Loop
#[derive(Debug, PartialEq)]
pub enum DataItem {
    /// A Single data item
    Single(Single),
    /// A Loop
    Loop(Loop),
}

/// A single data item, consisting of a tag with a value
#[derive(Debug, PartialEq)]
pub struct Single {
    /// The Tag or Name
    pub name: String,
    /// The value
    pub content: Value,
}

/// A loop consisting of a header with tags and a body with values
#[derive(Debug, PartialEq)]
pub struct Loop {
    /// The header with the names for the columns
    pub header: Vec<String>,
    /// The data itself, should be a multiple of the amount of headers
    pub data: Vec<Value>,
}

/// A value for a CIF record
#[derive(Debug, PartialEq)]
pub enum Value {
    /// A value that is inapplicable
    Inapplicable,
    /// A value that is unknown
    Unknown,
    /// A numeric value, integers are represented as floats
    Numeric(f64),
    /// A numeric value with a set uncertainty, written as 'number(uncertainty)' eg 42.0(9)
    NumericWithUncertainty(f64, u32),
    /// A textual value, possibly containing whitespace and newlines
    Text(String),
}

impl DataBlock {
    /// Find a Single data item with the given tag, it returns the first found
    pub fn find(&self, tag: &str) -> Option<&Single> {
        for item in &self.items {
            if let Some(single) = item.find(tag) {
                return Some(single);
            }
        }
        None
    }
}

impl Item {
    /// Find a Single data item with the given tag, it returns the first found
    pub fn find(&self, tag: &str) -> Option<&Single> {
        match self {
            Item::DataItem(di) => di.find(tag),
            Item::SaveFrame(sf) => sf.find(tag),
        }
    }
}

impl SaveFrame {
    /// Find a Single data item with the given tag, it returns the first found
    pub fn find(&self, tag: &str) -> Option<&Single> {
        for item in &self.items {
            if let Some(single) = item.find(tag) {
                return Some(single);
            }
        }
        None
    }
}

impl DataItem {
    /// Find a Single data item with the given tag, it returns the first found
    pub fn find(&self, tag: &str) -> Option<&Single> {
        match self {
            DataItem::Single(single) if single.name == tag => Some(single),
            _ => None,
        }
    }
}
