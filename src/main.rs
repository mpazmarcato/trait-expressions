use trait_expressions::{
    Expression, 
    Number, Negation, BinaryOperation, BinaryOperator
};

fn main() {
    // Exemplo 1: 10 + 20
    println!("Exemplo 1: 10 + 20");
    let expr1 = Box::new(BinaryOperation::new(
        Box::new(Number::new(10)),
        Box::new(Number::new(20)),
        BinaryOperator::Addition,
    ));
    println!("{}", expr1.to_string());
    println!("Resultado: {:?}", expr1.evaluate());
    println!();
    
    // Exemplo 2: 10 / 0
    println!("Exemplo 2: 10 / 0");
    let expr2 = Box::new(BinaryOperation::new(
        Box::new(Number::new(10)),
        Box::new(Number::new(0)),
        BinaryOperator::Divide,
    ));
    println!("{}", expr2.to_string());
    println!("Resultado: {:?}", expr2.evaluate());
    println!();
    
    // Exemplo 3: (10 + 20) * 30
    println!("Exemplo 3: (10 + 20) * 30");
    let expr3 = Box::new(BinaryOperation::new(
        Box::new(BinaryOperation::new(
            Box::new(Number::new(10)),
            Box::new(Number::new(20)),
            BinaryOperator::Addition,
        )),
        Box::new(Number::new(30)),
        BinaryOperator::Multiply,
    ));
    println!("{}", expr3.to_string());
    println!("Resultado: {:?}", expr3.evaluate());
    println!();
    
    // Exemplo 4: 10 + 20 * 30
    println!("Exemplo 4: 10 + 20 * 30");
    let expr4 = Box::new(BinaryOperation::new(
        Box::new(Number::new(10)),
        Box::new(BinaryOperation::new(
            Box::new(Number::new(20)),
            Box::new(Number::new(30)),
            BinaryOperator::Multiply,
        )),
        BinaryOperator::Addition,
    ));
    println!("{}", expr4.to_string());
    println!("Resultado: {:?}", expr4.evaluate());
    println!();
    
    // Exemplo 5: (-(10 + 20) + 30 + 40 + (50 + 60)) * -5
    println!("Exemplo 5: (-(10 + 20) + 30 + 40 + (50 + 60)) * -5");
    let expr5 = Box::new(BinaryOperation::new(
        // esquerda: (-(10 + 20) + 30 + 40 + (50 + 60))
        Box::new(BinaryOperation::new(
            // (-(10 + 20) + 30 + 40)
            Box::new(BinaryOperation::new(
                // (-(10 + 20) + 30)
                Box::new(BinaryOperation::new(
                    // -(10 + 20)
                    Box::new(Negation::new(Box::new(BinaryOperation::new(
                        Box::new(Number::new(10)),
                        Box::new(Number::new(20)),
                        BinaryOperator::Addition,
                    )))),
                    Box::new(Number::new(30)),
                    BinaryOperator::Addition,
                )),
                Box::new(Number::new(40)),
                BinaryOperator::Addition,
            )),
            // (50 + 60)
            Box::new(BinaryOperation::new(
                Box::new(Number::new(50)),
                Box::new(Number::new(60)),
                BinaryOperator::Addition,
            )),
            BinaryOperator::Addition,
        )),
        // direita: -5
        Box::new(Negation::new(Box::new(Number::new(5)))),
        BinaryOperator::Multiply,
    ));
    
    println!("{}", expr5.to_string());
    println!("Resultado: {:?}", expr5.evaluate());
}