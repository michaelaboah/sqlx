pub mod enums {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, sqlx::Type, Deserialize, PartialEq, Serialize)]
    #[repr(i32)]
    pub enum Categories {
        #[default]
        GENERIC,
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
    }

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    pub enum MidiType {
        USB,
        #[default]
        SERIAL,
    }

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    pub enum Analog {
        #[default]
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

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    pub enum NetworkType {
        #[default]
        Ethernet,
        Wifi,
        Cellular,
    }
    #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    pub enum Protocol {
        #[default]
        IP,
        DANTE,
        AES_67,
        AVB,
        AVB_MILAN,
        OPTOCORE,
        ULTRANET,
        A_NET,
    }
    #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    pub enum SampleRate {
        #[default]
        HD,
        SD,
        UHD,
    }

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    pub enum ComputerPortType {
        #[default]
        USB_B,
        USB_A,
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

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    pub enum TransmitterConnector {
        #[default]
        TRRS,
        SHURE_TA4,
        MICRODOT,
        TRI_PIN,
    }

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    pub enum MicrophoneType {
        #[default]
        DYNAMIC,
        PRE_POLORAIZED_CONDENSOR,
        CONDENSOR,
        RIBBON,
    }
}

pub mod structs {
    use serde::{Deserialize, Serialize};

    use super::enums::*;
    use super::field_structs::*;

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct Item {
        pub id: i32,
        pub created_at: String,
        pub updated_at: String,
        pub public_notes: Option<String>,
        pub cost: f32,
        pub weight: f32,
        pub dimensions: Dimension,
        pub model: String,
        pub category: Categories,
        pub amplifier: Option<AmplifierItem>,
        pub console: Option<ConsoleItem>,
        pub computer: Option<ComputerItem>,
        pub processor: Option<ProcessingItem>,
        pub network_item: Option<NetworkItem>,
        pub microphone: Option<MicrophoneItem>,
        pub radio_item: Option<RFItem>,
        pub speaker_item: Option<SpeakerItem>,
        pub monitoring_item: Option<MonitoringItem>,
        pub notes: Option<Vec<String>>,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct ConsoleItem {
        pub id: i32,
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

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct AmplifierItem {
        pub id: i32,
        total_inputs: i32,
        total_outputs: i32,
        midi: MidiType,
        physical_connectivity: Vec<PhysicalPort>,
        network_connectivity: Vec<NetworkPort>,
        signal_protocol: Protocol,
        max_sample_rate: SampleRate,
        power: Power,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct ComputerItem {
        pub id: i32,
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
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct ProcessingItem {
        pub id: i32,
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

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct NetworkItem {
        pub id: i32,
        network_type: NetworkType,
        poe_ports: i32,
        max_speed: i32,
        fiber: bool,
        network_connectivity: Vec<NetworkPort>,
        power: Power,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct MicrophoneItem {
        pub id: i32,
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
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct SpeakerItem {
        pub id: i32,
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
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct MonitoringItem {
        pub id: i32,
        distro: bool,
        network_connectivity: Vec<NetworkPort>,
        physical_connectivity: Vec<PhysicalPort>,
        power: Power,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct RFItem {
        pub id: i32,
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
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct Dimension {
        pub width: f32,
        pub length: f32,
        pub height: f32,
        pub rack_unit: Option<f32>,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct PhysicalPort {
        port_identifier: String,
        connector_type: Analog,
        signal_lines: i32,
        input: bool,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct NetworkPort {
        port_identifier: String,
        port_type: NetworkType,
        input: bool,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct Power {
        voltage: Option<f32>,
        current: Option<f32>,
        frequency: Option<f32>,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct Transmitter {
        connector: TransmitterConnector,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct Reciever {
        network_ports: Vec<NetworkPort>,
        physical_ports: Vec<PhysicalPort>,
        power: Power,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct DriverArrangment {
        speaker_size: f32,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
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

        #[derive(Debug, Default, Deserialize, PartialEq)]
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

pub mod creation_structs {
    use super::enums::*;
    use super::field_structs::*;
    use super::structs::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, sqlx::FromRow, PartialEq)]
    pub struct CreateItem {
        pub created_at: String,
        pub updated_at: String,
        pub public_notes: Option<String>,
        pub cost: f32,
        pub weight: f32,
        pub dimensions: serde_json::Value,
        pub model: String,
        pub category: Categories,
        pub amplifier: Option<i32>,
        pub console: Option<i32>,
        pub computer: Option<i32>,
        pub processor: Option<i32>,
        pub network_item: Option<i32>,
        pub microphone: Option<i32>,
        pub radio_item: Option<i32>,
        pub speaker_item: Option<i32>,
        pub monitoring_item: Option<i32>,
        pub notes: Option<serde_json::Value>,
    }

    impl CreateItem {
        pub fn new(item: Item) -> Self {
            CreateItem {
                created_at: item.created_at,
                updated_at: item.updated_at,
                public_notes: item.public_notes,
                cost: item.cost,
                weight: item.weight,
                model: item.model,
                dimensions: serde_json::to_value(item.dimensions)
                    .unwrap_or(serde_json::Value::Null),
                category: item.category,
                amplifier: item.amplifier.as_ref().map(|item| item.id),
                console: item.console.as_ref().map(|item| item.id),
                computer: item.computer.as_ref().map(|item| item.id),
                processor: item.processor.as_ref().map(|item| item.id),
                network_item: item.network_item.as_ref().map(|item| item.id),
                microphone: item.microphone.as_ref().map(|item| item.id),
                radio_item: item.radio_item.as_ref().map(|item| item.id),
                speaker_item: item.speaker_item.as_ref().map(|item| item.id),
                monitoring_item: item.monitoring_item.as_ref().map(|item| item.id),
                notes: serde_json::to_value(item.notes)
                    .map(|thing| Some(thing))
                    .unwrap_or(None),
            }
        }
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
    pub struct CreateAmplifierItem {
        pub id: i32,
        pub total_inputs: i32,
        pub total_outputs: i32,
        pub midi: MidiType,
        pub physical_connectivity: serde_json::Value,
        pub network_connectivity: Vec<NetworkPort>,
        pub signal_protocol: Protocol,
        pub max_sample_rate: SampleRate,
        pub power: Power,
    }
}
