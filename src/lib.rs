use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    // Import JavaScript's console.log function
    // #[wasm_bindgen(js_namespace = console)]
    // fn log(s: &str);

    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}


// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
