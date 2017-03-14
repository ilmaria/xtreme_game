pub mod math;

use self::math::Vec3;

#[no_mangle]
pub fn hello() -> String {
    "Hello 1".to_string()
}

#[no_mangle]
pub fn vec_test() -> Vec3 {
    let mut a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(5.0, 6.0, 7.0);
    a.add(&b);
    a
}

#[cfg(test)]
mod tests {
    use super::{vec_test, Vec3};

    #[test]
    fn it_works() {
        let a = vec_test();
        assert_eq!(a, Vec3::new(6.0, 8.0, 10.0));
    }
}
