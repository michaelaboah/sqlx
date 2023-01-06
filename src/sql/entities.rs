pub mod enums {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, sqlx::Type, Deserialize, Serialize)]
    #[repr(i32)]
    pub enum Categories {
        CONSOLE,
        PROCESSOR,
        MONITORING,
        SPEAKER,
        AMPLIFIER,
        COMPUTER,
        NETWORK,
        RADIO,
        MICROPHONES,
        SPK_HARDWARE,
        GENERIC,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum MidiType {
        USB,
        SERIAL,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Analog {
        XlrAnalog,
        XlrDigital,
        Ts,
        Trs,
        Trrs,
        TriPinPhoenix,
        DualPinPhoenix,
        Nl2,
        Nl4,
        Nl8,
        Dc12v,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum NetworkType {
        Ethernet,
        Wifi,
        Cellular,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub enum Protocol {
        DANTE,
        AES_67,
        AVB,
        AVB_MILAN,
        OPTOCORE,
        ULTRANET,
        A_NET,
        IP,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub enum SampleRate {
        SD,
        HD,
        UHD,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ComputerPortType {
        USB_A,
        USB_B,
        USB_C,
        HDMI,
        MINI_HDMI,
        DISPLAYPORT,
        MINI_DISPLAYPORT,
        MIRCO_B,
        SD_CARD,
        FIREWIRE,
        USB_C_THUNDERBOLT,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum TransmitterConnector {
        SHURE_TA4,
        MICRODOT,
        TRRS,
        TRI_PIN,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum MicrophoneType {
        PRE_POLORAIZED_CONDENSOR,
        CONDENSOR,
        RIBBON,
        DYNAMIC,
    }
}

pub mod structs {
    use serde::{Deserialize, Serialize};

    use super::enums::*;
    use super::field_structs::*;

    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize, sqlx::Type, sqlx::Encode)]
    pub struct Item {
        id: i32,
        created_at: String,
        updated_at: String,
        public_notes: Option<String>,
        cost: f32,
        weight: f32,
        dimensions: Dimension,
        model: String,
        category: Categories,
        amplifier: Option<AmplifierItem>,
        pub console: Option<ConsoleItem>,
        computer: Option<ComputerItem>,
        processor: Option<ProcessingItem>,
        network_item: Option<NetworkItem>,
        microphone: Option<MicrophoneItem>,
        radio_item: Option<RFItem>,
        speaker_item: Option<SpeakerItem>,
        monitoring_item: Option<MonitoringItem>,
        notes: Option<Vec<String>>,
    }

    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct ConsoleItem {
        id: i32,
        total_inputs: i32,
        total_outputs: i32,
        total_busses: i32,
        physical_inputs: i32,
        physical_outputs: i32,
        aux_inputs: i32,
        physical_aux_inputs: i32,
        phantom_power_inputs: i32,
        faders: i32,
        motorized: bool,
        midi: MidiType,
        protocol_inputs: i32,
        signal_protocol: Protocol,
        can_expand: bool,
        max_sample_rate: SampleRate,
        power: Power,
    }

    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct AmplifierItem {
        id: i32,
        total_inputs: i32,
        total_outputs: i32,
        midi: MidiType,
        physical_connectivity: Vec<PhysicalPort>,
        network_connectivity: Vec<NetworkPort>,
        signal_protocol: Protocol,
        max_sample_rate: SampleRate,
        power: Power,
    }

    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct ComputerItem {
        id: i32,
        cpu_processor: String,
        ram_size: i32,
        total_storage: i32,
        model_year: String,
        operating_system: String,
        dedicated_graphics: bool,
        network_connectivity: Vec<NetworkPort>,
        computer_ports: Vec<ComputerPort>,
        power: Power,
    }
    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    struct ProcessingItem {
        id: i32,
        total_inputs: i32,
        total_outputs: i32,
        physical_inputs: i32,
        physical_outputs: i32,
        midi: MidiType,
        protocol_inputs: i32,
        signal_protocol: Protocol,
        max_sample_rate: SampleRate,
        network_connectivity: Vec<NetworkPort>,
        physical_connectivity: Vec<PhysicalPort>,
        power: Power,
    }

    // Structs
    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct NetworkItem {
        id: i32,
        network_type: NetworkType,
        poe_ports: i32,
        max_speed: i32,
        fiber: bool,
        network_connectivity: Vec<NetworkPort>,
        power: Power,
    }
    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct MicrophoneItem {
        id: i32,
        max_spl: f32,
        phantom: bool,
        low_cut: bool,
        pad: bool,
        diaphragm_size: f32,
        output_impedance: f32,
        frequency_response: String,
        connector: Analog,
        microphone_type: Vec<MicrophoneType>,
    }
    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct SpeakerItem {
        id: i32,
        driver: DriverArrangment,
        built_in_processing: bool,
        wireless: bool,
        max_spl: f32,
        power: Power,
        lower_frequency_response: i32,
        upper_frequency_response: i32,
        mounting_options: Vec<String>,
        physical_connectivity: Vec<PhysicalPort>,
        network_connectivity: Vec<NetworkPort>,
    }
    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct MonitoringItem {
        id: i32,
        distro: bool,
        network_connectivity: Vec<NetworkPort>,
        physical_connectivity: Vec<PhysicalPort>,
        power: Power,
    }

    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct RFItem {
        id: i32,
        physical_range: i32,
        lower_frequency_response: i32,
        upper_frequency_response: i32,
        transmitter: Transmitter,
        reciever: Reciever,
    }
}

pub mod field_structs {
    use super::enums::*;
    use serde::{Deserialize, Serialize};
    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct Dimension {
        width: f32,
        length: f32,
        height: f32,
        rack_unit: Option<f32>,
    }

    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct PhysicalPort {
        port_identifier: String,
        connector_type: Analog,
        signal_lines: i32,
        input: bool,
    }
    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct NetworkPort {
        port_identifier: String,
        port_type: NetworkType,
        input: bool,
    }
    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct Power {
        voltage: Option<f32>,
        current: Option<f32>,
        frequency: Option<f32>,
    }
    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct Transmitter {
        connector: TransmitterConnector,
    }
    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct Reciever {
        network_ports: Vec<NetworkPort>,
        physical_ports: Vec<PhysicalPort>,
        power: Power,
    }

    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct DriverArrangment {
        speaker_size: f32,
    }

    #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
    pub struct ComputerPort {
        port_type: ComputerPortType,
        number_of_ports: i32,
        front_port: bool,
        version: String,
    }
}

pub mod struct_parsing {
    use serde::Deserialize;

    pub fn parse_item<T>(json_string: &str) -> Result<T, serde_json::Error>
    where
        T: for<'a> Deserialize<'a>,
    {
        serde_json::from_str(json_string)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use serde::Deserialize;

        #[derive(Debug, Deserialize, PartialEq)]
        struct Person {
            name: String,
            age: u32,
        }

        #[test]
        fn test_parse_item() {
            let json_string = r#"{"name": "John", "age": 30}"#;
            let expected = Person {
                name: "John".to_string(),
                age: 30,
            };

            let result = parse_item::<Person>(json_string);
            assert!(result.is_ok());
            let person = result.unwrap();
            assert_eq!(person, expected);
        }
    }
}
