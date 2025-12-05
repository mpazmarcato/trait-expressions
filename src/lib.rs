pub trait Expression: {
    fn evaluate(&self) -> Option<i64>;
    fn to_string(&self) -> String;
}

pub struct Number {
    value: i64,
}

pub struct Negation {
    expr: Box<dyn Expression>,
}

pub struct BinaryOperation {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    operator: BinaryOperator,
}

pub enum BinaryOperator {
    Addition,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

impl Expression for Number {
    fn evaluate(&self) -> Option<i64> {
        Some(self.value)
    }

    fn to_string(&self) -> String {
        format!("empilhar {}", self.value)
    }
}

impl Expression for Negation {
    fn evaluate(&self) -> Option<i64> {
        if let Some(valor) = self.expr.evaluate() {
            if let Some(negado) = valor.checked_neg() {
                Some(negado)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn to_string(&self) -> String {
        format!("{}\nnegar", self.expr.to_string())
    }
}

impl Expression for BinaryOperation {
    fn evaluate(&self) -> Option<i64> {
        let left_value = self.left.evaluate()?;
        let right_value = self.right.evaluate()?;
        match self.operator {
            BinaryOperator::Addition=> left_value.checked_add(right_value),
            BinaryOperator::Subtract => left_value.checked_sub(right_value),
            BinaryOperator::Multiply => left_value.checked_mul(right_value),
            BinaryOperator::Divide => {
                if right_value == 0 {
                    None
                } else {
                    left_value.checked_div(right_value)
                }
            }
            BinaryOperator::Remainder => {
                if right_value == 0 {
                    None
                } else {
                    left_value.checked_rem(right_value)
                }
            }
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{}\n{}\n{}",
            self.left.to_string(),
            self.right.to_string(),
            match self.operator {
                BinaryOperator::Addition => "adicionar",
                BinaryOperator::Subtract => "subtrair",
                BinaryOperator::Multiply => "multiplicar",
                BinaryOperator::Divide => "dividir",
                BinaryOperator::Remainder => "resto",
            }
        )
    }
}

impl Number {
    pub fn new(value: i64) -> Self { Number { value } }
}

impl Negation {
    pub fn new(expr: Box<dyn Expression>) -> Self { Negation { expr } }
}

impl BinaryOperation {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>, operator: BinaryOperator,) -> Self { BinaryOperation { left, right, operator, } }
}