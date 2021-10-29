use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;

#[proc_macro_attribute]
pub fn branching(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut out = input.clone();

    let ty = parse_macro_input!(input as syn::Item);
    assert!(args.is_empty());

    if let Err(e) = branching_variants(ty) {
        out.extend(TokenStream::from(e.to_compile_error()));
    }
    out
}

fn branching_variants(input: syn::Item) -> Result<(), syn::Error> {
    if let syn::Item::Enum(e) = input {
        // now check variant order
        let mut names = Vec::new();
        for variant in e.variants.iter() {
            let name = variant.ident.to_string();
            if names.last().map(|last| &name < last).unwrap_or(false) {
                let next_lex_i = names.binary_search(&name).unwrap_err();
                return Err(syn::Error::new(
                    variant.span(),
                    format!("{} should sort before {}", name, names[next_lex_i]),
                ));
            }
            names.push(name);
        }
        Ok(())
    } else {
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected enum or match expression",
        ))
    }
}

// #[derive(Default)]
// struct LexiographicMatching {
//     errors: Vec<syn::Error>,
// }

// impl syn::visit_mut::VisitMut for LexiographicMatching {
//     fn visit_expr_match_mut(&mut self, m: &mut syn::ExprMatch) {
//         if m.attrs.iter().any(|a| a.path.is_ident("branching")) {
//             // remove #[branching]
//             m.attrs.retain(|a| !a.path.is_ident("branching"));
//             // check the variants
//             let mut names = Vec::new();
//             let mut wild = None;
//             for arm in m.arms.iter() {
//                 if let Some(ref w) = wild {
//                     self.errors.push(syn::Error::new_spanned(
//                         &w,
//                         "wildcard pattern should come last",
//                     ));
//                     break;
//                 }

//                 let path = if let Some(path) = get_arm_path(arm.pats.iter().next().unwrap()) {
//                     path
//                 } else if let Some(syn::Pat::Wild(w)) = arm.pats.iter().next() {
//                     wild = Some(w);
//                     continue;
//                 } else {
//                     self.errors.push(syn::Error::new_spanned(
//                         &arm.pats,
//                         "unsupported by #[remain::branching]",
//                     ));
//                     continue;
//                 };
//                 let name = path_as_string(&path);
//                 if names.last().map(|last| &name < last).unwrap_or(false) {
//                     let next_lex_i = names.binary_search(&name).unwrap_err();
//                     self.errors.push(syn::Error::new_spanned(
//                         path,
//                         format!("{} should sort before {}", name, names[next_lex_i]),
//                     ));
//                 }
//                 names.push(name);
//             }
//         }

//         // keep recursing
//         syn::visit_mut::visit_expr_match_mut(self, m)
//     }
// }

// fn path_as_string(path: &syn::Path) -> String {
//     path.segments
//         .iter()
//         .map(|s| format!("{}", quote! {#s}))
//         .collect::<Vec<_>>()
//         .join("::")
// }

// fn get_arm_path(arm: &syn::Pat) -> Option<syn::Path> {
//     match *arm {
//         syn::Pat::Ident(syn::PatIdent { ident: ref id, .. }) => Some(id.clone().into()),
//         syn::Pat::Path(ref p) => Some(p.path.clone()),
//         syn::Pat::Struct(ref s) => Some(s.path.clone()),
//         syn::Pat::TupleStruct(ref s) => Some(s.path.clone()),
//         _ => None,
//     }
// }

// #[proc_macro_attribute]
// pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
//     let mut f = parse_macro_input!(input as syn::ItemFn);
//     assert!(args.is_empty());

//     let mut lm = LexiographicMatching::default();
//     lm.visit_item_fn_mut(&mut f);
//     let mut ts = quote! {#f};
//     ts.extend(lm.errors.into_iter().take(1).map(|e| e.to_compile_error()));
//     ts.into()
// }
