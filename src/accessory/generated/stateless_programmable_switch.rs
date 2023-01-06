// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
	accessory::{AccessoryInformation, HapAccessory},
	service::{HapService, accessory_information::AccessoryInformationService, stateless_programmable_switch::StatelessProgrammableSwitchService},
	HapType,
	Result,
};

/// Stateless Programmable Switch accessory.
#[derive(Debug, Default)]
pub struct StatelessProgrammableSwitchAccessory {
    /// ID of the Stateless Programmable Switch accessory.
    id: u64,

    /// Accessory Information service.
    pub accessory_information: AccessoryInformationService,
    /// Stateless Programmable Switch service.
    pub stateless_programmable_switch: StatelessProgrammableSwitchService,
}

impl StatelessProgrammableSwitchAccessory {
    /// Creates a new Stateless Programmable Switch accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;
        let stateless_programmable_switch_id = accessory_information.get_characteristics().len() as u64;
        let mut stateless_programmable_switch = StatelessProgrammableSwitchService::new(1 + stateless_programmable_switch_id + 1, id);
        stateless_programmable_switch.set_primary(true);

        Ok(Self {
            id,
            accessory_information,
            stateless_programmable_switch,
        })
    }
}

impl HapAccessory for StatelessProgrammableSwitchAccessory {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_service(&self, hap_type: HapType) -> Option<&dyn HapService> {
        self.get_services()
            .into_iter()
            .find(|&service| service.get_type() == hap_type)
    }

    fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn HapService> {
        self.get_mut_services()
            .into_iter()
            .find(|service| service.get_type() == hap_type)
    }

    fn get_services(&self) -> Vec<&dyn HapService> {
        vec![
            &self.accessory_information,
            &self.stateless_programmable_switch,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![
            &mut self.accessory_information,
            &mut self.stateless_programmable_switch,
        ]
    }
}

impl Serialize for StatelessProgrammableSwitchAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}
