#[test]
fn initialize() {
    let _ = wgpu::Instance::new(
        wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all),
    );
}

fn request_adapter_inner(power: wgt::PowerPreference) {
    let instance = wgpu::Instance::new(
        wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all),
    );

    let _adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: power,
        compatible_surface: None,
    }))
    .unwrap();
}

#[test]
fn request_adapter_low_power() {
    request_adapter_inner(wgt::PowerPreference::LowPower);
}

#[test]
fn request_adapter_high_power() {
    request_adapter_inner(wgt::PowerPreference::HighPerformance);
}