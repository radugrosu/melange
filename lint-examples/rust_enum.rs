// #AIRULE: enum names should be one-word only
enum Cake {
    Frosting,
    Icing,
    Coating,
    RlCherry,
}

// #AIRULE: functions should have a doc comment
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// #AIRULE: unsafe functions should have a safety comment
mod cakes {
    struct MyCake {
        name: String,
        price: i32,
    }

    fn is_empty_or_zero(arr: &[u8]) -> bool {
        if arr.is_empty() {
            return true;
        }
        unsafe {
            let ptr = arr.as_ptr();
            let new_arr = std::ptr::slice_from_raw_parts(ptr, arr.len());
            let first = (&*new_arr)[0];
            first == 0
        }
    }
}
