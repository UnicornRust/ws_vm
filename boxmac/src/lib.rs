use proc_macro::TokenStream;
use syn::{parenthesized, parse::{Parse, ParseBuffer, ParseStream}, parse_macro_input, token::Paren, Ident, LitFloat, LitInt, LitStr, Token};

/**
* proc_macro step for Rust
*
*  processing
*  parse
*  generating
*/

#[derive(Debug)]
enum Element{
    Ident(String),
    Str(String),
    ParenGroup(Vec<Element>),
    Int(i64),
    Double(f64),
}

impl Parse for Element {

    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Paren) {
            let content: ParseBuffer;

            // get the content of the parenthesis
            parenthesized!(content in input);
            let mut element: Vec<Element> = Vec::new();

            while !content.is_empty() {
                while content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }

                if !content.is_empty() {
                    element.push(content.parse()?);
                }
            }

            Ok(Element::ParenGroup(element))

        }else if input.peek(LitStr) { 
            let lit : LitStr = input.parse()?;
            Ok(Element::Str(lit.value()))
        }else if input.peek(LitInt) {
            let lit = input.parse::<LitInt>()?;
            Ok(Element::Int(lit.base10_parse::<i64>()?))
        }else if input.peek(LitFloat) {
            let lit = input.parse::<LitFloat>()?;
            Ok(Element::Double(lit.base10_parse::<f64>()?))
        }else if input.peek(Token![extern]) {
            input.parse::<Token![extern]>()?;
            let language = input.parse::<Ident>()?;
            if language.to_string() != "Rust" {
                panic!("Only Rust is supportingg as an external language")
            }
            let string = input.parse::<LitStr>()?;
            Ok(Element::Ident(string.value()))
        }else {
            let ident = input.parse::<Ident>()?;
            Ok(Element::Ident(ident.to_string()))
        }
    }
}


#[proc_macro]
pub fn racket(input: TokenStream) -> TokenStream {
    let mut source = String::new();
    for token in input{
        if let Some(span) = token.span().source_text() {
            source.push_str(&span);
        }else {
            source.push_str(&token.to_string());
        }
    }

    let processed = transform_native(&source);

    let tokens = match syn::parse_str::<proc_macro2::TokenStream>(&processed) {
        Ok(tokens) => tokens.into(),
        Err(err) => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Failed to parse source: {}", err)
            )
                .to_compile_error()
                .into();
        }
    };
    racket_impl(tokens)
}

fn racket_impl(input: TokenStream) -> TokenStream {
    let elments = parse_macro_input!(input as Element);
    make_output(&elments).parse().unwrap()
}


fn make_output(elements: &Element) -> String {
    match elements {
        Element::ParenGroup(elements) => {
            if elements.len() == 0 {
                "".to_string()
            }else if elements.len() == 1 {
                make_output(&elements[0])
            }else {
                let mut iter = elements.iter().enumerate();
                let mut output = match iter.next().unwrap().1 {
                    Element::Ident(id)  => id.clone() + "(",
                    _ => {
                        panic!("First element in ParenGroup must be an Ident");
                    }
                };
                for (i, element) in iter {
                    output.push_str(&make_output(element));
                    if i != elements.len() - 1 {
                        output.push_str(", ");
                    }else {
                        output.push_str(")");
                    }
                }
                output
            }
        },
        Element::Ident(ident) => ident.to_string(),
        Element::Str(str) => format!(r#"{}"#, str),
        Element::Int(int) => int.to_string(),
        Element::Double(double) => double.to_string(),
    }

}

fn transform_native(source: &str) -> String {
    let mut result = String::with_capacity(source.len());
    let mut i = 0;
    let chars: Vec<char> = source.chars().collect();

    while i < chars.len()  {
        if i + 5 < chars.len() 
            && chars[i] == 'n'
            && chars[i + 1] == 'a'
            && chars[i + 2] == 't'
            && chars[i + 3] == 'i'
            && chars[i + 4] == 'v'
            && chars[i + 5] == 'e'
        {
            i += 6;
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }
            if i < chars.len() && chars[i] == '{' {
                i += 1;
                let mut bracket_count = 1;
                let mut content = String::new();

                while i < chars.len() {
                    let ch = chars[i];
                    if ch == '{' {
                        bracket_count += 1;
                        content.push(ch);
                    }else if ch == '}' {
                        bracket_count -= 1;
                        if bracket_count == 0 {
                            break;
                        }
                        content.push(ch);
                    }else {
                        content.push(ch);
                    }
                    i += 1;
                }
                i += 1;
                result.push_str("extern Rust \"");
                result.push_str(&content);
                result.push('"');
                continue;
            }else  {
                panic!(r#"Expected: '{{' after 'native'"#);
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}
