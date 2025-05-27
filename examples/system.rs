//! Open the ADLX library, retrieve [`adlx::adlx::system::System`], and call a simple function on it

use adlx::{helper::AdlxHelper, Gpu1, Gpu2, Interface};
use anyhow::Result;

fn main() -> Result<()> {
    let helper = AdlxHelper::new()?;
    let system = helper.system();

    dbg!(system);
    let _ = dbg!(system.hybrid_graphics_type());

    let gpu_list = system.gpus()?;
    let gpu_count = gpu_list.size();

    let performance_monitoring_services = system.performance_monitoring_services()?;

    loop {
        for i in 0..gpu_count {
            let gpu = gpu_list.at(i)?;
            println!("GPU #{}: {}", i, gpu.name()?);

            let gpu1 = gpu.cast::<Gpu1>();
            if let Ok(gpu1) = gpu1 {
                // Test Deref:
                dbg!(gpu1.name()?);
                // Test new function:
                dbg!(gpu1.product_name()?);
                // Test Deref when passing as argument:
                let gpu_metrics = performance_monitoring_services.current_gpu_metrics(&gpu1)?;
                dbg!(&gpu_metrics);
            }

            let gpu2 = gpu.cast::<Gpu2>();
            if let Ok(gpu2) = gpu2 {
                dbg!(gpu2.amd_software_release_date()?);
                dbg!(gpu2.amd_software_edition()?);
                dbg!(gpu2.amd_software_version()?);
                dbg!(gpu2.driver_version()?);
                dbg!(gpu2.amd_windows_driver_version()?);
                dbg!(gpu2.luid()?);
            }

            let gpu_metrics = performance_monitoring_services.current_gpu_metrics(&gpu)?;
            let supported_metrics = performance_monitoring_services.supported_gpu_metrics(&gpu)?;

            if supported_metrics.is_supported_gpu_usage()? {
                dbg!(gpu_metrics.usage()?);
            } else {
                println!("usage metrics not supported");
            }
            if supported_metrics.is_supported_gpu_clock_speed()? {
                dbg!(gpu_metrics.clock_speed()?);
            } else {
                println!("clock_speed metrics not supported");
            }
            if supported_metrics.is_supported_gpu_vram_clock_speed()? {
                dbg!(gpu_metrics.vram_clock_speed()?);
            } else {
                println!("vram_clock_speed metrics not supported");
            }
            if supported_metrics.is_supported_gpu_power()? {
                dbg!(gpu_metrics.power()?);
            } else {
                println!("power metrics not supported");
            }
            if supported_metrics.is_supported_gpu_total_board_power()? {
                dbg!(gpu_metrics.total_board_power()?);
            } else {
                println!("total_board_power metrics not supported");
            }
            if supported_metrics.is_supported_gpu_voltage()? {
                dbg!(gpu_metrics.voltage()?);
            } else {
                println!("voltage metrics not supported");
            }
            if supported_metrics.is_supported_gpu_vram()? {
                dbg!(gpu_metrics.vram()?);
            } else {
                println!("vram metrics not supported");
            }
            if supported_metrics.is_supported_gpu_fan_speed()? {
                dbg!(gpu_metrics.fan_speed()?);
            } else {
                println!("fan_speed metrics not supported");
            }
            if supported_metrics.is_supported_gpu_temperature()? {
                dbg!(gpu_metrics.temperature()?);
            } else {
                println!("temperature metrics not supported");
            }
            if supported_metrics.is_supported_gpu_intake_temperature()? {
                dbg!(gpu_metrics.intake_temperature()?);
            } else {
                println!("intake_temperature metrics not supported");
            }
            if supported_metrics.is_supported_gpu_hotspot_temperature()? {
                dbg!(gpu_metrics.hotspot_temperature()?);
            } else {
                println!("hotspot_temperature metrics not supported");
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}
