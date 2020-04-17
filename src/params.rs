pub struct Params {
    params: Vec<(String, String)>,
}

impl Params {
    pub fn new() -> Self {
        Params { params: Vec::new() }
    }

    pub fn add(&mut self, param: &str, value: &str) -> &mut Self {
        self.params.push((param.to_string(), value.to_string()));
        self
    }

    pub fn get_params(&self) -> &Vec<(String, String)> {
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
