pub mod enums {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, sqlx::Type, Deserialize, PartialEq, Serialize, Clone, Copy)]
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
        // SPK_HARDWARE,
    }

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq, sqlx::Type, Clone, Copy)]
    pub enum MidiType {
        USB,
        #[default]
        SERIAL,
    }

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq, sqlx::Type, Clone, Copy)]
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

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq, sqlx::Type, Clone, Copy)]
    pub enum NetworkType {
        #[default]
        Ethernet,
        Wifi,
        Cellular,
    }
    #[derive(Debug, Default, Serialize, Deserialize, PartialEq, sqlx::Type, Clone, Copy)]
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
    #[derive(Debug, Default, Serialize, Deserialize, PartialEq, sqlx::Type, Clone, Copy)]
    pub enum SampleRate {
        #[default]
        HD,
        SD,
        UHD,
    }

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq, sqlx::Type, Clone, Copy)]
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

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq, sqlx::Type, Clone, Copy)]
    pub enum TransmitterConnector {
        #[default]
        TRRS,
        SHURE_TA4,
        MICRODOT,
        TRI_PIN,
    }

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq, sqlx::Type, Clone, Copy)]
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
    use sqlx::Decode;

    use super::enums::*;
    use super::field_structs::*;

    #[derive(Debug, Default, sqlx::FromRow, Decode, Serialize, Deserialize, PartialEq, Clone)]
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

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct ConsoleItem {
        pub id: i32,
        pub total_inputs: i32,
        pub total_outputs: i32,
        pub total_busses: i32,
        pub physical_inputs: i32,
        pub physical_outputs: i32,
        pub aux_inputs: i32,
        pub physical_aux_inputs: i32,
        pub phantom_power_inputs: i32,
        pub faders: i32,
        pub motorized: bool,
        pub midi: MidiType,
        pub protocol_inputs: i32,
        pub signal_protocol: Protocol,
        pub can_expand: bool,
        pub max_sample_rate: SampleRate,
        pub power: Power,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct AmplifierItem {
        pub id: i32,
        pub total_inputs: i32,
        pub total_outputs: i32,
        pub midi: MidiType,
        pub physical_connectivity: Vec<PhysicalPort>,
        pub network_connectivity: Vec<NetworkPort>,
        pub signal_protocol: Protocol,
        pub max_sample_rate: SampleRate,
        pub power: Power,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct ComputerItem {
        pub id: i32,
        pub cpu_processor: String,
        pub ram_size: i32,
        pub total_storage: i32,
        pub model_year: String,
        pub operating_system: String,
        pub dedicated_graphics: bool,
        pub network_connectivity: Vec<NetworkPort>,
        pub computer_ports: Vec<ComputerPort>,
        pub power: Power,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct ProcessingItem {
        pub id: i32,
        pub total_inputs: i32,
        pub total_outputs: i32,
        pub physical_inputs: i32,
        pub physical_outputs: i32,
        pub midi: MidiType,
        pub protocol_inputs: i32,
        pub signal_protocol: Protocol,
        pub max_sample_rate: SampleRate,
        pub network_connectivity: Vec<NetworkPort>,
        pub physical_connectivity: Vec<PhysicalPort>,
        pub power: Power,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct NetworkItem {
        pub id: i32,
        pub network_type: NetworkType,
        pub poe_ports: i32,
        pub max_speed: i32,
        pub fiber: bool,
        pub network_connectivity: Vec<NetworkPort>,
        pub power: Power,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct MicrophoneItem {
        pub id: i32,
        pub max_spl: f32,
        pub phantom: bool,
        pub low_cut: bool,
        pub pad: bool,
        pub diaphragm_size: f32,
        pub output_impedance: f32,
        pub frequency_response: String,
        pub connector: Analog,
        pub microphone_type: Vec<MicrophoneType>,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct SpeakerItem {
        pub id: i32,
        pub driver: DriverArrangment,
        pub built_in_processing: bool,
        pub wireless: bool,
        pub max_spl: f32,
        pub power: Power,
        pub lower_frequency_response: i32,
        pub upper_frequency_response: i32,
        pub mounting_options: Vec<String>,
        pub physical_connectivity: Vec<PhysicalPort>,
        pub network_connectivity: Vec<NetworkPort>,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct MonitoringItem {
        pub id: i32,
        pub distro: bool,
        pub network_connectivity: Vec<NetworkPort>,
        pub physical_connectivity: Vec<PhysicalPort>,
        pub power: Power,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct RFItem {
        pub id: i32,
        pub physical_range: i32,
        pub lower_frequency_response: i32,
        pub upper_frequency_response: i32,
        pub transmitter: Transmitter,
        pub reciever: Reciever,
    }
}

pub mod field_structs {
    use super::enums::*;
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct Dimension {
        pub width: f32,
        pub length: f32,
        pub height: f32,
        pub rack_unit: Option<f32>,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct PhysicalPort {
        port_identifier: String,
        connector_type: Analog,
        signal_lines: i32,
        input: bool,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct NetworkPort {
        port_identifier: String,
        port_type: NetworkType,
        input: bool,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct Power {
        pub wattage: i32,
        pub redundant: bool,
        pub lower_voltage: i32,
        pub max_wattage: f32,
        pub input_connector: String,
        pub output_connector: Option<String>,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct Transmitter {
        connector: TransmitterConnector,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct Reciever {
        network_ports: Vec<NetworkPort>,
        physical_ports: Vec<PhysicalPort>,
        power: Power,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct DriverArrangment {
        speaker_size: f32,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct ComputerPort {
        port_type: ComputerPortType,
        number_of_ports: i32,
        front_port: bool,
        version: String,
    }
}

pub mod struct_parsing {
    // use serde::Deserialize;

    // pub fn parse_item<T>(json_string: &str) -> Result<T, serde_json::Error>
    // where
    //     T: for<'a> Deserialize<'a>,
    // {
    //     serde_json::from_str(json_string)
    // }

    #[cfg(test)]
    mod tests {
        // use super::*;
        // use serde::Deserialize;

        // #[derive(Debug, Default, Deserialize, PartialEq)]
        // struct Person {
        //     name: String,
        //     age: u32,
        // }

        // #[test]
        // fn test_parse_item() {
        //     let json_string = r#"{"name": "John", "age": 30}"#;
        //     let expected = Person {
        //         name: "John".to_string(),
        //         age: 30,
        //     };

        //     let result = parse_item::<Person>(json_string);
        //     assert!(result.is_ok());
        //     let person = result.unwrap();
        //     assert_eq!(person, expected);
        // }
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
        pub amplifier_item_id: Option<i32>,
        pub console_item_id: Option<i32>,
        pub computer_item_id: Option<i32>,
        pub processor_item_id: Option<i32>,
        pub network_item_id: Option<i32>,
        pub microphone_item_id: Option<i32>,
        pub radio_item_id: Option<i32>,
        pub speaker_item_id: Option<i32>,
        pub monitoring_item_id: Option<i32>,
        pub notes: Option<serde_json::Value>,
    }

    impl CreateItem {
        pub fn new(item: &Item) -> Self {
            CreateItem {
                created_at: item.created_at.to_owned(),
                updated_at: item.updated_at.to_owned(),
                public_notes: item.public_notes.to_owned(),
                cost: item.cost,
                weight: item.weight,
                model: item.model.to_owned(),
                dimensions: serde_json::to_value(item.dimensions.to_owned()).unwrap_or_default(),
                category: item.category,
                amplifier_item_id: item.amplifier.as_ref().map(|item| item.id),
                // amplifier_item_id: None,
                console_item_id: item.console.as_ref().map(|item| item.id),
                computer_item_id: item.computer.as_ref().map(|item| item.id),
                processor_item_id: item.processor.as_ref().map(|item| item.id),
                network_item_id: item.network_item.as_ref().map(|item| item.id),
                microphone_item_id: item.microphone.as_ref().map(|item| item.id),
                radio_item_id: item.radio_item.as_ref().map(|item| item.id),
                speaker_item_id: item.speaker_item.as_ref().map(|item| item.id),
                monitoring_item_id: item.monitoring_item.as_ref().map(|item| item.id),
                notes: serde_json::to_value(item.notes.to_owned())
                    .map(|thing| Some(thing))
                    .unwrap_or_default(),
            }
        }
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
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

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    struct CreateConsoleItem {
        pub id: i32,
        pub total_inputs: i32,
        pub total_outputs: i32,
        pub total_busses: i32,
        pub physical_inputs: i32,
        pub physical_outputs: i32,
        pub aux_inputs: i32,
        pub physical_aux_inputs: i32,
        pub phantom_power_inputs: i32,
        pub faders: i32,
        pub motorized: bool,
        pub midi: MidiType,
        pub protocol_inputs: i32,
        pub signal_protocol: Protocol,
        pub can_expand: bool,
        pub max_sample_rate: SampleRate,
        pub power: Option<serde_json::Value>,
    }

    impl CreateConsoleItem {
        pub fn new(console: ConsoleItem) -> Self {
            CreateConsoleItem {
                id: console.id,
                total_inputs: todo!(),
                total_outputs: todo!(),
                total_busses: todo!(),
                physical_inputs: todo!(),
                physical_outputs: todo!(),
                aux_inputs: todo!(),
                physical_aux_inputs: todo!(),
                phantom_power_inputs: todo!(),
                faders: todo!(),
                motorized: todo!(),
                midi: todo!(),
                protocol_inputs: todo!(),
                signal_protocol: todo!(),
                can_expand: todo!(),
                max_sample_rate: todo!(),
                power: todo!(),
            }
        }
    }
}
