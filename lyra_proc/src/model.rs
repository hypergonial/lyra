use syn::{
    Ident, Result, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct Args(pub(super) Vec<Ident>);

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Self(vars.into_iter().collect()))
    }
}
