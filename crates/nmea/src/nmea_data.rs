mod dhv;
mod gga;
mod gll;
mod gsa;
mod vtg;
mod zda;

pub use dhv::*;
pub use gga::*;
pub use gll::*;
pub use gsa::*;
pub use vtg::*;
pub use zda::*;

use crate::utils::readonly_struct;
readonly_struct!(
    Identifier,
    "",
    {navigation_system: crate::NavigationSystem},
    {sentense_type: crate::NmeaType}
);
impl Identifier {
    pub(crate) fn new(
        navigation_system: crate::NavigationSystem,
        sentense_type: crate::NmeaType,
    ) -> Self {
        Self {
            navigation_system,
            sentense_type,
        }
    }
}
