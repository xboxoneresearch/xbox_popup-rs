use windows::core::HSTRING;
use windows::Data::Xml::Dom::XmlDocument;
use windows::UI;


static IDS: [&str; 19] = [
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.Collection",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.GameDVR",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.Achievements",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.Party",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.GameInvites",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.Tournaments",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.Social",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.Messages",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.Clubs",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.Broadcasts",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.System",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.ActivityAlerts",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.Tests_Legacy",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.NPS",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.ContentBlocks",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.Pins",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.XboxCare",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.Tests_Settings",
    "XboxOneSystemToasts!Windows.Xbox.SystemToasts.Tests",
];

pub fn show_toast() -> Result<(), Box<dyn std::error::Error>> {
    let id = IDS[2];
    let doc_text = 
r#"<toast scenario='rareAchievement'>
    <visual>
        <binding template='ToastGeneric'>
            <text>Collateral Damage</text>
            <text>executed</text>
            <text>Enjoy!</text>
        </binding>
    </visual>
    <actions>
        <action content='Dismiss' arguments='action=dismiss'/>
    </actions>
</toast>"#;
    let encoded_text: Vec<u16> = doc_text.encode_utf16().collect();
    
    let doc = XmlDocument::new()?;
    doc.LoadXml(&HSTRING::from_wide(&encoded_text)?)?;

    let encoded_id: Vec<u16> = id.encode_utf16().collect();
    let notification = UI::Notifications::ToastNotification::CreateToastNotification(&doc)?;
    let notifier = UI::Notifications::ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from_wide(&encoded_id)?)?;
    
    notifier.Show(&notification)?;
    Ok(())
}