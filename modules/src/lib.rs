// Rust basic program to learn the use of modules 
// Modules allow us to name `items` 

// define a module by looking at a restaurant 

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {} // Add the pub keyword to make the absolute or relative path to actually be usable
        fn seat_at_table() {}
    }
    // mod serving {
    //     fn take_order_() {}
    //     fn serve_order() {}
    //     fn take_payment() {}
    // }
}

// Define a function called eat a restaurant ( make this the public portion of our API )

pub fn eat_at_restaurant() {
    // absolute path 
    // Get the infro from the crate root 
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();
}