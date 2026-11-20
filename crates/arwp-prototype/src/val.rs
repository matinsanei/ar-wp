use indexmap::IndexMap;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArrayKey {
    Int(i64),
    Str(String),
}

impl fmt::Display for ArrayKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArrayKey::Int(i) => write!(f, "{}", i),
            ArrayKey::Str(s) => write!(f, "{}", s),
        }
    }
}

impl ArrayKey {
    pub fn from_str(s: &str) -> Self {
        // PHP rule: "123" -> Int(123), but "0123" -> Str("0123")
        if let Ok(i) = s.parse::<i64>() {
            if i.to_string() == s {
                return ArrayKey::Int(i);
            }
        }
        ArrayKey::Str(s.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhpArray {
    pub map: IndexMap<ArrayKey, Value>,
    pub next_int_key: i64,
}

impl PhpArray {
    pub fn new() -> Self {
        Self {
            map: IndexMap::new(),
            next_int_key: 0,
        }
    }

    pub fn push(&mut self, val: Value) {
        let key = ArrayKey::Int(self.next_int_key);
        self.next_int_key += 1;
        self.map.insert(key, val);
    }

    pub fn insert(&mut self, key: ArrayKey, val: Value) {
        if let ArrayKey::Int(i) = key {
            if i >= self.next_int_key {
                self.next_int_key = i + 1;
            }
        }
        self.map.insert(key, val);
    }

    pub fn get(&self, key: &ArrayKey) -> Option<&Value> {
        self.map.get(key)
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, key: &ArrayKey) -> Option<&mut Value> {
        self.map.get_mut(key)
    }

    #[allow(dead_code)]
    pub fn contains_key(&self, key: &ArrayKey) -> bool {
        self.map.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn sort_keys(&mut self) {
        self.map.sort_keys();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Rc<RefCell<PhpArray>>),
}

impl Value {
    pub fn new_array() -> Self {
        Value::Array(Rc::new(RefCell::new(PhpArray::new())))
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0 && !f.is_nan(),
            Value::String(s) => !s.is_empty() && s != "0",
            Value::Array(arr) => !arr.borrow().is_empty(),
        }
    }

    pub fn to_string_lossy(&self) -> String {
        match self {
            Value::Null => String::new(),
            Value::Bool(b) => if *b { "1".to_string() } else { String::new() },
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::Array(_) => "Array".to_string(),
        }
    }

    pub fn to_int_lossy(&self) -> i64 {
        match self {
            Value::Null => 0,
            Value::Bool(b) => if *b { 1 } else { 0 },
            Value::Int(i) => *i,
            Value::Float(f) => *f as i64,
            Value::String(s) => s.trim().parse::<i64>().unwrap_or(0),
            Value::Array(arr) => if arr.borrow().is_empty() { 0 } else { 1 },
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => {
                let borrowed = arr.borrow();
                write!(f, "Array([")?;
                for (idx, (k, v)) in borrowed.map.iter().enumerate() {
                    if idx > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} => {}", k, v)?;
                }
                write!(f, "])")
            }
        }
    }
}
