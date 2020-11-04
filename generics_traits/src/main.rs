// JSON encoder:
trait ToJson {
    fn to_json(&self) -> String {
        String::from("null")
    }
}

impl ToJson for () {}

impl ToJson for String {
    fn to_json(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl ToJson for i32 {
    fn to_json(&self) -> String {
        format!("{}", self)
    }
}

impl ToJson for f32 {
    fn to_json(&self) -> String {
        format!("{}", self)
    }
}

impl<T> ToJson for Option<T> where T: ToJson {
    fn to_json(&self) -> String {
        match self {
            Some(val) => val.to_json(),
            None => String::from("null"),
        }
    }
}

impl<T> ToJson for Vec<T> where T: ToJson {
    fn to_json(&self) -> String {
        let mut iter = self.iter();
        let first = iter.next();

        let mut result = match first {
            Some(first) => first.to_json(),
            None => String::new(),
        };

        for _i in iter {
            result.push_str(", ");
            result.push_str(&_i.to_json());
        }
        
        format!("[{}]", result)
    }
}

struct Student {
    age: i32,
    full_name: String,
    number: i32,
    hobby:Option<String>
}

impl ToJson for Student {
    fn to_json(&self) -> String {
        format!(
            r#"{{
                "age": {},
                "full_name": {},
                "number": {},
                "hobby": {}
            }}"#,
            self.age.to_json(), self.full_name.to_json(), 
            self.number.to_json(), self.hobby.to_json()
        )
    }
}

/*
fn to_json<T: ToJson>(value: T) -> String {
    value.to_json()
}*/

fn to_json(value: &dyn ToJson) -> String {
    value.to_json()
}

fn main() {
    println!("Number to json:{}", 9.to_json());
    println!("String to json:{}", String::from("UTF18").to_json());
    println!("Unit as json:{}", ().to_json());
    let arr = vec![Some(1.1), Some(2.2), None].to_json();
    println!("Vector as json:{}", arr);
    
    let student = Student {
        age: 16,
        full_name: String::from("Kevin McCarty"),
        number: 5,
        hobby: Some("Reading books".to_string())
    };
    println!("Our struct student as json:{}", student.to_json());

    let trait_object: &dyn ToJson = &5;

    println!("{}", to_json(trait_object));
    println!("{}", to_json(&5));
    println!("{}", to_json(&5 as &dyn ToJson));
    
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works_from_main() {
        assert_eq!(2 + 2, 4);
    }
}
