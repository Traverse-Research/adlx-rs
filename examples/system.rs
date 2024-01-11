//! Open the ADLX library, retrieve [`adlx::adlx::system::System`], and call a simple function on it

use adlx::adlx::helper::AdlxHelper;
use anyhow::Result;

fn main() -> Result<()> {
    let helper = AdlxHelper::new()?;
    let system = helper.system();

    dbg!(system);
    let _ = dbg!(system.hybrid_graphics_type());

    let gpu_list = system.get_gpus()?;
    let gpu_count = gpu_list.size();
    for i in 0..gpu_count {
        let gpu = gpu_list.gpu_at(i)?;
        println!("\nGPU #{}: {}", i, gpu.name()?)
    }

    Ok(())
}
