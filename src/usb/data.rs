use serde_json;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct InData {
    action: String,
    args: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OutData{
    r#type: String,
    args: Vec<String>,
}

impl InData {
    pub fn new(action: String, args: Vec<String>) -> InData{
        let output = InData{
            action,
            args,
        };
        output
    }

    //Methods
    pub fn get_action(&self) -> &str{
        self.action.as_str()
    }
    pub fn get_args(&self) -> &str{
        self.args[0].as_str()
    }
    pub fn to_json_string(&self) -> String{
        let output = serde_json::to_string(&self).unwrap();
        output
    }
}

impl OutData {
    pub fn new(r#type: String) -> OutData{
        let v = Vec::new();
        let output = OutData{
            r#type,
            args: v,
        };
        output
    }

    //Methods
    pub fn get_type(&self) -> &str{
        self.r#type.as_str()
    }
    pub fn get_args(&self) -> &str{
        self.args[0].as_str()
    }
    pub fn to_json_string(&self) -> String{
        let output = serde_json::to_string(&self).unwrap();
        output
    }
}