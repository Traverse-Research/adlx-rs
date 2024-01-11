//! Open the ADLX library, retrieve [`adlx::adlx::system::System`], and call a simple function on it

use adlx::{gpu::Gpu1, helper::AdlxHelper, interface::Interface};
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
            dbg!(&gpu1);
            if let Ok(gpu1) = gpu1 {
                // Test Deref:
                dbg!(gpu1.name()?);
                // Test new function:
                dbg!(gpu1.product_name()?);
                // Test Deref when passing as argument:
                let gpu_metrics = performance_monitoring_services.current_gpu_metrics(&gpu1)?;
                dbg!(&gpu_metrics);
            }

            let gpu_metrics = performance_monitoring_services.current_gpu_metrics(&gpu)?;

            dbg!(gpu_metrics.usage()?);
            dbg!(gpu_metrics.clock_speed()?);
            dbg!(gpu_metrics.vram_clock_speed()?);
            let _ = dbg!(gpu_metrics.power()); // Not supported???
            dbg!(gpu_metrics.total_board_power()?);
            dbg!(gpu_metrics.voltage()?);
            dbg!(gpu_metrics.vram()?);
            dbg!(gpu_metrics.fan_speed()?);
            dbg!(gpu_metrics.temperature()?);
            let _ = dbg!(gpu_metrics.intake_temperature()); // Not supported???
            dbg!(gpu_metrics.hotspot_temperature()?);
        }

        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}
