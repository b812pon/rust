// For `Send` generators, suggest a `T: Sync` requirement for `&T` upvars,
// and suggest a `T: Send` requirement for `&mut T` upvars.

#![feature(generators)]

fn assert_send<T: Send>(_: T) {}
//~^ NOTE required by a bound in `assert_send`
//~| NOTE required by this bound in `assert_send`
//~| NOTE required by a bound in `assert_send`
//~| NOTE required by this bound in `assert_send`

fn main() {
    let x: &*mut () = &std::ptr::null_mut();
    let y: &mut *mut () = &mut std::ptr::null_mut();
    assert_send(move || {
        //~^ ERROR generator cannot be sent between threads safely
        //~| NOTE generator is not `Send`
        //~| NOTE consider using `std::sync::Arc
        yield;
        let _x = x;
    });
    //~^^ NOTE captured value is not `Send` because `&` references cannot be sent unless their referent is `Sync`
    //~| NOTE has type `&*mut ()` which is not `Send`, because `*mut ()` is not `Sync`
    assert_send(move || {
        //~^ ERROR generator cannot be sent between threads safely
        //~| NOTE generator is not `Send`
        //~| NOTE consider using `std::sync::Arc
        yield;
        let _y = y;
    });
    //~^^ NOTE captured value is not `Send` because `&mut` references cannot be sent unless their referent is `Send`
    //~| NOTE has type `&mut *mut ()` which is not `Send`, because `*mut ()` is not `Send`
}
