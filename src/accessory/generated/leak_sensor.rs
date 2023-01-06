// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
	accessory::{AccessoryInformation, HapAccessory},
	service::{HapService, accessory_information::AccessoryInformationService, leak_sensor::LeakSensorService},
	HapType,
	Result,
};

/// Leak Sensor accessory.
#[derive(Debug, Default)]
pub struct LeakSensorAccessory {
    /// ID of the Leak Sensor accessory.
    id: u64,

    /// Accessory Information service.
    pub accessory_information: AccessoryInformationService,
    /// Leak Sensor service.
    pub leak_sensor: LeakSensorService,
}

impl LeakSensorAccessory {
    /// Creates a new Leak Sensor accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;
        let leak_sensor_id = accessory_information.get_characteristics().len() as u64;
        let mut leak_sensor = LeakSensorService::new(1 + leak_sensor_id + 1, id);
        leak_sensor.set_primary(true);

        Ok(Self {
            id,
            accessory_information,
            leak_sensor,
        })
    }
}

impl HapAccessory for LeakSensorAccessory {
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
            &self.leak_sensor,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![
            &mut self.accessory_information,
            &mut self.leak_sensor,
        ]
    }
}

impl Serialize for LeakSensorAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}
