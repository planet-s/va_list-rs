extern crate va_list;

#[link(name = "va_list_test")]
extern "C" {
    fn dispatch(context: *mut u8, count: u32, ...);
}

type CbType<'a> = &'a mut FnMut(u32, va_list::VaList);

#[no_mangle]
/// Method called by 'dispatch'
pub extern "C" fn inbound(context: *mut u8, count: u32, args: va_list::VaList) {
    let cb_ptr = unsafe { ::std::ptr::read(context as *mut CbType) };
    // call passed closure
    (cb_ptr)(count, args);
}

macro_rules! test_va_list {
	($int:expr, ($($args:expr),*), $code:expr) => ({
		let mut cb = $code;
		let mut cb_ref: CbType = &mut cb;

		unsafe {
			dispatch(&mut cb_ref as *mut _ as *mut u8, $int, $($args),*);
		}
	});
}

#[test]
fn trivial_values() {
    // Trivial test: Pass four random-ish sized integers
    test_va_list!(
        4,
        (123456u32, 2u64, 1i32, -23i64),
        |_count, mut list: va_list::VaList| unsafe {
            assert_eq!(list.get::<u32>(), 123456u32);
            assert_eq!(list.get::<u64>(), 2u64);
            assert_eq!(list.get::<i32>(), 1i32);
            assert_eq!(list.get::<i64>(), -23i64);
        }
    );
}

#[test]
fn floating_point() {
    test_va_list!(
        4,
        (123456f64, 0.1f64),
        |_count, mut list: va_list::VaList| unsafe {
            assert_eq!(list.get::<f64>(), 123456f64);
            assert_eq!(list.get::<f64>(), 0.1f64);
        }
    );
}
