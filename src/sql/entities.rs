pub mod enums {
    use num_derive::FromPrimitive;
    use serde::{Deserialize, Serialize};
    // use std::mem::transmute;

    #[derive(
        Debug, Default, sqlx::Type, Deserialize, PartialEq, Serialize, Clone, Copy, FromPrimitive,
    )]
    #[repr(i64)]
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
    #[derive(
        Debug, Default, sqlx::Type, Deserialize, PartialEq, Serialize, Clone, Copy, FromPrimitive,
    )]
    #[repr(i64)]
    pub enum MidiType {
        USB,
        #[default]
        SERIAL,
    }

    #[derive(
        Debug, Default, sqlx::Type, Deserialize, PartialEq, Serialize, Clone, Copy, FromPrimitive,
    )]
    #[repr(i64)]
    pub enum Analog {
        #[default]
        XLR_ANALOG,
        XLR_DIGITAL,
        TS,
        TRS,
        TRRS,
        TRI_PIN_PHOENIX,
        DUAL_PIN_PHOENIX,
        NL2,
        NL4,
        NL8,
    }

    #[derive(
        Debug, Default, sqlx::Type, Deserialize, PartialEq, Serialize, Clone, Copy, FromPrimitive,
    )]
    #[repr(i64)]
    pub enum NetworkType {
        #[default]
        Ethernet,
        Wifi,
        Cellular,
    }

    #[derive(
        Debug, Default, sqlx::Type, Deserialize, PartialEq, Serialize, Clone, Copy, FromPrimitive,
    )]
    #[repr(i64)]
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
    #[derive(
        Debug, Default, sqlx::Type, Deserialize, PartialEq, Serialize, Clone, Copy, FromPrimitive,
    )]
    #[repr(i64)]
    pub enum SampleRate {
        #[default]
        HD,
        SD,
        UHD,
    }

    #[derive(
        Debug, Default, sqlx::Type, Deserialize, PartialEq, Serialize, Clone, Copy, FromPrimitive,
    )]
    #[repr(i64)]
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

    #[derive(
        Debug, Default, sqlx::Type, Deserialize, PartialEq, Serialize, Clone, Copy, FromPrimitive,
    )]
    #[repr(i64)]
    pub enum TransmitterConnector {
        #[default]
        TRRS,
        SHURE_TA4,
        MICRODOT,
        TRI_PIN,
    }

    #[derive(
        Debug, Default, sqlx::Type, Deserialize, PartialEq, Serialize, Clone, Copy, FromPrimitive,
    )]
    #[repr(i64)]
    pub enum MicrophoneType {
        #[default]
        DYNAMIC,
        PRE_POLORAIZED_CONDENSOR,
        CONDENSOR,
        RIBBON,
    }

    #[derive(
        Debug, Default, sqlx::Type, Deserialize, PartialEq, Serialize, Clone, Copy, FromPrimitive,
    )]
    #[repr(i64)]
    pub enum NetworkSpeeds {
        #[default]
        GIGABIT,
        TEN_GIGABIT,
        SUPERSPEED,
    }

    #[derive(
        Debug, Default, sqlx::Type, Deserialize, PartialEq, Serialize, Clone, Copy, FromPrimitive,
    )]
    #[repr(i64)]
    pub enum PowerConnector {
        #[default]
        IEC,
        EDISON,
        EDISON_20A,
        POWERCON_20A,
        POWERCON_32A,
        POWERCON_TRUE1,
        POWERCON_TRUE1_TOP,
        L6_20,
        L6_30,
        L6_50,
        L6_60,
        POE,
        DC_12V,
    }
}

pub mod structs {
    use serde::{Deserialize, Serialize};
    use sqlx::sqlite;
    use sqlx::Decode;
    use sqlx::SqliteConnection;

    use crate::sql;
    use crate::sql::entities::creation_structs::CreateAmplifierItem;

    use super::enums::*;
    use super::field_structs::*;

    #[derive(Debug, Default, sqlx::FromRow, Decode, Serialize, Deserialize, PartialEq, Clone)]
    pub struct Item {
        pub id: i64,
        pub created_at: String,
        pub updated_at: String,
        pub public_notes: Option<String>,
        pub cost: f64,
        pub weight: f64,
        pub dimensions: Option<Dimension>,
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
    use super::creation_structs::CreateItem;
    impl Item {
        pub async fn new_from_table(sql_result: &CreateItem, conn: &mut SqliteConnection) -> Item {
            Item {
                id: sql_result.id,
                created_at: sql_result.created_at,
                updated_at: sql_result.updated_at,
                public_notes: sql_result.public_notes,
                cost: sql_result.cost,
                weight: sql_result.weight,
                dimensions: sql_result
                    .dimensions
                    .map(|d| serde_json::from_str::<Dimension>(&d).unwrap_or_default()),
                model: sql_result.model,
                category: num::FromPrimitive::from_i64(sql_result.category).unwrap_or_default(),
                amplifier: {
                    let amp_id = sqlx::query!(
                        "SELECT amplifier_item_id FROM item where id = ?",
                        sql_result.id
                    )
                    .fetch_one(conn)
                    .await
                    .unwrap()
                    .amplifier_item_id;
                    let amp = sqlx::query_as!(
                        CreateAmplifierItem,
                        "SELECT * FROM amplifier_item where id = ?",
                        amp_id.unwrap()
                    )
                    .fetch_one(conn)
                    .await
                    .expect("Err w/ amplifier query");
                    amp
                },
                console: todo!(),
                computer: todo!(),
                processor: todo!(),
                network_item: todo!(),
                microphone: todo!(),
                radio_item: todo!(),
                speaker_item: todo!(),
                monitoring_item: todo!(),
                notes: sql_result
                    .notes
                    .map(|n| serde_json::from_str::<Vec<String>>(&n).unwrap_or_default()),
            }
        }
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct ConsoleItem {
        pub id: i64,
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
        pub id: i64,
        pub total_inputs: i64,
        pub total_outputs: i64,
        pub midi: MidiType,
        pub physical_connectivity: Option<Vec<PhysicalPort>>,
        pub network_connectivity: Vec<NetworkPort>,
        pub signal_protocol: Protocol,
        pub max_sample_rate: SampleRate,
        pub power: Power,
    }

    impl AmplifierItem {
        pub fn convert_query(amplifier_query: &CreateAmplifierItem) -> AmplifierItem {
            AmplifierItem {
                id: amplifier_query.id,
                total_inputs: amplifier_query.total_inputs,
                total_outputs: amplifier_query.total_outputs,
                midi: num::FromPrimitive::from_i64(amplifier_query.midi).unwrap_or_default(),
                physical_connectivity: amplifier_query
                    .physical_connectivity
                    .map(|phys| {
                        let thing =
                            serde_json::from_str::<Option<Vec<PhysicalPort>>>(&phys).unwrap();
                        thing
                    })
                    .unwrap_or_default(),
                network_connectivity: serde_json::from_str::<Vec<NetworkPort>>(
                    &amplifier_query.physical_connectivity.unwrap_or_default(),
                )
                .expect("JSON parse err AmplifierItem"),
                signal_protocol: num::FromPrimitive::from_i64(amplifier_query.signal_protocol)
                    .unwrap_or_default(),
                max_sample_rate: SampleRate::HD,
                power: amplifier_query
                    .power
                    .map(|p| serde_json::from_str::<Power>(&p).unwrap())
                    .unwrap(),
            }
        }
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct ComputerItem {
        pub id: i64,
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
        pub id: i64,
        pub total_inputs: i32,
        pub total_outputs: i32,
        pub physical_inputs: i32,
        pub physical_outputs: i32,
        pub midi: MidiType,
        pub protocol_inputs: i32,
        pub signal_protocol: Protocol,
        pub max_sample_rate: SampleRate,
        pub network_connectivity: Vec<NetworkPort>,
        pub physical_connectivity: Option<Vec<PhysicalPort>>,
        pub power: Power,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct NetworkItem {
        pub id: i64,
        pub network_type: NetworkType,
        pub poe_ports: i32,
        pub max_speed: i32,
        pub fiber: bool,
        pub network_connectivity: Vec<NetworkPort>,
        pub power: Power,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct MicrophoneItem {
        pub id: i64,
        pub max_spl: f64,
        pub phantom: bool,
        pub low_cut: bool,
        pub pad: bool,
        pub diaphragm_size: Option<f64>,
        pub output_impedance: f64,
        pub frequency_response: String,
        pub connector: Analog,
        pub microphone_type: Vec<MicrophoneType>,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct SpeakerItem {
        pub id: i64,
        pub driver: DriverArrangment,
        pub built_in_processing: bool,
        pub wireless: bool,
        pub max_spl: f64,
        pub power: Power,
        pub lower_frequency_response: i32,
        pub upper_frequency_response: i32,
        pub mounting_options: Vec<String>,
        pub physical_connectivity: Option<Vec<PhysicalPort>>,
        pub network_connectivity: Vec<NetworkPort>,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct MonitoringItem {
        pub id: i64,
        pub distro: bool,
        pub network_connectivity: Vec<NetworkPort>,
        pub physical_connectivity: Option<Vec<PhysicalPort>>,
        pub power: Power,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct RFItem {
        pub id: i64,
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
        pub width: f64,
        pub length: f64,
        pub height: f64,
        pub rack_unit: Option<f64>,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct PhysicalPort {
        port_identifier: Option<String>,
        connector_type: Analog,
        signal_lines: i32,
        input: bool,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct NetworkPort {
        port_identifier: Option<String>,
        max_connection_speed: NetworkSpeeds,
        power_over_ethernet: bool,
        protocol: Protocol,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct Power {
        pub wattage: f64,
        pub redundant: bool,
        pub lower_voltage: f64,
        pub max_wattage: f64,
        pub input_connector: PowerConnector,
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
        speaker_size: f64,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct ComputerPort {
        port_type: ComputerPortType,
        number_of_ports: i32,
        front_port: bool,
        version: Option<String>,
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
        pub id: i64,
        pub created_at: String,
        pub updated_at: String,
        pub public_notes: Option<String>,
        pub cost: f64,
        pub weight: f64,
        pub dimensions: Option<String>,
        pub model: String,
        pub category: i64,
        pub amplifier_item_id: Option<i64>,
        pub console_item_id: Option<i64>,
        pub computer_item_id: Option<i64>,
        pub processor_item_id: Option<i64>,
        pub network_item_id: Option<i64>,
        pub microphone_item_id: Option<i64>,
        pub radio_item_id: Option<i64>,
        pub speaker_item_id: Option<i64>,
        pub monitoring_item_id: Option<i64>,
        pub notes: Option<String>,
        pub searchable_model: Option<String>,
    }

    impl CreateItem {
        pub fn new(item: &Item) -> Self {
            CreateItem {
                id: item.id,
                created_at: item.created_at.to_owned(),
                updated_at: item.updated_at.to_owned(),
                public_notes: item.public_notes.to_owned(),
                cost: item.cost,
                weight: item.weight,
                model: item.model.to_owned(),
                dimensions: Some(serde_json::to_string(&item.dimensions).unwrap_or_default()),
                category: item.category as i64,
                amplifier_item_id: item.amplifier.as_ref().map(|item| item.id),
                console_item_id: item.console.as_ref().map(|item| item.id),
                computer_item_id: item.computer.as_ref().map(|item| item.id),
                processor_item_id: item.processor.as_ref().map(|item| item.id),
                network_item_id: item.network_item.as_ref().map(|item| item.id),
                microphone_item_id: item.microphone.as_ref().map(|item| item.id),
                radio_item_id: item.radio_item.as_ref().map(|item| item.id),
                speaker_item_id: item.speaker_item.as_ref().map(|item| item.id),
                monitoring_item_id: item.monitoring_item.as_ref().map(|item| item.id),
                notes: serde_json::to_string(&item.notes)
                    .map(|thing| Some(thing))
                    .unwrap_or_default(),
                searchable_model: Some(item.model.to_owned()),
            }
        }
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct CreateAmplifierItem {
        pub id: i64,
        pub total_inputs: i64,
        pub total_outputs: i64,
        pub midi: i64,
        pub physical_connectivity: Option<String>,
        pub network_connectivity: Option<String>,
        pub signal_protocol: i64,
        pub max_sample_rate: String,
        pub power: Option<String>,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    struct CreateConsoleItem {
        pub id: i64,
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
