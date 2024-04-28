fn main() {
    let form = Form::new()
        .name("Poll")
        .field("Question1")
        .field("Question2")
        .field("Question3");

    println!("Form name: {}", form.name);
    for field in form.fields {
        println!("Field name: {}", field.name);
    }
}

struct Form {
    name: String,
    fields: Vec<Field>,
}

struct Field {
    name: String,
}

impl Form {
    fn new() -> Self {
        Self {
            name: String::new(),
            fields: vec![],
        }
    }

    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    fn field(mut self, name: &str) -> Self {
        let field = Field {
            name: name.to_string(),
        };
        self.fields.push(field);
        self
    }
}