use crate::error::Error;
use crate::content::Content;
use crate::content::general::ship::Ship;
//use crate::content::general::voyage::Voyage;
use crate::content::general::itinerary::Itinerary;

pub mod ship;
// mod voyage; TODO
pub mod itinerary;

pub struct General {
    ship: Ship,
    //voyage: Voyage,
    itinerary: Itinerary,
}
//
impl General {
    pub fn new(    
        ship: Ship,
      //  voyage: Voyage,
        itinerary: Itinerary,
    ) -> Self {
        Self {
            ship,
        //    voyage,
            itinerary,
        }
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        Ok( self.ship.to_string()? + "\n" + 
   //         &self.voyage.to_string()? + "\n" + 
            &self.itinerary.to_string()?
        )
    }
}
