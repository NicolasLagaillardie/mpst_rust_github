use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::{collections::HashSet, mem};
use syn::{
    parse::Nothing, parse2, parse_quote, punctuated::Punctuated, Error, Fields, GenericArgument,
    GenericParam, Ident, Index, Item, ItemEnum, ItemStruct, ItemType, PathArguments, Result, Type,
};

fn idents_set<P>(params: &Punctuated<GenericParam, P>) -> HashSet<Ident> {
    let idents = params.iter().filter_map(|param| match param {
        GenericParam::Type(ty) => Some(ty.ident.clone()),
        _ => None,
    });
    idents.collect::<HashSet<_>>()
}

fn punctuated_prepend<T, P: Default>(left: &mut Punctuated<T, P>, mut right: Punctuated<T, P>) {
    right.extend(mem::take(left));
    *left = right;
}

fn unroll_type(mut ty: &mut Type) -> &mut Type {
    loop {
        ty = match ty {
            Type::Group(ty) => ty.elem.as_mut(),
            Type::Paren(ty) => ty.elem.as_mut(),
            _ => break,
        }
    }

    ty
}

fn augment_type(mut ty: &mut Type, exclude: &HashSet<Ident>) {
    while let Type::Path(path) = unroll_type(ty) {
        if *path == parse_quote!(Self) {
            break;
        }

        let segment = match path.path.segments.last_mut() {
            Some(segment) => segment,
            _ => break,
        };

        if let PathArguments::None = segment.arguments {
            if exclude.contains(&segment.ident) {
                break;
            }

            segment.arguments = PathArguments::AngleBracketed(parse_quote!(<>));
        }

        let args = match &mut segment.arguments {
            PathArguments::AngleBracketed(args) => &mut args.args,
            _ => break,
        };

        let is_empty = args.is_empty();
        punctuated_prepend(args, parse_quote!('__r, __R));

        if is_empty {
            break;
        }

        ty = match args.last_mut() {
            Some(GenericArgument::Type(ty)) => ty,
            _ => break,
        };
    }
}

fn session_type(mut input: ItemType) -> TokenStream {
    let exclude = idents_set(&input.generics.params);
    punctuated_prepend(
        &mut input.generics.params,
        parse_quote!('__r, __R: ::rumpsteak::Role),
    );
    augment_type(&mut input.ty, &exclude);
    input.into_token_stream()
}

fn session_enum(mut input: ItemEnum) -> Result<TokenStream> {
    if input.variants.is_empty() {
        let message = "expected at least one variant";
        return Err(Error::new_spanned(&input.variants, message));
    }

    let ident = &input.ident;
    let exclude = idents_set(&input.generics.params);

    let mut generics = input.generics.clone();
    punctuated_prepend(
        &mut generics.params,
        parse_quote!('__q, '__r, __R: ::rumpsteak::Role + '__r),
    );
    let (impl_generics, _, _) = generics.split_for_impl();

    let mut generics = input.generics.clone();
    punctuated_prepend(
        &mut generics.params,
        parse_quote!('__q, __R: ::rumpsteak::Role),
    );
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let mut idents = Vec::with_capacity(input.variants.len());
    let mut labels = Vec::with_capacity(input.variants.len());
    let mut tys = Vec::with_capacity(input.variants.len());

    for variant in &mut input.variants {
        idents.push(&variant.ident);
        let fields = match &mut variant.fields {
            Fields::Unnamed(fields) => Ok(&mut fields.unnamed),
            fields => Err(Error::new_spanned(fields, "expected tuple variants")),
        }?;

        if fields.len() != 2 {
            let message = "expected exactly two fields per variant";
            return Err(Error::new_spanned(fields, message));
        }

        let mut fields = fields.iter_mut();

        let label = &fields.next().unwrap().ty;
        labels.push(label);

        let ty = &mut fields.next().unwrap().ty;
        augment_type(ty, &exclude);
        tys.push(&*ty);
    }

    let mut output = TokenStream::new();
    for (label, ty) in labels.iter().zip(&tys) {
        output.extend(quote! {
            impl #impl_generics ::rumpsteak::Choice<'__r, #label> for #ident #ty_generics #where_clause {
                type Session = #ty;
            }
        });
    }

    punctuated_prepend(
        &mut input.generics.params,
        parse_quote!('__r, __R: ::rumpsteak::Role),
    );
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();

    #[cfg(feature = "serialize")]
    {
        let mut where_clause = where_clause.cloned().unwrap_or_else(|| parse_quote!(where));
        where_clause.predicates.push(parse_quote!(Self: 'static));

        output.extend(quote! {
            impl #impl_generics ::rumpsteak::serialize::SerializeChoices for #ident #ty_generics #where_clause {
                fn serialize_choices(mut s: ::rumpsteak::serialize::ChoicesSerializer<'_>) {
                    #(s.serialize_choice::<#labels, #tys>();)*
                }
            }
        });
    }

    let mut generics = input.generics.clone();
    generics.make_where_clause().predicates.push(parse_quote! {
        __R::Message: #(::rumpsteak::Message<#labels> +)*
    });

    let (_, _, where_clause) = generics.split_for_impl();
    output.extend(quote! {
        impl #impl_generics ::rumpsteak::Choices<'__r> for #ident #ty_generics #where_clause {
            type Role = __R;

            fn downcast(
                state: ::rumpsteak::State<'__r, Self::Role>,
                message: <Self::Role as Role>::Message,
            ) -> ::core::result::Result<Self, <Self::Role as Role>::Message> {
                #(let message = match ::rumpsteak::Message::downcast(message) {
                    Ok(label) => {
                        return Ok(Self::#idents(
                            label,
                            ::rumpsteak::FromState::from_state(state)
                        ));
                    }
                    Err(message) => message
                };)*

                Err(message)
            }
        }
    });

    Ok(quote!(#input #output))
}

pub fn session(attr: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let Nothing = parse2(attr)?;
    match parse2::<Item>(input)? {
        Item::Type(input) => Ok(session_type(input)),
        Item::Enum(input) => session_enum(input),
        item => Err(Error::new_spanned(item, "expected a type or enum")),
    }
}
