// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		security_system_current_state::SecuritySystemCurrentStateCharacteristic,
		security_system_target_state::SecuritySystemTargetStateCharacteristic,
		name::NameCharacteristic,
		security_system_alarm_type::SecuritySystemAlarmTypeCharacteristic,
		status_fault::StatusFaultCharacteristic,
		status_tampered::StatusTamperedCharacteristic,
	},
    HapType,
};

/// Security System service.
#[derive(Debug, Default)]
pub struct SecuritySystemService {
    /// Instance ID of the Security System service.
    id: u64,
    /// [`HapType`](HapType) of the Security System service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

	/// Security System Current State characteristic (required).
	pub security_system_current_state: SecuritySystemCurrentStateCharacteristic,
	/// Security System Target State characteristic (required).
	pub security_system_target_state: SecuritySystemTargetStateCharacteristic,

	/// Name characteristic (optional).
	pub name: Option<NameCharacteristic>,
	/// Security System Alarm Type characteristic (optional).
	pub security_system_alarm_type: Option<SecuritySystemAlarmTypeCharacteristic>,
	/// Status Fault characteristic (optional).
	pub status_fault: Option<StatusFaultCharacteristic>,
	/// Status Tampered characteristic (optional).
	pub status_tampered: Option<StatusTamperedCharacteristic>,
}

impl SecuritySystemService {
    /// Creates a new Security System service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::SecuritySystem,
			security_system_current_state: SecuritySystemCurrentStateCharacteristic::new(id + 1, accessory_id),
			security_system_target_state: SecuritySystemTargetStateCharacteristic::new(id + 1 + 1, accessory_id),
			name: Some(NameCharacteristic::new(id + 1 + 2, accessory_id)),
			security_system_alarm_type: Some(SecuritySystemAlarmTypeCharacteristic::new(id + 1 + 1 + 2, accessory_id)),
			status_fault: Some(StatusFaultCharacteristic::new(id + 1 + 2 + 2, accessory_id)),
			status_tampered: Some(StatusTamperedCharacteristic::new(id + 1 + 3 + 2, accessory_id)),
			..Default::default()
        }
    }
}

impl HapService for SecuritySystemService {
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
			&self.security_system_current_state,
			&self.security_system_target_state,
		];
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.security_system_alarm_type {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_fault {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_tampered {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.security_system_current_state,
			&mut self.security_system_target_state,
		];
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.security_system_alarm_type {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_fault {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_tampered {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for SecuritySystemService {
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
