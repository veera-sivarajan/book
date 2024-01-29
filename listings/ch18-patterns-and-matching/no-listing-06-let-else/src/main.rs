fn main() {
    // ANCHOR: here
    let Some(x) = some_option_value else {
        // `else` block must be diverging (!)
        return; 
    };
    // ANCHOR_END: here
}
