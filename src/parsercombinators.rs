// parser s r = [s] -> [(r, [s])]
// Based on the uu-tc haskell package, but then in Rust
type ParserFN<'a, S, R> = Box<Fn(Vec<S>) -> Vec<(R, Vec<S>)> + 'a>;

struct Parser<'a, S: 'a, R: 'a> {
    function: ParserFN<'a, S, R>
}

impl<'a, S, R> Parser<'a, S, R> {
    fn Run(&self, x: Vec<S>) -> Vec<(R, Vec<S>)> {
        (self.function)(x)
    }
    fn new(x: ParserFN<'a, R, S>) -> Parser<'a, R, S>{
        Parser {function: x}
    }
}

impl<'a, S, R> Clone for Parser<'a, S, R> {
    fn clone(&self) -> Self {
        Parser {
            function: self.function,
        }
    }
}

pub fn tryparse() {
    let input = vec!('a', 'b');
    println!("{:?}", sequence(symbol('a'), symbol('b'), &combine).Run(input));
}

fn combine(a: char, b: char) -> String {
    let mut x = a.to_string();
    x.push(b);
    x
}

// Elementary

fn any_symbol<'a, T: Clone>() -> Parser<'a, T, T> {
    Parser::new(Box::new(
        |x: Vec<T>| 
        if x.len() > 0 {
            vec!((x[0].clone(), x[1..].to_vec()))
        } else {
            Vec::new()
        }
    ))
} 

fn satisfy<'a, T, F>(pred: F) -> Parser<'a, T, T> 
where
    T: Clone,
    F: Fn(T) -> bool + 'a
{
    Parser::new(Box::new(
        move |x| 
        if x.len() > 0 && pred(x[0].clone()) {
            vec!((x[0].clone(), x[1..].to_vec()))
        } else {
            Vec::new()
        }
    ))
}

fn empty<'a, S, A>() -> Parser<'a, S, A> {
    Parser::new(Box::new(
        |_| 
        Vec::new()
    ))
}

fn succeed<'a, S, A>(result: A) -> Parser<'a, S, A> 
where
    A: Clone + 'a
{
    Parser::new(Box::new(
        move |x| 
        vec!((result.clone(), x))
    ))
}

// Combinators

// <|>
fn choice<'a, S, A>(parser1: Parser<'a, S, A>, parser2: Parser<'a, S, A>) -> Parser<'a, S, A> 
where
    S: Clone + 'a,
    A: 'a
{
    Parser::new(Box::new(
        move |x|
        if x.len() > 0 {
            let mut result = parser1.Run(x.clone());
            result.extend(parser2.Run(x.clone()));
            return result; 
        } else {
            Vec::new()
        }
    ))
}

// <*>
fn sequence<'a, S: 'a, A: Clone + 'a, B: 'a, C: 'a>(parser1: Parser<'a, S, A>, parser2: Parser<'a, S, B>, combine: &'a Fn(A, B) -> C) -> Parser<'a, S, C> 
{
    Parser::new(Box::new(
        &move |x| {
            let r1 = parser1.Run(x);
            let mut output = Vec::new();
            for option1 in r1 {
                let r2 = parser2.Run(option1.1);
                for option2 in r2 {
                    output.push((combine(option1.0.clone(), option2.0), option2.1));
                }
            }
            return output;
        }
    ))
}


// <$>
fn map<'a, S: 'a, A: 'a, B: 'a>(parser: Parser<'a, S, A>, function: &'a Fn(A) -> B) -> Parser<'a, S, B> {
    Parser::new(Box::new(
        move |x| {
            let r1 = parser.Run(x);
            let mut output = Vec::new();
            for option in r1 {
                let r2 = function(option.0);
                output.push((r2, option.1));
            }
            return output;
        }
    ))
}

// Derived

fn symbol<'a, S: Clone + PartialEq + 'a>(symbol: S) -> Parser<'a, S, S> {
    satisfy(move |x| x == symbol)
}

fn token<'a, S: Clone + PartialEq + 'a>(tokens: Vec<S>) -> Parser<'a, S, Vec<S>> {
    Parser::new(Box::new(
        move |x|
        if x.len() > 0 {
            let mut y = x;
            for t in tokens.clone() {
                let r = symbol(t).Run(y.clone());
                if r.len() == 0 {return Vec::new();}
                y = r[0].1.clone();
            }
            return vec!((tokens.clone(), y));
        } else {
            Vec::new()
        }
    ))
}

//option
fn option<'a, S: Clone + 'a, A: Clone + 'a>(parser: Parser<'a, S, A>, default: A) -> Parser<'a, S, A> {
    Parser::new(Box::new(
        move |x| {
            let result = parser.Run(x.clone());
            if result.len() > 0 {
                result
            } else {
                vec!((default.clone(), x))
            }
        }
    ))
}

//many
fn many<'a, S: Clone + 'a, A: Clone + 'a>  (parser: Parser<'a, S, A>) -> Parser<'a, S, Vec<A>> {
    Parser::new(Box::new(
        move |x| 
        map(
            sequence(
                parser.clone(), 
                choice(many(parser.clone()), succeed(Vec::new())),
                &|y: A, ys: Vec<A>| {let mut res = ys.clone(); res.push(y.clone()); res}
            ),
            &|y| y.clone()
        ).Run(x)
    ))
}
//some 
