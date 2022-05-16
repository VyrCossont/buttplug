// Buttplug Rust Source Code File - See https://buttplug.io for more info.
//
// Copyright 2016-2022 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.

mod util;
use util::test_server_with_device;
use buttplug::{
  core::{
    errors::{ButtplugDeviceError, ButtplugError},
    messages::{
      self,
      ButtplugDeviceMessageType,
      ButtplugServerMessage,
      BUTTPLUG_CURRENT_MESSAGE_SPEC_VERSION,
      Endpoint
    },
  },
  server::{ButtplugServerBuilder, device::hardware::communication::test::TestDeviceCommunicationManagerBuilder},
  util::async_manager,
};
use futures::{pin_mut, StreamExt};
use std::matches;

// Test devices that have protocols that support movements not all devices do.
// For instance, the Onyx+ is part of a protocol that supports vibration, but
// the device itself does not.
#[test]
fn test_capabilities_exposure() {
  async_manager::block_on(async {
    let (server, _) = test_server_with_device("Onyx+").await;
    let recv = server.event_stream();
    pin_mut!(recv);

    server
      .parse_message(
        messages::RequestServerInfo::new("Test Client", BUTTPLUG_CURRENT_MESSAGE_SPEC_VERSION)
          .into(),
      )
      .await
      .expect("Test, assuming infallible.");
    server
      .parse_message(messages::StartScanning::default().into())
      .await
      .expect("Test, assuming infallible.");
    while let Some(msg) = recv.next().await {
      if let ButtplugServerMessage::DeviceAdded(device) = msg {
        assert!(!device
          .device_messages()
          .contains_key(&ButtplugDeviceMessageType::VibrateCmd));
        assert!(!device
          .device_messages()
          .contains_key(&ButtplugDeviceMessageType::SingleMotorVibrateCmd));
        assert!(device
          .device_messages()
          .contains_key(&ButtplugDeviceMessageType::LinearCmd));
        assert!(device
          .device_messages()
          .contains_key(&ButtplugDeviceMessageType::StopDeviceCmd));
        return;
      }
    }
  });
}

#[test]
fn test_server_raw_message() {
  async_manager::block_on(async {
    let mut server_builder = ButtplugServerBuilder::default();
    let builder = TestDeviceCommunicationManagerBuilder::default();
    let helper = builder.helper();
    server_builder
      .allow_raw_messages(true)
      .device_manager_builder()
      .comm_manager(builder);
    let server = server_builder.finish().unwrap();
    helper
      .add_ble_device("Massage Demo")
      .await;
    let recv = server.event_stream();
    pin_mut!(recv);
    assert!(server
      .parse_message(
        messages::RequestServerInfo::new("Test Client", BUTTPLUG_CURRENT_MESSAGE_SPEC_VERSION)
          .into()
      )
      .await
      .is_ok());
    assert!(server
      .parse_message(messages::StartScanning::default().into())
      .await
      .is_ok());
    while let Some(msg) = recv.next().await {
      if let ButtplugServerMessage::ScanningFinished(_) = msg {
        continue;
      } else if let ButtplugServerMessage::DeviceAdded(da) = msg {
        assert!(da
          .device_messages()
          .contains_key(&ButtplugDeviceMessageType::RawReadCmd));
        assert!(da
          .device_messages()
          .contains_key(&ButtplugDeviceMessageType::RawWriteCmd));
        assert!(da
          .device_messages()
          .contains_key(&ButtplugDeviceMessageType::RawSubscribeCmd));
        assert!(da
          .device_messages()
          .contains_key(&ButtplugDeviceMessageType::RawUnsubscribeCmd));
        assert_eq!(da.device_name(), "Aneros Vivi (Raw Messages Allowed)");
        return;
      } else {
        panic!(
          "Returned message was not a DeviceAdded message or timed out: {:?}",
          msg
        );
      }
    }
  });
}

#[test]
fn test_server_no_raw_message() {
  async_manager::block_on(async {
    let (server, _) = test_server_with_device("Massage Demo").await;
    let recv = server.event_stream();
    pin_mut!(recv);
    assert!(server
      .parse_message(
        messages::RequestServerInfo::new("Test Client", BUTTPLUG_CURRENT_MESSAGE_SPEC_VERSION)
          .into()
      )
      .await
      .is_ok());
    assert!(server
      .parse_message(messages::StartScanning::default().into())
      .await
      .is_ok());
    while let Some(msg) = recv.next().await {
      if let ButtplugServerMessage::ScanningFinished(_) = msg {
        continue;
      } else if let ButtplugServerMessage::DeviceAdded(da) = msg {
        assert_eq!(da.device_name(), "Aneros Vivi");
        assert!(!da
          .device_messages()
          .contains_key(&ButtplugDeviceMessageType::RawReadCmd));
        assert!(!da
          .device_messages()
          .contains_key(&ButtplugDeviceMessageType::RawWriteCmd));
        assert!(!da
          .device_messages()
          .contains_key(&ButtplugDeviceMessageType::RawSubscribeCmd));
        assert!(!da
          .device_messages()
          .contains_key(&ButtplugDeviceMessageType::RawUnsubscribeCmd));
        return;
      } else {
        panic!(
          "Returned message was not a DeviceAdded message or timed out: {:?}",
          msg
        );
      }
    }
  });
}

#[test]
fn test_reject_on_no_raw_message() {
  async_manager::block_on(async {
    let (server, _) = test_server_with_device("Massage Demo").await;
    let recv = server.event_stream();
    pin_mut!(recv);
    assert!(server
      .parse_message(
        messages::RequestServerInfo::new("Test Client", BUTTPLUG_CURRENT_MESSAGE_SPEC_VERSION)
          .into()
      )
      .await
      .is_ok());
    assert!(server
      .parse_message(messages::StartScanning::default().into())
      .await
      .is_ok());
    while let Some(msg) = recv.next().await {
      if let ButtplugServerMessage::ScanningFinished(_) = msg {
        continue;
      } else if let ButtplugServerMessage::DeviceAdded(da) = msg {
        assert_eq!(da.device_name(), "Aneros Vivi");
        let mut should_be_err;
        should_be_err = server
          .parse_message(
            messages::RawWriteCmd::new(da.device_index(), Endpoint::Tx, vec![0x0], false).into(),
          )
          .await;
        assert!(should_be_err.is_err());
        assert!(matches!(
          should_be_err.unwrap_err().original_error(),
          ButtplugError::ButtplugDeviceError(ButtplugDeviceError::MessageNotSupported(_))
        ));

        should_be_err = server
          .parse_message(messages::RawReadCmd::new(da.device_index(), Endpoint::Tx, 0, 0).into())
          .await;
        assert!(should_be_err.is_err());
        assert!(matches!(
          should_be_err.unwrap_err().original_error(),
          ButtplugError::ButtplugDeviceError(ButtplugDeviceError::MessageNotSupported(_))
        ));

        should_be_err = server
          .parse_message(messages::RawSubscribeCmd::new(da.device_index(), Endpoint::Tx).into())
          .await;
        assert!(should_be_err.is_err());
        assert!(matches!(
          should_be_err.unwrap_err().original_error(),
          ButtplugError::ButtplugDeviceError(ButtplugDeviceError::MessageNotSupported(_))
        ));

        should_be_err = server
          .parse_message(messages::RawUnsubscribeCmd::new(da.device_index(), Endpoint::Tx).into())
          .await;
        assert!(should_be_err.is_err());
        assert!(matches!(
          should_be_err.unwrap_err().original_error(),
          ButtplugError::ButtplugDeviceError(ButtplugDeviceError::MessageNotSupported(_))
        ));
        return;
      } else {
        panic!(
          "Returned message was not a DeviceAdded message or timed out: {:?}",
          msg
        );
      }
    }
  });
}

#[cfg(target_os = "windows")]
#[ignore="Has weird timeout issues"]
#[test]
fn test_repeated_address_additions() {
  async_manager::block_on(async {
    let mut server_builder = ButtplugServerBuilder::default();
    let builder = TestDeviceCommunicationManagerBuilder::default();
    let helper = builder.helper();
    server_builder
      .device_manager_builder()
      .comm_manager(builder);
    let server = server_builder.finish().unwrap();
    let recv = server.event_stream();
    pin_mut!(recv);
    helper
      .add_ble_device_with_address("Massage Demo", "SameAddress")
      .await;
    helper
      .add_ble_device_with_address("Massage Demo", "SameAddress")
      .await;
    assert!(server
      .parse_message(
        messages::RequestServerInfo::new("Test Client", BUTTPLUG_CURRENT_MESSAGE_SPEC_VERSION)
          .into()
      )
      .await
      .is_ok());
    assert!(server
      .parse_message(messages::StartScanning::default().into())
      .await
      .is_ok());
    let mut device_index = None;
    let mut device_removed_called = true;
    while let Some(msg) = recv.next().await {
      match msg {
        ButtplugServerMessage::ScanningFinished(_) => continue,
        ButtplugServerMessage::DeviceAdded(da) => {
          assert_eq!(da.device_name(), "Aneros Vivi");
          if device_index.is_none() {
            device_index = Some(da.device_index());
          } else {
            assert!(device_removed_called);
            assert_eq!(
              da.device_index(),
              device_index.expect("Test, assuming infallible.")
            );
            return;
          }
        }
        ButtplugServerMessage::DeviceRemoved(dr) => {
          assert_eq!(
            dr.device_index(),
            device_index.expect("Test, assuming infallible.")
          );
          device_removed_called = true;
        }
        _ => {
          panic!(
            "Returned message was not a DeviceAdded message or timed out: {:?}",
            msg
          );
        }
      }
    }
  });
}
