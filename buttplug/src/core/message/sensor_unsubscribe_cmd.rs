// Buttplug Rust Source Code File - See https://buttplug.io for more info.
//
// Copyright 2016-2022 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.

use super::*;
use getset::Getters;
#[cfg(feature = "serialize-json")]
use serde::{Deserialize, Serialize};

#[derive(Debug, ButtplugDeviceMessage, PartialEq, Eq, Clone, Getters)]
#[cfg_attr(feature = "serialize-json", derive(Serialize, Deserialize))]
pub struct SensorUnsubscribeCmd {
  #[cfg_attr(feature = "serialize-json", serde(rename = "Id"))]
  id: u32,
  #[cfg_attr(feature = "serialize-json", serde(rename = "DeviceIndex"))]
  device_index: u32,
  #[cfg_attr(feature = "serialize-json", serde(rename = "SensorIndex"))]
  #[getset(get = "pub")]
  sensor_index: u32,
  #[cfg_attr(feature = "serialize-json", serde(rename = "SensorType"))]
  #[getset(get = "pub")]
  sensor_type: SensorType,
}

impl SensorUnsubscribeCmd {
  pub fn new(device_index: u32, sensor_index: u32, sensor_type: SensorType) -> Self {
    Self {
      id: 1,
      device_index,
      sensor_index,
      sensor_type,
    }
  }
}

impl ButtplugMessageValidator for SensorUnsubscribeCmd {
  fn is_valid(&self) -> Result<(), ButtplugMessageError> {
    self.is_not_system_id(self.id)
  }
}
