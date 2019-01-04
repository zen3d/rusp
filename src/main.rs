use std::fmt;

#[derive(Debug)]
enum Object {
    Nil,
    T,
    Number(i32),
    Symbol(String),
    Cons { car: Box<Object>, cdr: Box<Object> }
}

fn write_list(f: &mut fmt::Formatter, obj: &Object) -> fmt::Result {
    match *obj {
        Object::Cons { ref car, ref cdr } => {
            write!(f, "{}", car);
            match **cdr {
                Object::Nil => Ok(()),
                _ => {
                    write!(f, " ");
                    write_list(f, cdr)
                }
            }
        },
        _ => write!(f, ". {}", obj)
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Nil => write!(f, "nil"),
            Object::T => write!(f, "t"),
            Object::Number(ref num) => write!(f,"{}", num),
            Object::Symbol(ref sym) => write!(f, "{}", sym),
            Object::Cons { .. } => {
                write!(f, "(");
                write_list(f, self);
                write!(f, ")")
            }
        }
    }
}

fn symbol(name: &str) -> Object {
    Object::Symbol(name.to_string())
}

fn cons(car: Object, cdr: Object) -> Object {
    Object::Cons { car: Box::new(car), cdr: Box::new(cdr) }
}

fn main() {
    let obj = cons(
        symbol("+"),
        cons(
            Object::Number(1),
            cons(Object::Number(2), Object::Nil)
        )
    );
    println!("{}", obj);
}
