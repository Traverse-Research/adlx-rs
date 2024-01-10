//! Open the ADLX library, retrieve [`adlx::adlx::system::System`], and call a simple function on it

use adlx::adlx::helper::AdlxHelper;
use anyhow::Result;

fn main() -> Result<()> {
    let helper = AdlxHelper::new()?;
    let system = helper.system();

    dbg!(system);
    let _ = dbg!(system.hybrid_graphics_type());

    Ok(())
}
