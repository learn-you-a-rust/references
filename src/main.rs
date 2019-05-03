fn main() {
    // this var must be declared as mutable here to be passed as
    // a mutable reference later
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    // we can still use s1 because we passed it as
    // a reference!
    println!("The length of {} is {}.", s1, len);

    //change(&s1); // this doesn't work! references are immutable
                  // by default
                  
    change_mut(&mut s1); // this does work

    // making mutable references while we have immutable ones is also 
    // not allowed!
    let s4 = &s1;        // <-- if this ref exists, &mut s1 refs can't be used 
    
    //let s2 = &mut s1;    // you can't do this. if you try to use these vars, 
    //let s3 = &mut s1;    // the code will fail

    //println!("{}, {}", s2, s3); // this won't run because you can't do
                                // two mutable references in the same scope
                                
    //println!("{}, {}", s1, s2); // this doesn't work either

    // but you can be sneaky and make a new scope like this
    {
        let s2 = &mut s1;
    } // s2 goes out of scope here

    //let s3 = &mut s1;

    // multiple immutable refs are okay because they are not 
    // changing the data
    let immut_ref1 = &s1;
    let immut_ref2 = &s1; 

    println!("{}, {}", immut_ref1, immut_ref2);


    // dangling references are prevented in rust
    let reference_to_nothing = no_dangle();

}
// this will give an error because we're trying to return
// a pointer to a string that's gone out of scope
fn no_dangle() -> String {
//fn dangle() -> &String {
    let s = String::from("hello");

    //&s // this won't work
    s // return the String directly instead
}

// this works because its argument is declared mutable
fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}


// this does not work
fn change(some_string: &String) {
    //some_string.push_str(", world");
}

// takes a reference to a string as an argument
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope. but the function did not have ownership of it, so 
  // nothing happens
