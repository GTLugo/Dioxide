use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn entrypoint(_: TokenStream, annotated_item: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(annotated_item as ItemFn);

  let name = &ast.sig.ident;

  quote! {
    mod boot {
      core::arch::global_asm! {
        ".section .text._start"
      }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn _start() -> ! {
      #name();
      loop {
        unsafe { core::arch::asm!("nop"); };
      }
    }

    #ast
  }
  .into()
}
