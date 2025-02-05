/// The square root of `x` (f32).
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn sqrtf(x: f32) -> f32 {
    let result;
    unsafe {
        core::arch::asm!(
            "vsqrt.f32 {x}, {x}",
            x = inlateout(sreg) x => result,
            options(pure, nomem, nostack),
        );
    }
    result
}
