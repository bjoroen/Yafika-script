#[derive(PartialEq, Debug)]
pub enum Statement {
    Let{
        name: String,
        initial_value: Expression,
    }, 
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Number(f64)
    
}
