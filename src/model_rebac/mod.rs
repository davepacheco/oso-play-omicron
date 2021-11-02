//! Oso wrapper for ReBAC-oriented configuration

use anyhow::Context;
use oso::Oso;

const REBAC_OSO_CONFIG: &str = include_str!("rebac.polar");

pub fn make_oso() -> Result<Oso, anyhow::Error> {
    let mut oso = Oso::new();
    let mut classes = crate::resources::COMMON_RESOURCES.to_vec();
    classes.extend_from_slice(&[]);
    for c in classes {
        oso.register_class(c).context("registering class")?;
    }
    oso.load_str(REBAC_OSO_CONFIG)
        .context("loading built-in Polar (Oso) config")?;
    Ok(oso)
}
