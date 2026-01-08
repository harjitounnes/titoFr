use anyhow::Result;

pub fn system() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    Ok(())
}
