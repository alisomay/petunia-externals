use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    visit_mut::{self, VisitMut},
    Expr, ItemFn, Stmt,
};

struct ErrorLogger;

impl VisitMut for ErrorLogger {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        let handled = match expr {
            // Handle try operator (?) in any context
            Expr::Try(expr_try) => {
                // Visit the inner expression first
                visit_mut::visit_expr_mut(self, &mut expr_try.expr);

                let inner = &expr_try.expr;
                *expr = syn::parse_quote! {
                    (#inner.inspect_err(|err| { error!("{}", err); }))?
                };
                true // Mark as handled
            }
            // Handle direct Err calls in expressions
            Expr::Call(expr_call) => {
                if let Expr::Path(path) = &*expr_call.func {
                    if path
                        .path
                        .segments
                        .last()
                        .map(|s| s.ident == "Err")
                        .unwrap_or(false)
                    {
                        // Visit the arguments first
                        for arg in &mut expr_call.args {
                            visit_mut::visit_expr_mut(self, arg);
                        }

                        let error_expr = &expr_call.args[0];
                        *expr = syn::parse_quote! {
                            Err(#error_expr).inspect_err(|err| { error!("{}", err); })
                        };
                        true // Mark as handled
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            // Handle match expressions
            Expr::Match(expr_match) => {
                // Visit the match expression
                visit_mut::visit_expr_mut(self, &mut expr_match.expr);

                for arm in &mut expr_match.arms {
                    // Visit the guard if it exists
                    if let Some((_, guard)) = &mut arm.guard {
                        visit_mut::visit_expr_mut(self, guard);
                    }

                    // Special handling for the body
                    match &mut *arm.body {
                        Expr::Block(block) => {
                            self.visit_block_mut(&mut block.block);
                        }
                        Expr::If(expr_if) => {
                            visit_mut::visit_expr_mut(self, &mut expr_if.cond);
                            self.visit_block_mut(&mut expr_if.then_branch);
                            if let Some((_, else_branch)) = &mut expr_if.else_branch {
                                visit_mut::visit_expr_mut(self, else_branch);
                            }

                            if let Some(Stmt::Expr(Expr::Call(call), _)) =
                                expr_if.then_branch.stmts.last()
                            {
                                if let Expr::Path(path) = &*call.func {
                                    if path
                                        .path
                                        .segments
                                        .last()
                                        .map(|s| s.ident == "Err")
                                        .unwrap_or(false)
                                    {
                                        let last_idx = expr_if.then_branch.stmts.len() - 1;
                                        let error_expr = &call.args[0];
                                        expr_if.then_branch.stmts[last_idx] = syn::parse_quote! {
                                            Err(#error_expr).inspect_err(|err| { error!("{}", err); })
                                        };
                                    }
                                }
                            }
                        }
                        Expr::Call(call) => {
                            if let Expr::Path(path) = &*call.func {
                                if path
                                    .path
                                    .segments
                                    .last()
                                    .map(|s| s.ident == "Err")
                                    .unwrap_or(false)
                                {
                                    let error_expr = &call.args[0];
                                    *arm.body = syn::parse_quote! {
                                        Err(#error_expr).inspect_err(|err| { error!("{}", err); })
                                    };
                                } else {
                                    visit_mut::visit_expr_mut(self, &mut arm.body);
                                }
                            } else {
                                visit_mut::visit_expr_mut(self, &mut arm.body);
                            }
                        }
                        _ => visit_mut::visit_expr_mut(self, &mut arm.body),
                    }
                }
                true // Mark as handled
            }
            _ => false,
        };

        // Only continue visiting if we haven't handled this expression
        if !handled {
            visit_mut::visit_expr_mut(self, expr);
        }
    }
}

#[proc_macro_attribute]
pub fn log_errors(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let mut logger = ErrorLogger;
    logger.visit_block_mut(&mut input_fn.block);

    let output = quote! {
        #input_fn
    };

    output.into()
}
