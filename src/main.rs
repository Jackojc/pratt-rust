#![allow(dead_code, non_camel_case_types, unused_variables, non_snake_case, non_upper_case_globals)]


use std::str::Chars;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;



macro_rules! c_enum {
	($name:ident : $repr:ty { $($fields:ident = $values:expr),* $(,)? }) => {
		mod $name {
			$( pub const $fields: $repr = $values; )*
		}
	}
}



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tokens {
	Literal(String),
	Add, Sub, Mul, Div, Pow,
	LParen, RParen,
	Whitespace,
}


/// Get next token.
fn next_token(source: &mut Peekable<Chars>) -> Option<Tokens> {
	use Tokens::*;

	Some(match source.next() {
		Some(first @ '0' ..= '9') => {
			let mut content = String::new();
			content.push(first);

			while let Some(next_ch @ '0'..='9') = source.peek() {
				content.push(*next_ch);
				source.next();
			}

			Literal(content)
		},
		Some('+') => Add,
		Some('-') => Sub,
		Some('*') => Mul,
		Some('/') => Div,
		Some('^') => Pow,

		Some('(') => LParen,
		Some(')') => RParen,

		Some(' ') => Whitespace,

		None => return None,
		_ => panic!("wtf is this"),
	})
}

c_enum! { Precedence: i32 {
	Add = 1,
	Sub = 1,

	Mul = 2,
	Div = 2,

	Neg = 3,
	Pos = 3,

	Pow = 4,
}}

c_enum! { Associativity: i32 {
	Left = 0,
	Right = 1,
}}



type PrecTable = HashMap<Tokens, (i32, i32)>;
type AffixTable = (PrecTable, PrecTable);


fn make_tables() -> AffixTable {
	let mut prefix = HashMap::new();
	let mut infix = HashMap::new();

	// unary
	prefix.insert(Tokens::Sub, (Precedence::Neg, Associativity::Right));
	prefix.insert(Tokens::Add, (Precedence::Pos, Associativity::Right));

	// binary
	infix.insert(Tokens::Add, (Precedence::Add, Associativity::Left));
	infix.insert(Tokens::Sub, (Precedence::Sub, Associativity::Left));
	infix.insert(Tokens::Mul, (Precedence::Mul, Associativity::Left));
	infix.insert(Tokens::Div, (Precedence::Div, Associativity::Left));
	infix.insert(Tokens::Pow, (Precedence::Pow, Associativity::Right));

	(prefix, infix)
}

fn parse(
	tokens: &mut Peekable<Iter<Tokens>>,
	table: &AffixTable,
	bp: i32
) -> f32 {
	use Tokens::*;

	let (prefix, infix) = table;

	// handle prefix shit
	let mut left = match tokens.next() {
		Some(Tokens::LParen) => {
			let left = parse(tokens, table, 0);
			assert_eq!(tokens.next(), Some(&Tokens::RParen));
			left
		},
		Some(Tokens::Add) => {
			let &(prec, ass) = prefix.get(&Tokens::Add).expect("no bueno 1");
			parse(tokens, table, prec + ass)
		},
		Some(Tokens::Sub) => {
			let &(prec, ass) = prefix.get(&Tokens::Sub).expect("no bueno2 ");
			-parse(tokens, table, prec + ass)
		},
		Some(Literal(string)) => string.parse::<f32>().unwrap(),
		_ => unreachable!("sdoifhdsfh"),
	};


	// pratt parsing lets go baby
	while let Some(token) = tokens.peek() {
		if let Some(&(prec, ass)) = infix.get(token) {
			if prec < bp {
				break
			}

			left = match tokens.next().unwrap() {
				Sub => left - parse(tokens, table, prec + ass),
				Add => left + parse(tokens, table, prec + ass),
				Mul => left * parse(tokens, table, prec + ass),
				Div => left / parse(tokens, table, prec + ass),
				Pow => left.powf(parse(tokens, table, prec + ass)),
				_ => unreachable!("nox has a not ass"),
			};

		} else {
			break
		}
	}

	left
}


fn main() {
	let source = " 3 + (-4) ^ 2";
	let mut iter = source.chars().peekable();


	let mut tokens = vec![];
	while let Some(token) = next_token(&mut iter) {
		match token {
			Tokens::Whitespace => (),
			_ => tokens.push(token),
		}
	}

	let table = make_tables();
	let mut iter = tokens.iter().peekable();
	println!("{}", parse(&mut iter, &table, 0));
}

