mod token;
mod parser;
mod property;
//mod util;

#[macro_use] extern crate lazy_static;
extern crate regex;

use std::env;

use parser::Parser;
use parser::ASTNode;
use property::Property;

fn main() {
    propositional_logic();
}

fn propositional_logic(){
    let parser = Parser::new_with_regex_str("[TF]");

    let rules = vec![
        Property::new(parser.parse("=(and(x,y),and(y,x))")).unwrap(), // Commutative and
        Property::new(parser.parse("=(and(x,and(y,z)),and(and(x,y),z))")).unwrap(), // Associative and
        Property::new(parser.parse("=(or(x,y),or(y,x))")).unwrap(), // Commutative or
        Property::new(parser.parse("=(or(x,or(y,z)),or(or(x,y),z))")).unwrap(), // Associative or
        Property::new(parser.parse("=(or(x,and(y,z)),and(or(x,y),or(x,z)))")).unwrap(), // Distributive or
        Property::new(parser.parse("=(and(x,or(y,z)),or(and(x,y),and(x,z)))")).unwrap(), // Distributive and
        Property::new(parser.parse("=(not(not(x)),x)")).unwrap(), // Double negation
        Property::new(parser.parse("=(not(T),F)")).unwrap(), // Definition of T (Definition of F comes from double negation)
        Property::new(parser.parse("=(and(not(x),x),F)")).unwrap(), // Condition of negation and
        Property::new(parser.parse("=(or(not(x),x),T)")).unwrap(), // Condition of negation or
        Property::new(parser.parse("=(or(x,x),x)")).unwrap(), // Idempotent or
        Property::new(parser.parse("=(and(x,x),x)")).unwrap(), // Idempotent and
        Property::new(parser.parse("=(and(x,T),x)")).unwrap(), //identity T
        Property::new(parser.parse("=(or(x,F),x)")).unwrap(), //identity F
        Property::new(parser.parse("=(not(and(x,y)),or(not(x),not(y)))")).unwrap(), // De Morgan's and
        Property::new(parser.parse("=(not(or(x,y)),and(not(x),not(y)))")).unwrap(), // De Morgan's or
        Property::new(parser.parse("=(imp(x,y),or(not(x),y))")).unwrap(), // Material Implication
        Property::new(parser.parse("=(bi(x,y),and(imp(x,y),imp(y,x)))")).unwrap(), // Bi-implication
    ];

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => eprintln!("{}: missing parameters. Try -h.", {&args[0]}),
        2 => if args[1]=="-h" {
                println!("Use: [OPTION] \"<formula>\" <rule>");
                println!("Rules:");
                rules.iter().enumerate().for_each(
                    |(i,x)| println!("{}: =({},{})", i, x.get_left(), x.get_right())
                )
            },
        _ => {
            let expression = parser.parse(&args[1]);
            println!("Inputted Formula: {}",expression);
            println!("Outputted Formulas: ");
            display_node_vec(rules[args[2].parse::<usize>().unwrap()].apply_thorough(&expression));
        }
    }
}

fn natural_numbers(){ //equivalent to peano axioms
    let parser = Parser::new_with_regex_str(r"(\d*\.?\d)+");

    let rules = vec![
        Property::new(parser.parse("=(  +(x,y),  +(y,x)  )")).unwrap(), //commutative
        Property::new(parser.parse("=(  +(x,+(y,z)),  +(+(x,y),z)  )")).unwrap(), //associate
        Property::new(parser.parse("=(  *(x,y),  *(y,x)  )")).unwrap(), //commutative
        Property::new(parser.parse("=(  *(x,*(y,z)),  *(*(x,y),z)  )")).unwrap(), //associate
        Property::new(parser.parse("=(  *(x,+(y,z)),  +(*(x,y),*(x,z))  )")).unwrap(), //distributive
        Property::new(parser.parse("=(  +(x,0), x  )")).unwrap(), //additive identity
        Property::new(parser.parse("=(  *(x,0), 0  )")).unwrap(), //multiplicative
    ];
    
    
    let expression = parser.parse("+(3,+(4,+(5,6,3)))");

    println!("{}",expression);
}

fn display_node_vec(nodes : Vec<ASTNode>) {
    for node in nodes.iter() {
        println!("{}",node);
    }
}

// fn get_tree(line: &str) -> ASTNode {
//     let token_iter = Token::iter_from_exp(line);
//     //println!("{:?}",Token::iter_from_exp(line).collect::<Vec<Token>>());
//     Parser::new().parse(line)
// }

// fs::read_to_string("filename.txt").lines();

// now... allow properties to be defined for functions
// +(x,y)=+(y,x) commutative
// *(x,y)=*(y,x) commutative
// *(x,+(y,z))=+(*(x,y),*(x,z)) distributive
// *(x,1)=x identity
// +(x,0)=x identity
// +(x,+(y,z))=+(+(x,y),z) associative

// hmmm... how do I specify on which symbols do properties apply?
// for example, if D_x is the differentiation operator for x, then
//
// D_x(^(x,n))=*(n,^(x,-(n,1)))
//
// but this should only apply for non-functions
// reasoning is that differentiation is not a real to real mapping
// but rather an operator (function to function)
// probably best to not include differentiation for now 