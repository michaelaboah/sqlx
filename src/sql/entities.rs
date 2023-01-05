pub mod enums {

    #[derive(Debug)]
    pub enum Categories {
        Console,
        Processor,
        Monitoring,
        Speaker,
        Amplifier,
        Computer,
        Network,
        Radio,
        Microphones,
        SpkHardware,
        Generic,
    }

    #[derive(Debug)]
    pub enum MidiType {
        USB,
        Serial,
    }

    #[derive(Debug)]
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

    #[derive(Debug)]
    pub enum NetworkType {
        Ethernet,
        Wifi,
        Cellular,
    }
    #[derive(Debug)]
    pub enum Protocol {
        Ethernet,
        Usb,
        Midi,
        Rs232,
        Rj45,
        Tcpip,
        Thunderbolt,
    }
    #[derive(Debug)]
    pub enum SampleRate {
        SD,
        HD,
        UHD,
    }

    #[derive(Debug)]
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

    #[derive(Debug)]
    pub enum TransmitterConnector {
        SHURE_TA4,
        MICRODOT,
        TRRS,
        TRI_PIN,
    }

    #[derive(Debug)]
    pub enum MicrophoneType {
        PRE_POLORAIZED_CONDENSOR,
        CONDENSOR,
        RIBBON,
        DYNAMIC,
    }
}

pub mod stucts {
    use super::enums::*;
    use super::field_structs::*;

    #[derive(Debug)]
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
        console: Option<ConsoleItem>,
        computer: Option<ComputerItem>,
        processor: Option<ProcessingItem>,
        network_item: Option<NetworkItem>,
        microphone: Option<MicrophoneItem>,
        radio_item: Option<RFItem>,
        speaker_item: Option<SpeakerItem>,
        monitoring_item: Option<MonitoringItem>,
        notes: Vec<String>,
    }

    #[derive(Debug)]
    struct ConsoleItem {
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

    #[derive(Debug)]
    struct ComputerItem {
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
    #[derive(Debug)]
    struct ComputerPort {
        port_type: ComputerPortType,
        number_of_ports: i32,
        front_port: bool,
        version: String,
    }
    #[derive(Debug)]
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
    #[derive(Debug)]
    struct NetworkItem {
        id: i32,
        network_type: NetworkType,
        poe_ports: i32,
        max_speed: i32,
        fiber: bool,
        network_connectivity: Vec<NetworkPort>,
        power: Power,
    }
    #[derive(Debug)]
    struct MicrophoneItem {
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
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub struct MonitoringItem {
        id: i32,
        distro: bool,
        network_connectivity: Vec<NetworkPort>,
        physical_connectivity: Vec<PhysicalPort>,
        power: Power,
    }

    #[derive(Debug)]
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

    #[derive(Debug)]
    pub struct Dimension {
        width: f32,
        length: f32,
        height: f32,
        rack_unit: Option<f32>,
    }
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub struct PhysicalPort {
        port_identifier: String,
        connector_type: Analog,
        signal_lines: i32,
        input: bool,
    }
    #[derive(Debug)]
    pub struct NetworkPort {
        port_identifier: String,
        port_type: NetworkType,
        input: bool,
    }
    #[derive(Debug)]
    pub struct Power {
        voltage: Option<f32>,
        current: Option<f32>,
        frequency: Option<f32>,
    }
    #[derive(Debug)]
    pub struct Transmitter {
        connector: TransmitterConnector,
    }
    #[derive(Debug)]
    pub struct Reciever {
        network_ports: Vec<NetworkPort>,
        physical_ports: Vec<PhysicalPort>,
        power: Power,
    }

    #[derive(Debug)]
    pub struct DriverArrangment {
        speaker_size: f32,
    }
}
