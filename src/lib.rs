mod impersonate;
mod process_list;
mod toast;
mod messagedialog;
mod bindings;
mod exports;

/* 
fn main() {
    println!("Before: {:?}", Impersonate::get_username());

    let mut imp = Impersonate::create();

    if let Err(e) = imp.do_impersonate("XboxUI.exe") {
        println!("Failed to impersonate, err: {e:?}");
        return;
    }

    println!("Impersonated username: {:?}", Impersonate::get_username());

    println!("Do toast");
    let _ = toast::show_toast();

    Impersonate::revert_to_self().unwrap();
    println!("After impersonation: {:?}", Impersonate::get_username());

}
*/
