use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::{collections::HashSet, mem};
use syn::{
    parse::Nothing, parse2, parse_quote, punctuated::Punctuated, Error, Fields, GenericArgument,
    GenericParam, Ident, Item, ItemEnum, ItemType, PathArguments, Result, Type,
};

/// Get HashSet of params of the given type
fn idents_set<P>(params: &Punctuated<GenericParam, P>) -> HashSet<Ident> {
    params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(ty) => Some(ty.ident.clone()),
            _ => None,
        })
        .collect::<HashSet<_>>()
}

/// Extend `right` with `left`, then assign `right` value to `left`
fn punctuated_prepend<T, P: Default>(left: &mut Punctuated<T, P>, mut right: Punctuated<T, P>) {
    right.extend(mem::take(left));
    *left = right;
}

/// Extract types (parameters) from given type
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

/// Augment/expand the provided type, and take the excluded types in consideration
fn augment_type(mut ty: &mut Type, exclude: &HashSet<Ident>) {
    while let Type::Path(path) = unroll_type(ty) {
        // If recursion on the same type, break
        if *path == parse_quote!(Self) {
            break;
        }

        // If no element after the last element, break
        let segment = match path.path.segments.last_mut() {
            Some(segment) => segment,
            _ => break,
        };

        // If end of arguments of segment and no loop, complete with angle brackets
        if let PathArguments::None = segment.arguments {
            // If exclude does not contain ident of segment, break
            if exclude.contains(&segment.ident) {
                break;
            }

            segment.arguments = PathArguments::AngleBracketed(parse_quote!(<>));
        }

        // Unroll and assign arguments of segment to args
        let args = match &mut segment.arguments {
            PathArguments::AngleBracketed(args) => &mut args.args,
            _ => break,
        };

        // Complete args with parse_quote!('__r, __R)
        let is_empty = args.is_empty();
        punctuated_prepend(args, parse_quote!('__r, __R));

        // If no arguments, break
        if is_empty {
            break;
        }

        // Reassigne the arguments to the type ty
        ty = match args.last_mut() {
            Some(GenericArgument::Type(ty)) => ty,
            _ => break,
        };
    }
}

// If session_timed is a type
fn session_type(mut input: ItemType) -> TokenStream {
    let exclude = idents_set(&input.generics.params);
    punctuated_prepend(
        &mut input.generics.params,
        parse_quote!('__r, __R: ::rumpsteak::Role),
    );
    augment_type(&mut input.ty, &exclude);
    input.into_token_stream()
}

/// If session_timed is an enum
fn session_enum(mut input: ItemEnum) -> Result<TokenStream> {
    // If no variants, return error
    if input.variants.is_empty() {
        let message = "expected at least one variant";
        return Err(Error::new_spanned(&input.variants, message));
    }

    // Get Ident of type
    let ident = &input.ident;

    // Get excluded idents set from generic parameters
    let exclude = idents_set(&input.generics.params);

    // Get genereics of input
    let mut generics = input.generics.clone();

    // Concat params of generics with rumpsteak role and `__r
    punctuated_prepend(
        &mut generics.params,
        parse_quote!('__q, '__r, __R: ::rumpsteak::Role + '__r),
    );

    // Split a type's generics into the pieces required for impl'ing a trait for that type
    let (impl_generics, _, _) = generics.split_for_impl();

    let mut generics = input.generics.clone();

    // Concat params of generics with rumpsteak role
    punctuated_prepend(
        &mut generics.params,
        parse_quote!('__q, __R: ::rumpsteak::Role),
    );

    // Split a type's generics into the pieces required for impl'ing a trait for that type
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let mut idents = Vec::with_capacity(input.variants.len());
    let mut labels = Vec::with_capacity(input.variants.len());
    let mut tys = Vec::with_capacity(input.variants.len());

    // For each variant of the enum, augment the types it contains and add them to `tys`
    for variant in &mut input.variants {
        idents.push(&variant.ident);

        // Extract all the fields of the variant
        let fields = match &mut variant.fields {
            Fields::Unnamed(fields) => Ok(&mut fields.unnamed),
            fields => Err(Error::new_spanned(fields, "expected tuple variants")),
        }?;

        // If not exactly 2 fields
        if fields.len() != 2 {
            let message = "expected exactly two fields per variant";
            return Err(Error::new_spanned(fields, message));
        }

        // Iter over fields, get encapsulated label and type
        let mut fields = fields.iter_mut();

        let label = &fields.next().unwrap().ty;
        labels.push(label);

        let ty = &mut fields.next().unwrap().ty;

        // Augment encapsulated type
        augment_type(ty, &exclude);

        // Add type ty to vec tys
        tys.push(&*ty);
    }

    // For each label, add its related impl
    let mut output = TokenStream::new();
    for (label, ty) in labels.iter().zip(&tys) {
        output.extend(quote! {
            impl #impl_generics ::rumpsteak::Choice<'__r, #label> for #ident #ty_generics #where_clause {
                type Session = #ty;
            }
        });
    }

    // Extend params of generics of input with rumpsteak role
    punctuated_prepend(
        &mut input.generics.params,
        parse_quote!('__r, __R: ::rumpsteak::Role),
    );

    // Split a type's generics into the pieces required for impl'ing a trait for that type
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();

    // Extend where clauses with related impl
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

/// Maych given timed session with either a type of an enum
pub fn session_timed(attr: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let Nothing = parse2(attr)?;
    match parse2::<Item>(input)? {
        Item::Type(input) => Ok(session_type(input)),
        Item::Enum(input) => session_enum(input),
        item => Err(Error::new_spanned(item, "expected a type or enum")),
    }
}
