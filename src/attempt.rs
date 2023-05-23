//! This module contains the macro
//! for attempting to run a block of
//! code.

/// Try to run the first block of code and,
/// upon error, run the second block of code.
/// Each step in the block of code must
/// return `Result<Ok(_), Err(e)>`.
///
/// # Example
///
/// ```
/// use mpstthree::attempt;
///
/// fn do_step1() -> Result<(), String> {
///     println!("Step 1");
///     Ok(())
/// }

/// fn do_step2() -> Result<(), String> {
///     println!("Step 2");
///     Err("Error in step 2".into())
/// }
/// fn do_step3() -> Result<(), String> {
///     println!("Step 3");
///     Ok(())
/// }
///
/// fn main() {
///     attempt! {{
///        do_step1();
///        do_step2();
///        do_step3();
///     } catch (e) {
///        println!("Failed to perform necessary steps: {e:?}");
///     }}
/// }
/// ```
#[macro_export]
macro_rules! attempt { // `try` is a reserved keyword
    (@recurse ($a:expr) { } catch ($e:ident) $b:block) => {
       if let Err ($e) = $a $b
    };
    (@recurse ($a:expr) { $e:expr; $($tail:tt)* } $($handler:tt)*) => {
       attempt!{@recurse ($a.and_then (|_| $e)) { $($tail)* } $($handler)*}
    };
    ({ $e:expr; $($tail:tt)* } $($handler:tt)*) => {
       attempt!{@recurse ($e) { $($tail)* } $($handler)* }
    };
 }
