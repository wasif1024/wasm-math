use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn add2numbers(n1:i32,n2:i32)->i32{
n1+n2
}
#[wasm_bindgen]
pub fn square(n1:i32,n2:i32)->i32{
n1*n2
}
/*pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/
