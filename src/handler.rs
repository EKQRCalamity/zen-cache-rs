use std::{
    collections::HashMap,
    sync::Arc,
};

use crate::server;
use crate::cache;

#[allow(dead_code)]
pub struct Function {
    key: String,
    properties: Vec<String>,
    pub methods: Option<Vec<String>>,
    pub function: Arc<
        dyn (FnMut(&server::HTMLRequest, &mut cache::Cache) -> Result<String, std::io::Error>) + Send + Sync
    >,
    description: Option<String>,
}

#[allow(dead_code)]
impl Function {
    fn new(
        key: String,
        properties: Vec<&str>,
        methods: Option<Vec<String>>,
        description: Option<String>,
        function: Arc<
            dyn (FnMut(&server::HTMLRequest, &mut cache::Cache) -> Result<String, std::io::Error>) + Send + Sync
        >
    ) -> Function {
        Function {
            key: key,
            properties: properties
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            methods: methods,
            function: function,
            description: description,
        }
    }

    fn n(
        key: &str,
        properties: Vec<&str>,
        methods: Option<Vec<&str>>,
        description: Option<&str>,
        function: Arc<
            dyn (FnMut(&server::HTMLRequest, &mut cache::Cache) -> Result<String, std::io::Error>) + Send + Sync
        >
    ) -> Function {
        Function {
            key: String::from(key),
            properties: properties
                .iter()
                .map(|y| y.to_string())
                .collect::<Vec<String>>(),
            methods: match methods {
                Some(xmethods) => {
                    Some(xmethods
                        .iter()
                        .map(|y| y.to_string())
                        .collect::<Vec<String>>())
                },
                None => None,
            },
            description: match description {
                Some(xdesc) => Some(xdesc.to_string()),
                None => None,
            },
            function: function
        }
    }
}

#[allow(dead_code)]
pub struct FuncHelper {
    store: Vec<Function>,
}

#[allow(dead_code)]
impl FuncHelper {
    pub fn new() -> FuncHelper {
        FuncHelper {
            store: vec![
                Function::n(
                    "/addfloat",
                    vec![],
                    Some(vec!["POST"]),
                    Some("Add an entry to the cache."),
                    Arc::new(&add)
                )
            ],
        }
    }

    pub fn add(&mut self, func: Function) -> &FuncHelper {
        self.store.push(func);
        self
    }

    pub fn get_func_map(
        &self
    ) -> HashMap<
        String,
        Arc<dyn (FnMut(&server::HTMLRequest, &mut cache::Cache) -> Result<String, std::io::Error>) + Send + Sync>
    > {
        let mut map: HashMap<
            String,
            Arc<dyn (FnMut(&server::HTMLRequest, &mut cache::Cache) -> Result<String, std::io::Error>) + Send + Sync>
        > = HashMap::new();
        for function in self.store.iter() {
            map.insert(function.key.clone(), Arc::clone(&function.function));
        }
        map
    }

    pub fn get_func_map_raw(self) -> HashMap<String, Arc<Function>> {
        let mut map: HashMap<String, Arc<Function>> = HashMap::new();
        for function in self.store.into_iter() {
            map.insert(function.key.clone(), Arc::new(function));
        }
        map
    }

    pub fn pad(str: String, len: usize) -> String {
        let mut out = str.clone();
        if str.len() >= len {
            return str;
        }
        for _ in 0..len - str.len() {
            out.push(' ');
        }
        out
    }

    pub fn display(&self) -> String {
        // Calculate minimum lengths for formatting
        let mut min_length_key = 0;
        let mut min_length_properties = 0;
        let mut min_length_description = 0;
        for function in self.store.iter() {
            if function.key.len() > min_length_key {
                min_length_key = function.key.len();
            }
            if function.properties.join(", ").len() > min_length_properties {
                min_length_properties = function.properties.join(", ").len();
            }
            if
                function.description
                    .clone()
                    .unwrap_or(String::from("No description provided."))
                    .len() > min_length_description
            {
                min_length_description = function.description
                    .clone()
                    .unwrap_or(String::from("No description provided."))
                    .len();
            }
        }

        let mut out = String::new();
        // Header
        out.push_str(FuncHelper::pad("Key".to_string(), min_length_key).as_str());
        out.push_str(" | ");
        out.push_str(FuncHelper::pad("Properties".to_string(), min_length_properties).as_str());
        out.push_str(" | ");
        out.push_str(FuncHelper::pad("Description".to_string(), min_length_description).as_str());
        out.push_str(" | ");
        out.push_str("Methods\n");

        out.push_str(FuncHelper::pad("----".to_string(), min_length_key).as_str());
        out.push_str(" | ");
        out.push_str(FuncHelper::pad("-----------".to_string(), min_length_properties).as_str());
        out.push_str(" | ");
        out.push_str(FuncHelper::pad("------------".to_string(), min_length_description).as_str());
        out.push_str(" | ");
        out.push_str("--------\n");

        // Body
        for function in self.store.iter() {
            out.push_str(FuncHelper::pad(function.key.clone(), min_length_key).as_str());
            out.push_str(" | ");
            out.push_str(
                FuncHelper::pad(function.properties.join(", "), min_length_properties).as_str()
            );
            out.push_str(" | ");
            out.push_str(
                FuncHelper::pad(
                    function.description
                        .clone()
                        .unwrap_or(String::from("No description provided.")),
                    min_length_description
                ).as_str()
            );
            out.push_str(" | ");
            if let Some(methods) = &function.methods {
                out.push_str(methods.join(", ").as_str());
            } else {
                out.push_str("None");
            }
            out.push_str("\n");
        }
        out
    }
}

fn add(request: &server::HTMLRequest, cache: &mut cache::Cache) -> Result<String, std::io::Error> {
    cache.add_float("test", 6.4);
    Ok(String::from("Added float."))
}