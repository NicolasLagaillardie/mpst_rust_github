use proc_macro2::Span;
use syn::spanned::Spanned;

pub(crate) fn branching_variants(input: syn::Item) -> Result<(), syn::Error> {
    if let syn::Item::Enum(e) = input
    {
        // now check variant order
        let mut names = Vec::new();
        for variant in e.variants.iter()
        {
            let name = variant.ident.to_string();

            println!("variant: {:?}", &variant);

            if names.last().map(|last| &name < last).unwrap_or(false)
            {
                let next_lex_i = names.binary_search(&name).unwrap_err();
                return Err(syn::Error::new(
                    variant.span(),
                    format!("{name} should sort before {}", names[next_lex_i]),
                ));
            }
            names.push(name);
        }
        Ok(())
    }
    else
    {
        Err(syn::Error::new(
            Span::call_site(),
            "expected enum or match expression",
        ))
    }
}
