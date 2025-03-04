// Buttplug Rust Source Code File - See https://buttplug.io for more info.
//
// Copyright 2016-2022 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.

use crate::{
  core::{
    errors::ButtplugDeviceError,
    message::{ActuatorType, Endpoint},
  },
  server::device::{
    hardware::{HardwareCommand, HardwareWriteCmd},
    protocol::{generic_protocol_setup, ProtocolHandler},
  },
};

generic_protocol_setup!(JeJoue, "jejoue");

#[derive(Default)]
pub struct JeJoue {}

impl ProtocolHandler for JeJoue {
  fn handle_scalar_cmd(
    &self,
    cmds: &[Option<(ActuatorType, u32)>],
  ) -> Result<Vec<HardwareCommand>, ButtplugDeviceError> {
    // Store off result before the match, so we drop the lock ASAP.
    // Default to both vibes
    let mut pattern: u8 = 1;

    // Use vibe 1 as speed
    let mut speed = cmds[0].unwrap_or((ActuatorType::Vibrate, 0)).1 as u8;

    // Unless it's zero, then five vibe 2 a chance
    if speed == 0 {
      speed = cmds[1].unwrap_or((ActuatorType::Vibrate, 0)).1 as u8;

      // If we've vibing on 2 only, then change the pattern
      if speed != 0 {
        pattern = 3;
      }
    }

    // If we've vibing on 1 only, then change the pattern
    if pattern == 1 && speed != 0 && cmds[1].unwrap_or((ActuatorType::Vibrate, 0)).1 == 0 {
      pattern = 2;
    }
    Ok(vec![HardwareWriteCmd::new(
      Endpoint::Tx,
      vec![pattern, speed],
      false,
    )
    .into()])
  }
}

/*
#[cfg(all(test, feature = "server"))]
mod test {
  use crate::{
    core::messages::{Endpoint, StopDeviceCmd, VibrateCmd, VibrateSubcommand},
    server::device::{
      hardware::{HardwareCommand, HardwareWriteCmd},
      hardware::communication::test::{
        check_test_recv_empty,
        check_test_recv_value,
        new_bluetoothle_test_device,
      },
    },
    util::async_manager,
  };

  #[test]
  pub fn test_je_joue_protocol() {
    async_manager::block_on(async move {
      let (device, test_device) = new_bluetoothle_test_device("Je Joue")
        .await
        .expect("Test, assuming infallible");
      let command_receiver = test_device
        .endpoint_receiver(&Endpoint::Tx)
        .expect("Test, assuming infallible");
      device
        .parse_message(VibrateCmd::new(0, vec![VibrateSubcommand::new(0, 0.5)]).into())
        .await
        .expect("Test, assuming infallible");
      // We just vibe 1 so expect 1 write (mode 2)
      check_test_recv_value(
        &command_receiver,
        HardwareCommand::Write(HardwareWriteCmd::new(Endpoint::Tx, vec![2, 3], false)),
      );
      assert!(check_test_recv_empty(&command_receiver));

      device
        .parse_message(VibrateCmd::new(0, vec![VibrateSubcommand::new(0, 0.5)]).into())
        .await
        .expect("Test, assuming infallible");
      // no-op
      assert!(check_test_recv_empty(&command_receiver));

      device
        .parse_message(
          VibrateCmd::new(
            0,
            vec![
              VibrateSubcommand::new(0, 0.1),
              VibrateSubcommand::new(1, 0.5),
            ],
          )
          .into(),
        )
        .await
        .expect("Test, assuming infallible");
      // setting second vibe whilst changing vibe 1, 1 writes (mode 1)
      check_test_recv_value(
        &command_receiver,
        HardwareCommand::Write(HardwareWriteCmd::new(Endpoint::Tx, vec![1, 1], false)),
      );
      assert!(check_test_recv_empty(&command_receiver));

      device
        .parse_message(
          VibrateCmd::new(
            0,
            vec![
              VibrateSubcommand::new(0, 0.1),
              VibrateSubcommand::new(1, 0.9),
            ],
          )
          .into(),
        )
        .await
        .expect("Test, assuming infallible");
      // only vibe 1 changed, 1 write, same data
      check_test_recv_value(
        &command_receiver,
        HardwareCommand::Write(HardwareWriteCmd::new(Endpoint::Tx, vec![1, 1], false)),
      );
      assert!(check_test_recv_empty(&command_receiver));

      device
        .parse_message(
          VibrateCmd::new(
            0,
            vec![
              VibrateSubcommand::new(0, 0.0),
              VibrateSubcommand::new(1, 0.9),
            ],
          )
          .into(),
        )
        .await
        .expect("Test, assuming infallible");
      // turn off vibe 1, 1 write (mode 3)
      check_test_recv_value(
        &command_receiver,
        HardwareCommand::Write(HardwareWriteCmd::new(Endpoint::Tx, vec![3, 5], false)),
      );
      assert!(check_test_recv_empty(&command_receiver));

      device
        .parse_message(StopDeviceCmd::new(0).into())
        .await
        .expect("Test, assuming infallible");
      // stop on both, 1 write (mode 1)
      check_test_recv_value(
        &command_receiver,
        HardwareCommand::Write(HardwareWriteCmd::new(Endpoint::Tx, vec![1, 0], false)),
      );
      assert!(check_test_recv_empty(&command_receiver));
    });
  }
}
*/
