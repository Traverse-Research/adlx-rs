//! Open the ADLX library, retrieve [`adlx::adlx::system::System`], and call a simple function on it

use adlx::adlx::{helper::AdlxHelper, performance_monitoring_services};
use anyhow::Result;

fn main() -> Result<()> {
    let helper = AdlxHelper::new()?;
    let system = helper.system();

    dbg!(system);
    let _ = dbg!(system.hybrid_graphics_type());

    let gpu_list = system.get_gpus()?;
    let gpu_count = gpu_list.size();

    let performance_monitoring_services = system.performance_monitoring_services()?;

    loop {
        for i in 0..gpu_count {
            let gpu = gpu_list.gpu_at(i)?;
            println!("GPU #{}: {}", i, gpu.name()?);
    
    
            let gpu_metrics = performance_monitoring_services.current_gpu_metrics(&gpu)?;
    
            dbg!(gpu_metrics.usage()?);
            dbg!(gpu_metrics.clock_speed()?);
            dbg!(gpu_metrics.vram_clock_speed()?);
            // dbg!(gpu_metrics.power()?); // Not supported???
            dbg!(gpu_metrics.total_board_power()?);
            dbg!(gpu_metrics.voltage()?);
            dbg!(gpu_metrics.vram()?);
            dbg!(gpu_metrics.fan_speed()?);
            dbg!(gpu_metrics.temperature()?);
            // dbg!(gpu_metrics.intake_temperature()?); // Not supported???
            dbg!(gpu_metrics.hotspot_temperature()?);
        }

        std::thread::sleep(std::time::Duration::from_millis(2000));
    }

    Ok(())
}
