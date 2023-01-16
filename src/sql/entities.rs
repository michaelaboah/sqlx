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
    use super::enums::*;
    use super::field_structs::*;
    use serde::{Deserialize, Serialize};
    use sqlx::Decode;

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
        pub processor: Option<ProcessorItem>,
        pub network_item: Option<NetworkItem>,
        pub microphone: Option<MicrophoneItem>,
        pub radio_item: Option<RFItem>,
        pub speaker_item: Option<SpeakerItem>,
        pub monitoring_item: Option<MonitoringItem>,
        pub notes: Option<Vec<String>>,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct ConsoleItem {
        pub id: i64,
        pub total_inputs: i64,
        pub total_outputs: i64,
        pub total_busses: i64,
        pub physical_inputs: i64,
        pub physical_outputs: i64,
        pub aux_inputs: i64,
        pub physical_aux_inputs: i64,
        pub phantom_power_inputs: i64,
        pub faders: i64,
        pub motorized: bool,
        pub midi: MidiType,
        pub protocol_inputs: i64,
        pub signal_protocol: Protocol,
        pub can_expand: bool,
        pub max_sample_rate: SampleRate,
        pub power: Power,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct AmplifierItem {
        pub amplifier_id: i64,
        pub total_inputs: i64,
        pub total_outputs: i64,
        pub midi: MidiType,
        pub physical_connectivity: Option<Vec<PhysicalPort>>,
        pub network_connectivity: Vec<NetworkPort>,
        pub signal_protocol: Protocol,
        pub max_sample_rate: SampleRate,
        pub power: Power,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct ComputerItem {
        pub computer_id: i64,
        pub cpu_processor: String,
        pub ram_size: i64,
        pub total_storage: i64,
        pub model_year: String,
        pub operating_system: String,
        pub dedicated_graphics: bool,
        pub network_connectivity: Vec<NetworkPort>,
        pub computer_ports: Vec<ComputerPort>,
        pub power: Power,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct ProcessorItem {
        pub processor_id: i64,
        pub total_inputs: i64,
        pub total_outputs: i64,
        pub physical_inputs: i64,
        pub physical_outputs: i64,
        pub midi: MidiType,
        pub protocol_inputs: i64,
        pub signal_protocol: Protocol,
        pub max_sample_rate: SampleRate,
        pub network_connectivity: Vec<NetworkPort>,
        pub physical_connectivity: Option<Vec<PhysicalPort>>,
        pub power: Option<Power>,
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct NetworkItem {
        pub network_id: i64,
        pub network_type: NetworkType,
        pub poe_ports: i64,
        pub max_speed: i64,
        pub fiber: bool,
        pub network_connectivity: Vec<NetworkPort>,
        pub power: Power,
    }
    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct MicrophoneItem {
        pub microphone_id: i64,
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
        pub lower_frequency_response: i64,
        pub upper_frequency_response: i64,
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
        pub physical_range: i64,
        pub lower_frequency_response: i64,
        pub upper_frequency_response: i64,
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
        signal_lines: i64,
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
        number_of_ports: i64,
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
    use sqlx::pool;
    use sqlx::sqlite::Sqlite;
    use sqlx::Pool;

    // trait SqlConvert {
    //     fn convert_from_row(&self, pool: &Pool)
    // }
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
                amplifier_item_id: item.amplifier.as_ref().map(|item| item.amplifier_id),
                console_item_id: item.console.as_ref().map(|item| item.id),
                computer_item_id: item.computer.as_ref().map(|item| item.computer_id),
                processor_item_id: item.processor.as_ref().map(|item| item.processor_id),
                network_item_id: item.network_item.as_ref().map(|item| item.network_id),
                microphone_item_id: item.microphone.as_ref().map(|item| item.microphone_id),
                radio_item_id: item.radio_item.as_ref().map(|item| item.id),
                speaker_item_id: item.speaker_item.as_ref().map(|item| item.id),
                monitoring_item_id: item.monitoring_item.as_ref().map(|item| item.id),
                notes: serde_json::to_string(&item.notes)
                    .map(|thing| Some(thing))
                    .unwrap_or_default(),
                searchable_model: Some(item.model.to_owned()),
            }
        }
        pub async fn convert_from_row(&self, pool: &Pool<Sqlite>) -> Item {
            let dim = self
                .dimensions
                .as_ref()
                .map_or(None, |d| serde_json::from_str(&d).unwrap_or_default());
            Item {
                id: self.id,
                created_at: self.created_at.to_owned(),
                updated_at: self.updated_at.to_owned(),
                public_notes: self.public_notes.to_owned(),
                cost: self.cost,
                weight: self.weight,
                dimensions: dim,
                model: self.model.to_owned(),
                category: num::FromPrimitive::from_i64(self.category).unwrap_or_default(),
                amplifier: {
                    let sub_id =
                        sqlx::query!("SELECT amplifier_item_id FROM item where id = ?", self.id)
                            .fetch_one(pool)
                            .await
                            .unwrap()
                            .amplifier_item_id;

                    let sub_result = match sub_id {
                        Some(id) => {
                            let result = sqlx::query_as!(
                                CreateAmplifierItem,
                                "SELECT * FROM amplifier_item where amplifier_id = ?",
                                id
                            )
                            .fetch_one(pool)
                            .await
                            .expect("Err w/ amplifier query");
                            Some(result)
                        }
                        None => None,
                    };
                    match sub_result {
                        Some(res) => Some(res.convert_query()),
                        None => None,
                    }
                },
                console: {
                    let sub_id =
                        sqlx::query!("SELECT console_item_id FROM item where id = ?", self.id)
                            .fetch_one(pool)
                            .await
                            .unwrap()
                            .console_item_id;

                    let sub_result = match sub_id {
                        Some(id) => {
                            let result = sqlx::query_as!(
                                CreateConsoleItem,
                                "SELECT * FROM console_item where console_id = ?",
                                id
                            )
                            .fetch_one(pool)
                            .await
                            .expect("Err w/ amplifier query");
                            Some(result)
                        }
                        None => None,
                    };
                    match sub_result {
                        Some(res) => Some(res.convert_query()),
                        None => None,
                    }
                },
                computer: {
                    let sub_id =
                        sqlx::query!("SELECT computer_item_id FROM item where id = ?", self.id)
                            .fetch_one(pool)
                            .await
                            .unwrap()
                            .computer_item_id;

                    let sub_result = match sub_id {
                        Some(id) => {
                            let result = sqlx::query_as!(
                                CreateComputerItem,
                                "SELECT * FROM computer_item where computer_id = ?",
                                id
                            )
                            .fetch_one(pool)
                            .await
                            .expect("Err w/ amplifier query");
                            Some(result)
                        }
                        None => None,
                    };
                    match sub_result {
                        Some(res) => Some(res.convert_query()),
                        None => None,
                    }
                },
                processor: {
                    let sub_id =
                        sqlx::query!("SELECT network_item_id FROM item where id = ?", self.id)
                            .fetch_one(pool)
                            .await
                            .unwrap()
                            .network_item_id;

                    let sub_result = match sub_id {
                        Some(id) => {
                            let result = sqlx::query_as!(
                                CreateProcessorItem,
                                "SELECT * FROM processor_item where processor_id = ?",
                                id
                            )
                            .fetch_one(pool)
                            .await
                            .expect("Err w/ amplifier query");
                            Some(result)
                        }
                        None => None,
                    };
                    match sub_result {
                        Some(res) => Some(res.convert_query()),
                        None => None,
                    }
                    // None
                },
                network_item: {
                    let sub_id =
                        sqlx::query!("SELECT network_item_id FROM item where id = ?", self.id)
                            .fetch_one(pool)
                            .await
                            .unwrap()
                            .network_item_id;

                    let sub_result = match sub_id {
                        Some(id) => {
                            let result = sqlx::query_as!(
                                CreateNetworkItem,
                                "SELECT * FROM network_item where network_id = ?",
                                id
                            )
                            .fetch_one(pool)
                            .await
                            .expect("Err w/ amplifier query");
                            Some(result)
                        }
                        None => None,
                    };
                    match sub_result {
                        Some(res) => Some(res.convert_query()),
                        None => None,
                    }
                },
                microphone: {
                    let sub_id =
                        sqlx::query!("SELECT microphone_item_id FROM item where id = ?", self.id)
                            .fetch_one(pool)
                            .await
                            .unwrap()
                            .microphone_item_id;

                    let sub_result = match sub_id {
                        Some(id) => {
                            let result = sqlx::query_as!(
                                CreateMicrophoneItem,
                                "SELECT * FROM microphone_item where microphone_id = ?",
                                id
                            )
                            .fetch_one(pool)
                            .await
                            .expect("Err w/ microphone query");
                            Some(result)
                        }
                        None => None,
                    };
                    match sub_result {
                        Some(res) => Some(res.convert_query()),
                        None => None,
                    }
                },
                radio_item: None,
                speaker_item: None,
                monitoring_item: None,
                notes: self
                    .notes
                    .as_ref()
                    .map(|n| serde_json::from_str::<Vec<String>>(&n).unwrap_or_default()),
            }
        }
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct CreateAmplifierItem {
        pub amplifier_id: i64,
        pub total_inputs: i64,
        pub total_outputs: i64,
        pub midi: i64,
        pub physical_connectivity: Option<String>,
        pub network_connectivity: Option<String>,
        pub signal_protocol: i64,
        pub max_sample_rate: i64,
        pub power: Option<String>,
    }

    impl CreateAmplifierItem {
        pub fn convert_query(&self) -> AmplifierItem {
            AmplifierItem {
                amplifier_id: self.amplifier_id,
                total_inputs: self.total_inputs,
                total_outputs: self.total_outputs,
                midi: num::FromPrimitive::from_i64(self.midi).unwrap_or_default(),
                physical_connectivity: self
                    .physical_connectivity
                    .as_ref()
                    .map(|phys| {
                        let thing =
                            serde_json::from_str::<Option<Vec<PhysicalPort>>>(&phys).unwrap();
                        thing
                    })
                    .unwrap_or_default(),
                network_connectivity: {
                    self.network_connectivity
                        .as_ref()
                        .map(|net| serde_json::from_str::<Vec<NetworkPort>>(&net).unwrap())
                        .unwrap_or_default()
                },
                signal_protocol: num::FromPrimitive::from_i64(self.signal_protocol)
                    .unwrap_or_default(),
                max_sample_rate: SampleRate::HD,
                power: self
                    .power
                    .as_ref()
                    .map(|p| serde_json::from_str::<Power>(&p).unwrap())
                    .unwrap(),
            }
        }
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    struct CreateConsoleItem {
        pub console_id: i64,
        pub total_inputs: i64,
        pub total_outputs: i64,
        pub total_busses: i64,
        pub physical_inputs: i64,
        pub physical_outputs: i64,
        pub aux_inputs: i64,
        pub physical_aux_inputs: i64,
        pub phantom_power_inputs: i64,
        pub faders: i64,
        pub motorized: i64,
        pub midi: i64,
        pub protocol_inputs: Option<i64>,
        pub signal_protocol: i64,
        pub can_expand: Option<i64>,
        pub max_sample_rate: i64,
        pub power: Option<String>,
    }

    impl CreateConsoleItem {
        pub fn new(console: ConsoleItem) -> Self {
            CreateConsoleItem {
                console_id: console.id,
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
        pub fn convert_query(&self) -> ConsoleItem {
            ConsoleItem {
                id: self.console_id,
                total_inputs: self.total_inputs,
                total_outputs: self.total_outputs,
                midi: num::FromPrimitive::from_i64(self.midi).unwrap_or_default(),
                total_busses: self.physical_inputs,
                physical_inputs: self.physical_inputs,
                physical_outputs: self.physical_outputs,
                aux_inputs: self.aux_inputs,
                physical_aux_inputs: self.physical_aux_inputs,
                phantom_power_inputs: self.phantom_power_inputs,
                faders: self.faders,
                motorized: self.motorized != 0,
                protocol_inputs: self.protocol_inputs.unwrap(),
                can_expand: match self.can_expand {
                    Some(i) => i != 0,
                    None => false,
                },
                signal_protocol: num::FromPrimitive::from_i64(self.signal_protocol)
                    .unwrap_or_default(),
                max_sample_rate: num::FromPrimitive::from_i64(self.max_sample_rate)
                    .unwrap_or_default(),
                power: self
                    .power
                    .as_ref()
                    .map(|p| serde_json::from_str::<Power>(&p).unwrap())
                    .unwrap(),
            }
        }
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct CreateComputerItem {
        pub computer_id: i64,
        pub cpu_processor: String,
        pub ram_size: i64,
        pub total_storage: i64,
        pub model_year: Option<String>,
        pub operating_system: Option<String>,
        pub dedicated_graphics: i64,
        pub network_connectivity: Option<String>,
        pub computer_ports: Option<String>,
        pub power: Option<String>,
    }

    impl CreateComputerItem {
        pub fn convert_query(&self) -> ComputerItem {
            ComputerItem {
                computer_id: self.computer_id,
                cpu_processor: self.cpu_processor.to_owned(),
                ram_size: self.ram_size,
                total_storage: self.total_storage,
                model_year: self.model_year.to_owned().unwrap(),
                operating_system: self.operating_system.to_owned().unwrap(),
                dedicated_graphics: self.dedicated_graphics != 0,
                computer_ports: self
                    .computer_ports
                    .as_ref()
                    .map_or(None, |cp| {
                        Some(serde_json::from_str::<Vec<ComputerPort>>(&cp).unwrap())
                    })
                    .unwrap_or_default(),
                network_connectivity: {
                    self.network_connectivity
                        .as_ref()
                        .map(|net| serde_json::from_str::<Vec<NetworkPort>>(&net).unwrap())
                        .unwrap_or_default()
                },
                power: self
                    .power
                    .as_ref()
                    .map(|p| serde_json::from_str::<Power>(&p).unwrap())
                    .unwrap(),
            }
        }
    }

    #[derive(Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
    pub struct CreateMicrophoneItem {
        pub microphone_id: i64,
        pub max_spl: i64,
        pub phantom: Option<i64>,
        pub low_cut: Option<i64>,
        pub pad: Option<i64>,
        pub diaphragm_size: Option<i64>,
        pub output_impedance: Option<i64>,
        pub frequency_response: Option<String>,
        pub connector: i64,
        pub microphone_type: String,
    }

    impl CreateMicrophoneItem {
        pub fn convert_query(&self) -> MicrophoneItem {
            MicrophoneItem {
                microphone_id: self.microphone_id,
                max_spl: self.max_spl as f64,
                phantom: self.phantom.unwrap_or_default() != 0,
                low_cut: self.low_cut.unwrap_or_default() != 0,
                pad: self.pad.unwrap_or_default() != 0,
                diaphragm_size: Some(self.diaphragm_size.unwrap_or_default() as f64),
                output_impedance: self.output_impedance.unwrap_or_default() as f64,
                frequency_response: self.frequency_response.to_owned().unwrap_or_default(),
                connector: num::FromPrimitive::from_i64(self.connector).unwrap_or_default(),
                microphone_type: {
                    serde_json::from_str(&self.microphone_type).unwrap_or_default()
                },
            }
        }
    }

    pub struct CreateNetworkItem {
        pub network_id: i64,
        pub network_type: i64,
        pub poe_ports: i64,
        pub max_speed: i64,
        pub fiber: Option<i64>,
        pub network_connectivity: Option<String>,
        pub power: Option<String>,
    }

    impl CreateNetworkItem {
        pub fn convert_query(&self) -> NetworkItem {
            NetworkItem {
                network_id: self.network_id,
                network_type: num::FromPrimitive::from_i64(self.network_type).unwrap_or_default(),
                poe_ports: self.poe_ports,
                max_speed: self.max_speed,
                fiber: self.fiber.unwrap_or_default() != 0,
                network_connectivity: self
                    .network_connectivity
                    .as_ref()
                    .map(|net| serde_json::from_str::<Vec<NetworkPort>>(net).unwrap_or_default())
                    .unwrap_or_default(),
                power: self
                    .power
                    .as_ref()
                    .map(|p| serde_json::from_str::<Power>(&p).unwrap_or_default())
                    .unwrap(),
            }
        }
    }

    pub struct CreateProcessorItem {
        pub processor_id: i64,
        pub total_inputs: i64,
        pub total_outputs: i64,
        pub physical_inputs: i64,
        pub physical_outputs: i64,
        pub midi: Option<i64>,
        pub protocol_inputs: Option<i64>,
        pub signal_protocol: i64,
        pub max_sample_rate: i64,
        pub network_connectivity: Option<String>,
        pub physical_connectivity: Option<String>,
        pub power: Option<String>,
    }

    impl CreateProcessorItem {
        fn convert_query(&self) -> ProcessorItem {
            ProcessorItem {
                processor_id: self.processor_id,
                total_inputs: self.total_inputs,
                total_outputs: self.total_outputs,
                physical_inputs: self.physical_inputs,
                physical_outputs: self.physical_outputs,
                midi: num::FromPrimitive::from_i64(self.midi.unwrap_or_default())
                    .unwrap_or_default(),
                protocol_inputs: self.protocol_inputs.unwrap_or_default(),
                signal_protocol: num::FromPrimitive::from_i64(self.signal_protocol)
                    .unwrap_or_default(),
                max_sample_rate: num::FromPrimitive::from_i64(self.max_sample_rate)
                    .unwrap_or_default(),
                network_connectivity: self
                    .network_connectivity
                    .as_ref()
                    .map(|net| serde_json::from_str(net).unwrap_or_default())
                    .unwrap(),
                physical_connectivity: self
                    .physical_connectivity
                    .as_ref()
                    .map(|phys| serde_json::from_str(phys).unwrap_or_default()),
                power: self
                    .power
                    .as_ref()
                    .map(|pow| serde_json::from_str(pow).unwrap_or_default()),
            }
        }
    }
}
