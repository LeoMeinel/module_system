/*
 * File: lib.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 *	If defined, a lib crate is automatically generated.
 *  There can only be zero or one library crate per package.
 */

// Example using parent modules (alternative 1 when having same name structs from different modules)
use std::fmt;
// Example using full path (alternative 2 when having same name structs from different modules)
use std::fmt::Result;
// Same as use std::io; std::io::Write;
#[allow(unused_imports)]
use std::io::{self, Write};
// Glob operator imports all pub modules
#[allow(unused_imports)]
use std::io::*;
// Rename one to avoid conflict with as
use std::io::Result as IoResult;

// Compact import of multiple modules from one parent module
#[allow(unused_imports)]
use rand::{CryptoRng, Error, Rng};

// Absolute path
//use crate::front_of_house::hosting;
/*
 * Relative path using self.
 * Bring hosting into scope. Should normally bring parent module into scope!
 * enum, struct, other items: specify full path
 * Make use pub to make external code able to reference it!
 */
pub use self::front_of_house::hosting;

// Define front_of_house module here, but get contents from file crate::front_of_house#
mod front_of_house;

pub fn eat_at_restaurant_add() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative Path
    front_of_house::hosting::add_to_waitlist();
}

#[allow(dead_code)]
fn server_order() {}

// Create module, can also create mod inside mod
#[allow(dead_code)]
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // Relative Path with reference to parent module
        super::server_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        //seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                //seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

#[allow(unused_variables)]
pub fn eat_at_restaurant_structs() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    //meal.seasonal_fruit = String::from("abc"); // seasonal_fruit is not public
    // Can not create struct because seasonal_fruit is not public
    let meal2 = back_of_house::Breakfast {
        toast: String::from("Wheat"),
        //seasonal_fruit: String::from("peaches"),
    };
}

#[allow(unused_variables)]
pub fn eat_at_restaurant_enum() {
    // If you mark an enum as pub, all variants are pub too
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

//use std::io;

#[allow(dead_code)]
fn function1() -> fmt::Result {
    Ok(())
}

#[allow(dead_code)]
fn function2() -> io::Result<()> {
    Ok(())
}

#[allow(dead_code)]
fn function3() -> Result {
    Ok(())
}

#[allow(dead_code)]
fn function4() -> IoResult<()> {
    Ok(())
}
