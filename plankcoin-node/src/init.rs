use super::*;

mod settings;

pub use settings::Settings;

#[inline]
#[once(panic)]
pub fn try_build() -> AnyResult<Settings> {
    settings::try_build()
}
