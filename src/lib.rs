use impersonate::Impersonate;

mod impersonate;
mod process_list;
mod toast;
mod messagedialog;
mod bindings;

/// Show rare-achievement toast
#[no_mangle]
pub unsafe extern "C" fn show_toast() -> bool {
    if let Err(_) = toast::show_toast() {
        return false;
    }

    return true;
}

/// Show error dialog
#[no_mangle]
pub unsafe extern "C" fn show_dialog() -> bool {
    if let Err(_) = messagedialog::show_error_dialog() {
        return false;
    }

    return true;
}

/// Impersonate
#[no_mangle]
pub unsafe extern "C" fn impersonate() -> bool {
    let mut impersonator = Impersonate::create();

    if let Err(_) = impersonator.do_impersonate("XboxUI.exe") {
        return false;
    }

    return true;
}

/// Revert impersonation
#[no_mangle]
pub unsafe extern "C" fn impersonate_revert() -> bool {
    if let Err(_) = Impersonate::revert_to_self() {
        return false;
    }

    return true;
}

/// Get username
#[no_mangle]
pub unsafe extern "C" fn get_username(out_username: *mut u8, out_username_len: &mut u32) -> bool {
    match Impersonate::get_username() {
        Ok(username) => {
            let byts =username.as_bytes();
            *out_username_len = byts.len() as u32;
            std::ptr::copy(byts.as_ptr(), out_username, byts.len());

            true
        },
        Err(_) => false,
    }
}

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
