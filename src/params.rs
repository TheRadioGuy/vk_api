// TODO: Add param macro
use std::ops::Add;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Params {
    params: HashMap<String, String>,
}

impl Add for Params {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let params2 = other.get_params();
        let mut params = self.clone();


        for (k, v) in params2.into_iter(){
            params.add_param(k, v);
        }

        params
    }
}

impl Params {
    pub fn new() -> Self {
        Params { params: HashMap::new() }
    }

    pub fn add_param(&mut self, param: &str, value: &str) -> &mut Self {
        self.params.insert(param.to_string(), value.to_string());
        self
    }

    pub fn get_params(&self) -> &HashMap<String, String> {
        &self.params
    }

    pub fn concat(&self) -> String {
        let mut result = String::new();
        self.params
            .iter()
            .for_each(|(p, v)| result.push_str(&format!("{}={}&", p, v)));
        result
    }
}

#[macro_export]
/// Used for creating params
/// # Example:
///
/// let param = param!{"group_id" => "1", "fields" => "bdate"};
macro_rules! param(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut params = vkapi::params::Params::new();
            $(
                params.add_param($key, $value);
            )+
            params
        }
     };
);