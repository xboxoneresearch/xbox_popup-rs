use windows::core::h;

use crate::bindings::Xbox;

pub fn show_error_dialog() -> Result<(), Box<dyn std::error::Error>> {
    let dialog = windows::UI::Popups::MessageDialog::CreateWithTitle(
        h!("Was only able to cause a tiny damage.\n\nFor the possibility of *Collateral Damage*, reboot the console and execute the exploit again!"),
        h!("Exploit failed")
    )?;
    let mut commands = dialog.Commands();
    let commands_mut = commands
        .as_mut()
        .map_err(|e|e.to_owned())?;

    let reboot_action = windows::UI::Popups::UICommand::CreateWithHandler(h!("Reboot"), &windows::UI::Popups::UICommandInvokedHandler::new(|_| {
        println!("Rebooting...");
        Xbox::System::Internal::Power::PowerProperties::RestartConsole()?;
        Ok(())
    }))?;
    let do_nothing_action = windows::UI::Popups::UICommand::CreateWithHandler(h!("Continue"), &windows::UI::Popups::UICommandInvokedHandler::new(|_| {
        println!("Doing nothing...");
        Ok(())
    }))?;

    commands_mut.Append(&reboot_action)?;
    commands_mut.Append(&do_nothing_action)?;
    
    let _ = dialog.ShowAsync()?.get()?;

    Ok(())
}