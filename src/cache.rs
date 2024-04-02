use std::collections::HashMap;

#[allow(dead_code)]
enum CacheValue {
    Int(i32),
    Int64(i64),
    Float(f64),
    String(String),
    StringVec(Vec<String>),
    IntVec(Vec<i32>),
    I64Vec(Vec<i64>),
    FloatVec(Vec<f64>),
}

#[allow(dead_code)]
pub struct Cache {
    savelocation: String,
    cache: HashMap<String, CacheValue>,
}

#[allow(dead_code)]
impl Cache {
    pub fn new(savelocation: String) -> Cache {
        // Register panic hook to save to filesystem in case of failure.
        // This will take quite some work to implement probably.
        /*
        panic::set_hook(Box::new(|panici| {
            eprintln!("Panic detected:\n {}", panici)
            
        }));
         */

        Cache {
            savelocation: savelocation,
            cache: HashMap::new(),
        }
    }

    pub fn add_int32(&mut self, key: &str, val: i32) -> &mut Cache {
        self.add_i32(String::from(key), val)
    }

    pub fn add_i32(&mut self, key: String, value: i32) -> &mut Cache {
        self.cache.insert(key, CacheValue::Int(value));
        self
    }

    pub fn add_int64(&mut self, key: &str, val: i64) -> &mut Cache {
        self.add_i64(String::from(key), val)
    }

    pub fn add_i64(&mut self, key: String, value: i64) -> &mut Cache {
        self.cache.insert(key, CacheValue::Int64(value));
        self
    }

    pub fn add_float(&mut self, key: &str, val: f64) -> &mut Cache {
        self.add_f64(String::from(key), val)
    }

    pub fn add_f64(&mut self, key: String, value: f64) -> &mut Cache {
        self.cache.insert(key, CacheValue::Float(value));
        self
    }

    pub fn add_str(&mut self, key: &str, val: &str) -> &mut Cache {
        self.add_string(String::from(key), String::from(val))
    }

    pub fn add_string(&mut self, key: String, value: String) -> &mut Cache {
        self.cache.insert(key, CacheValue::String(value));
        self
    }

    pub fn add_vec_str(&mut self, key: &str, value: Vec<&str>) -> &mut Cache {
        self.add_string_vector(
            String::from(key),
            value.iter().map(|x| x.to_string()).collect(),
        )
    }

    pub fn add_vec_string(&mut self, key: &str, value: Vec<String>) -> &mut Cache {
        self.add_string_vector(
            String::from(key),
            value
        )
    }

    pub fn add_string_vector(&mut self, key: String, value: Vec<String>) -> &mut Cache {
        self.cache.insert(key, CacheValue::StringVec(value));
        self
    }

    pub fn add_vec_int(&mut self, key: &str, value: Vec<i32>) -> &mut Cache {
        self.add_int_vector(String::from(key), value)
    }

    pub fn add_int_vector(&mut self, key: String, value: Vec<i32>) -> &mut Cache {
        self.cache.insert(key, CacheValue::IntVec(value));
        self
    }

    pub fn add_vec_i64(&mut self, key: &str, value: Vec<i64>) -> &mut Cache {
        self.add_i64_vector(String::from(key), value)
    }

    pub fn add_i64_vector(&mut self, key: String, value: Vec<i64>) -> &mut Cache {
        self.cache.insert(key, CacheValue::I64Vec(value));
        self
    }

    pub fn add_vec_f64(&mut self, key: &str, value: Vec<f64>) -> &mut Cache {
        self.add_f64_vector(String::from(key), value)
    }

    pub fn add_f64_vector(&mut self, key: String, value: Vec<f64>) -> &mut Cache {
        self.cache.insert(key, CacheValue::FloatVec(value));
        self
    }
}
