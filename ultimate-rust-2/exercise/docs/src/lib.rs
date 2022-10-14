//! # Pumpkins
//!
//! A pumpkin is a cultivar of winter squash that is round with smooth, slightly ribbed skin, and is most often deep
//! yellow to orange in coloration.[1] The thick shell contains the seeds and pulp. The name is most commonly used for
//! cultivars of Cucurbita pepo, but some cultivars of Cucurbita maxima, C. argyrosperma, and C. moschata with similar
//! appearance are also sometimes called "pumpkins".
//!
//! ![pumpkins](https://upload.wikimedia.org/wikipedia/commons/thumb/5/5c/FrenchMarketPumpkinsB.jpg/620px-FrenchMarketPumpkinsB.jpg)

/// Big orange thing
///
/// # Recipes
/// Coming soon.
pub struct Pumpkin {
    /// Percentage of roundness
    pub roundness: f32,
    /// Number from 8 to 27
    pub orangeness: i32,
}

impl Pumpkin {
    /// If you smash the pumpkin, it will be gone. Then it can't be used for pie. 😢
    pub fn smash(self) {}
}

/// Color of the [`Pumpkin`]
pub const BURNT_ORANGE: i32 = 13;

/// For internal use only. In fact, this documentation is so private that it won't be generated.
/// At least not by default. But if you pass `--document-private-items` in, it will magically appear!
#[allow(dead_code)] // to silence the warning
enum PrivateEnum {
    /// For Halloween. To be lit by candlelight.
    JackOLantern,
    /// For dessert during North American winter holidays.
    PumpkinPie,
}
