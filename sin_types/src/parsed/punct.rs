use super::*;

pub trait ParsedPunct: Parse + AsInStr {}

#[macro_export]
macro_rules! define_parsed_punct {
    ($ident:ident, [$($tt:tt)+]) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $ident {
            pub span: $crate::Span,
        }

        impl ParsedPunct for $ident {}

        impl $crate::traits::AsInStr for $ident {
			fn in_str(&self) -> InStr {
				stringify!($($tt)+).into()
			}
		}

		impl $crate::span::Spanned for $ident {
			fn span(&self) -> Span {
				self.span
			}
		}

		impl $crate::ToTokens for $ident {
			fn to_token_stream(&self) -> $crate::TokenStream {
				[$crate::TokenTree::Leaf(t![$($tt)+], self.span)][..].into()
			}
		}

		impl $crate::parsing::Parse for $ident {
			fn parse<'a, T: Default + Clone>(input: &mut $crate::parsing::Parser<'a, T>) -> $crate::parsing::ParseResult<Self> {
				let Some(token_tree) = input.next() else {
					return Err($crate::parsing::ParseError::new().expected_token(
						pat![$($tt)+],
						None,
						input.span(),
					))
				};
				let token: Token = token_tree.clone().into();
				match token {
					t![$($tt)+] => Ok($ident { span: input.span() }),
					_ => Err($crate::parsing::ParseError::new().expected_token(pat![$($tt)+], Some(token), token_tree.span())),
				}
			}
		}
    };
}

define_parsed_punct!(Plus, [+]);
define_parsed_punct!(Minus, [-]);
define_parsed_punct!(Star, [*]);
define_parsed_punct!(Slash, [/]);
define_parsed_punct!(Percent, [%]);
define_parsed_punct!(Caret, [^]);
define_parsed_punct!(Not, [!]);
define_parsed_punct!(And, [&]);
define_parsed_punct!(Or, [|]);
define_parsed_punct!(AndAnd, [&&]);
define_parsed_punct!(OrOr, [||]);
define_parsed_punct!(Shl, [<<]);
define_parsed_punct!(Shr, [>>]);
define_parsed_punct!(PlusEq, [+=]);
define_parsed_punct!(MinusEq, [-=]);
define_parsed_punct!(StarEq, [*=]);
define_parsed_punct!(SlashEq, [/=]);
define_parsed_punct!(PercentEq, [%=]);
define_parsed_punct!(CaretEq, [^=]);
define_parsed_punct!(AndEq, [&=]);
define_parsed_punct!(OrEq, [|=]);
define_parsed_punct!(ShlEq, [<<=]);
define_parsed_punct!(ShrEq, [>>=]);
define_parsed_punct!(Eq, [=]);
define_parsed_punct!(EqEq, [==]);
define_parsed_punct!(Ne, [!=]);
define_parsed_punct!(Gt, [>]);
define_parsed_punct!(Lt, [<]);
define_parsed_punct!(Ge, [>=]);
define_parsed_punct!(LArrow, [<-]);
define_parsed_punct!(Le, [<=]);
define_parsed_punct!(At, [@]);
define_parsed_punct!(Underscore, [_]);
define_parsed_punct!(Dot, [.]);
define_parsed_punct!(DotDot, [..]);
define_parsed_punct!(DotDotDot, [...]);
define_parsed_punct!(DotDotEq, [..=]);
define_parsed_punct!(Comma, [,]);
define_parsed_punct!(Semi, [;]);
define_parsed_punct!(Colon, [:]);
define_parsed_punct!(PathSep, [::]);
define_parsed_punct!(RArrow, [->]);
define_parsed_punct!(FatArrow, [=>]);
define_parsed_punct!(Pound, [#]);
define_parsed_punct!(Dollar, [$]);
define_parsed_punct!(Question, [?]);
define_parsed_punct!(Tilde, [~]);
