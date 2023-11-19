#![feature(let_chains)]

extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree, Spacing, Delimiter};
use proc_macro::{Literal, Punct, Group};

#[proc_macro]
pub fn with_int(t: TokenStream) -> TokenStream {
  let (expr,  t) = split_at_comma(t);
  let (name,  t) = split_at_comma(t);
  let (range, t) = split_at_comma(t);

  let mut result = TokenStream::new();
  result.extend("match".parse::<TokenStream>().unwrap());
  result.extend(expr);

  let mut group = Vec::new();
  let (a, b) = parse_range(range);
  for n in a..=b {
    group.push(TokenTree::Literal(Literal::usize_suffixed(n)));
    group.push(TokenTree::Punct(Punct::new('=', Spacing::Joint)));
    group.push(TokenTree::Punct(Punct::new('>', Spacing::Alone)));

    let mut arm_group = TokenStream::new();
    arm_group.extend("const".parse::<TokenStream>().unwrap());
    arm_group.extend(name.clone());
    arm_group.extend(": usize = ".parse::<TokenStream>().unwrap());
    arm_group.extend([TokenTree::Literal(Literal::usize_suffixed(n))]);
    arm_group.extend([TokenTree::Punct(Punct::new(';', Spacing::Alone))]);
    arm_group.extend(t.clone());
    let arm = TokenTree::Group(Group::new(Delimiter::Brace, arm_group));

    group.push(arm);
    group.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
  }
  group.extend("_ => { unreachable!() }"
    .parse::<TokenStream>()
    .unwrap()
    .into_iter());

  result.extend([TokenTree::Group(Group::new(
     Delimiter::Brace, TokenStream::from_iter(group.into_iter())))]);

  result
}

fn parse_range(t: TokenStream) -> (usize, usize) {
  let mut iter = t.into_iter();

  let a = parse_usize(iter.next().unwrap());

  assert!(is_punct(&iter.next().unwrap(), '.'));
  assert!(is_punct(&iter.next().unwrap(), '.'));

  let bx = iter.next().unwrap();
  let b  = if is_punct(&bx, '=') {
    parse_usize(iter.next().unwrap())
  } else {
    parse_usize(bx) - 1
  };

  (a, b)
}

fn parse_usize(t: TokenTree) -> usize {
  t.to_string().parse().unwrap()
}

fn is_punct(t: &TokenTree, p: char) -> bool {
  if let TokenTree::Punct(r) = t {
     r == &p
  } else {
    return false
  }
}

fn split_at_comma(t: TokenStream) -> (TokenStream, TokenStream) {
  let mut first_half = Vec::new();
  let mut iter = t.into_iter();

  while let Some(tree) = iter.next() {
    if is_punct(&tree, ',') {
      break;
    } else {
      first_half.push(tree);
    }
  }

  if !first_half.is_empty() {
    return (
      TokenStream::from_iter(first_half.into_iter()),
      TokenStream::from_iter(iter)
    );
  } else {
    panic!("expected comma in with_length!")
  }
}

