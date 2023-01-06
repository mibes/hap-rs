// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		power_state::PowerStateCharacteristic,
		brightness::BrightnessCharacteristic,
		characteristic_value_active_transition_count::CharacteristicValueActiveTransitionCountCharacteristic,
		characteristic_value_transition_control::CharacteristicValueTransitionControlCharacteristic,
		color_temperature::ColorTemperatureCharacteristic,
		hue::HueCharacteristic,
		name::NameCharacteristic,
		saturation::SaturationCharacteristic,
		supported_characteristic_value_transition_configuration::SupportedCharacteristicValueTransitionConfigurationCharacteristic,
	},
    HapType,
};

/// Lightbulb service.
#[derive(Debug, Default)]
pub struct LightbulbService {
    /// Instance ID of the Lightbulb service.
    id: u64,
    /// [`HapType`](HapType) of the Lightbulb service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

	/// Power State characteristic (required).
	pub power_state: PowerStateCharacteristic,

	/// Brightness characteristic (optional).
	pub brightness: Option<BrightnessCharacteristic>,
	/// Characteristic Value Active Transition Count characteristic (optional).
	pub characteristic_value_active_transition_count: Option<CharacteristicValueActiveTransitionCountCharacteristic>,
	/// Characteristic Value Transition Control characteristic (optional).
	pub characteristic_value_transition_control: Option<CharacteristicValueTransitionControlCharacteristic>,
	/// Color Temperature characteristic (optional).
	pub color_temperature: Option<ColorTemperatureCharacteristic>,
	/// Hue characteristic (optional).
	pub hue: Option<HueCharacteristic>,
	/// Name characteristic (optional).
	pub name: Option<NameCharacteristic>,
	/// Saturation characteristic (optional).
	pub saturation: Option<SaturationCharacteristic>,
	/// Supported Characteristic Value Transition Configuration characteristic (optional).
	pub supported_characteristic_value_transition_configuration: Option<SupportedCharacteristicValueTransitionConfigurationCharacteristic>,
}

impl LightbulbService {
    /// Creates a new Lightbulb service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::Lightbulb,
			power_state: PowerStateCharacteristic::new(id + 1, accessory_id),
			brightness: Some(BrightnessCharacteristic::new(id + 1 + 1, accessory_id)),
			characteristic_value_active_transition_count: Some(CharacteristicValueActiveTransitionCountCharacteristic::new(id + 1 + 1 + 1, accessory_id)),
			characteristic_value_transition_control: Some(CharacteristicValueTransitionControlCharacteristic::new(id + 1 + 2 + 1, accessory_id)),
			color_temperature: Some(ColorTemperatureCharacteristic::new(id + 1 + 3 + 1, accessory_id)),
			hue: Some(HueCharacteristic::new(id + 1 + 4 + 1, accessory_id)),
			name: Some(NameCharacteristic::new(id + 1 + 5 + 1, accessory_id)),
			saturation: Some(SaturationCharacteristic::new(id + 1 + 6 + 1, accessory_id)),
			supported_characteristic_value_transition_configuration: Some(SupportedCharacteristicValueTransitionConfigurationCharacteristic::new(id + 1 + 7 + 1, accessory_id)),
			..Default::default()
        }
    }
}

impl HapService for LightbulbService {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn set_type(&mut self, hap_type: HapType) {
        self.hap_type = hap_type;
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_linked_services(&self) -> Vec<u64> {
        self.linked_services.clone()
    }

    fn set_linked_services(&mut self, linked_services: Vec<u64>) {
        self.linked_services = linked_services;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        self.get_characteristics().into_iter().find(|&characteristic| characteristic.get_type() == hap_type)
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        self.get_mut_characteristics().into_iter().find(|characteristic| characteristic.get_type() == hap_type)
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.power_state,
		];
		if let Some(c) = &self.brightness {
		    characteristics.push(c);
		}
		if let Some(c) = &self.characteristic_value_active_transition_count {
		    characteristics.push(c);
		}
		if let Some(c) = &self.characteristic_value_transition_control {
		    characteristics.push(c);
		}
		if let Some(c) = &self.color_temperature {
		    characteristics.push(c);
		}
		if let Some(c) = &self.hue {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.saturation {
		    characteristics.push(c);
		}
		if let Some(c) = &self.supported_characteristic_value_transition_configuration {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.power_state,
		];
		if let Some(c) = &mut self.brightness {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.characteristic_value_active_transition_count {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.characteristic_value_transition_control {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.color_temperature {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.hue {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.saturation {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.supported_characteristic_value_transition_configuration {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for LightbulbService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}
