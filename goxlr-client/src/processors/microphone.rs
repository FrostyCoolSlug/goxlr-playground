use anyhow::Result;

use goxlr_ipc::client::Client;
use goxlr_ipc::commands::mic::compressor::CompressorCommand;
use goxlr_ipc::commands::mic::equaliser::{
    EqualiserCommand, FullEqualiserCommand, MiniEqualiserCommand, SetFullFrequency, SetFullGain,
    SetMiniFrequency, SetMiniGain,
};
use goxlr_ipc::commands::mic::gate::GateCommand;
use goxlr_ipc::commands::mic::setup::SetupCommand;
use goxlr_ipc::commands::mic::MicrophoneCommand;
use goxlr_ipc::commands::{DaemonRequest, DeviceCommand, GoXLRCommand};

use crate::cli::{
    MicrophoneCommands, MicrophoneCompressorCommands, MicrophoneEqCommands,
    MicrophoneEqFullCommands, MicrophoneEqMiniCommands, MicrophoneGateCommands,
    MicrophoneSetupCommands,
};

pub async fn handle_microphone(
    serial: String,
    client: Box<dyn Client>,
    command: MicrophoneCommands,
) -> Result<()> {
    match command {
        MicrophoneCommands::SetUp { command } => {
            handle_mic_setup_command(serial, client, command).await?
        }
        MicrophoneCommands::Compressor { command } => {
            handle_mic_compressor_command(serial, client, command).await?
        }
        MicrophoneCommands::Gate { command } => {
            handle_mic_gate_command(serial, client, command).await?
        }
        MicrophoneCommands::Equaliser { command } => {
            handle_mic_eq_command(serial, client, command).await?;
        }
    }
    Ok(())
}

pub async fn handle_mic_setup_command(
    serial: String,
    mut client: Box<dyn Client>,
    command: MicrophoneSetupCommands,
) -> Result<()> {
    match command {
        MicrophoneSetupCommands::MicType { microphone_type } => {
            let command = SetupCommand::SetMicType(microphone_type);
            let command = MicrophoneCommand::Setup(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }

        MicrophoneSetupCommands::MicGain { gain } => {
            let command = SetupCommand::SetMicGain(gain);
            let command = MicrophoneCommand::Setup(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
    }

    Ok(())
}

pub async fn handle_mic_compressor_command(
    serial: String,
    mut client: Box<dyn Client>,
    command: MicrophoneCompressorCommands,
) -> Result<()> {
    match command {
        MicrophoneCompressorCommands::Threshold { threshold } => {
            let command = CompressorCommand::SetThreshold(threshold);
            let command = MicrophoneCommand::Compressor(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
        MicrophoneCompressorCommands::Ratio { ratio } => {
            let command = CompressorCommand::SetRatio(ratio);
            let command = MicrophoneCommand::Compressor(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
        MicrophoneCompressorCommands::Attack { attack } => {
            let command = CompressorCommand::SetAttack(attack);
            let command = MicrophoneCommand::Compressor(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
        MicrophoneCompressorCommands::Release { release } => {
            let command = CompressorCommand::SetRelease(release);
            let command = MicrophoneCommand::Compressor(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
        MicrophoneCompressorCommands::MakupGain { gain } => {
            let command = CompressorCommand::SetMakeupGain(gain);
            let command = MicrophoneCommand::Compressor(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
    }

    Ok(())
}

pub async fn handle_mic_gate_command(
    serial: String,
    mut client: Box<dyn Client>,
    command: MicrophoneGateCommands,
) -> Result<()> {
    match command {
        MicrophoneGateCommands::Enabled { enabled } => {
            let command = GateCommand::SetEnabled(enabled);
            let command = MicrophoneCommand::Gate(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
        MicrophoneGateCommands::Threshold { threshold } => {
            let command = GateCommand::SetThreshold(threshold);
            let command = MicrophoneCommand::Gate(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
        MicrophoneGateCommands::Attack { attack } => {
            let command = GateCommand::SetAttack(attack);
            let command = MicrophoneCommand::Gate(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
        MicrophoneGateCommands::Release { release } => {
            let command = GateCommand::SetRelease(release);
            let command = MicrophoneCommand::Gate(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
        MicrophoneGateCommands::Attenuation { attenuation } => {
            let command = GateCommand::SetAttenuation(attenuation);
            let command = MicrophoneCommand::Gate(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
    }
    Ok(())
}

pub async fn handle_mic_eq_command(
    serial: String,
    mut client: Box<dyn Client>,
    command: MicrophoneEqCommands,
) -> Result<()> {
    match command {
        MicrophoneEqCommands::Full { command } => {
            handle_mic_full_eq_command(serial, client, command).await?;
        }
        MicrophoneEqCommands::Mini { command } => {
            handle_mic_mini_eq_command(serial, client, command).await?;
        }
    }
    Ok(())
}

pub async fn handle_mic_mini_eq_command(
    serial: String,
    mut client: Box<dyn Client>,
    command: MicrophoneEqMiniCommands,
) -> Result<()> {
    match command {
        MicrophoneEqMiniCommands::Frequency { base, frequency } => {
            let command = MiniEqualiserCommand::SetFrequency(SetMiniFrequency { base, frequency });
            let command = EqualiserCommand::Mini(command);
            let command = MicrophoneCommand::Equaliser(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
        MicrophoneEqMiniCommands::Gain { base, gain } => {
            let command = MiniEqualiserCommand::SetGain(SetMiniGain { base, gain });
            let command = EqualiserCommand::Mini(command);
            let command = MicrophoneCommand::Equaliser(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
    }
    Ok(())
}

pub async fn handle_mic_full_eq_command(
    serial: String,
    mut client: Box<dyn Client>,
    command: MicrophoneEqFullCommands,
) -> Result<()> {
    match command {
        MicrophoneEqFullCommands::Frequency { base, frequency } => {
            let command = FullEqualiserCommand::SetFrequency(SetFullFrequency { base, frequency });
            let command = EqualiserCommand::Full(command);
            let command = MicrophoneCommand::Equaliser(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
        MicrophoneEqFullCommands::Gain { base, gain } => {
            let command = FullEqualiserCommand::SetGain(SetFullGain { base, gain });
            let command = EqualiserCommand::Full(command);
            let command = MicrophoneCommand::Equaliser(command);
            let command = GoXLRCommand::Microphone(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
    }
    Ok(())
}
