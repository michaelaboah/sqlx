pub mod schema {
    pub const TABLE_QUERIES: [&str; 11] = [
        ITEM_TABLE,
        AMPLIFIER_ITEM_TABLE,
        COMPUTER_ITEM_TABLE,
        CONSOLE_ITEM_TABLE,
        MICROPHONE_ITEM_TABLE,
        MONITORING_ITEM,
        NETWORK_ITEM,
        PROCESSING_ITEM_TABLE,
        RADIO_ITEM,
        SPEAKER_ITEM,
        RFBAND,
    ];

    pub const ITEM_RELATIONSHIPS: [&str; 12] = [
        "CREATE INDEX IF NOT EXISTS  rfband_rf_item_id_index ON rfband (rf_item_id);",
        "CREATE UNIQUE INDEX IF NOT EXISTS rfband_band_name_unique ON rfband (band_name);",
        "CREATE UNIQUE INDEX IF NOT EXISTS item_model_unique ON item (model);",
        "CREATE UNIQUE INDEX IF NOT EXISTS item_amplifier_item_id_unique ON item (amplifier_item_id);",
        "CREATE UNIQUE INDEX IF NOT EXISTS item_console_item_id_unique ON item (console_item_id);",
        "CREATE UNIQUE INDEX IF NOT EXISTS item_computer_item_id_unique ON item (computer_item_id);",
        "CREATE UNIQUE INDEX IF NOT EXISTS item_processor_item_id_unique ON item (processor_item_id);",
        "CREATE UNIQUE INDEX IF NOT EXISTS item_network_item_id_unique ON item (network_item_id);",
        "CREATE UNIQUE INDEX IF NOT EXISTS item_microphone_item_id_unique ON item (microphone_item_id);",
        "CREATE UNIQUE INDEX IF NOT EXISTS item_radio_item_id_unique ON item (radio_item_id);",
        "CREATE UNIQUE INDEX IF NOT EXISTS item_speaker_item_id_unique ON item (speaker_item_id);",
        "CREATE UNIQUE INDEX IF NOT EXISTS item_monitoring_item_id_unique ON item (monitoring_item_id);",
    ];

    pub const PRAGMA_QUERIES: [&str; 2] = ["PRAGMA foreign_keys = ON;", "PRAGMA integrity_check;"];

    const ITEM_TABLE: &str = "CREATE TABLE IF NOT EXISTS item (
        id integer NOT NULL PRIMARY KEY autoincrement,
        created_at TEXT NOT NULL DEFAULT 'NOW()',
        updated_at TEXT NOT NULL,
        public_notes text NULL,
        cost DOUBLE NOT NULL,
        weight DOUBLE NOT NULL,
        dimensions TEXT NULL,
        model text UNIQUE NOT NULL,
        category integer NOT NULL,
        amplifier_item_id integer NULL,
        console_item_id integer NULL,
        computer_item_id integer NULL,
        processor_item_id integer NULL,
        network_item_id integer NULL,
        microphone_item_id integer NULL,
        radio_item_id integer NULL,
        speaker_item_id integer NULL,
        monitoring_item_id integer NULL,
        searchable_model text NULL,
        notes TEXT NULL,
        CONSTRAINT item_amplifier_id_foreign FOREIGN key(amplifier_item_id) REFERENCES amplifier_item(id) ON DELETE
        SET NULL ON UPDATE CASCADE,
            CONSTRAINT item_console_id_foreign FOREIGN key(console_item_id) REFERENCES console_item(id) ON DELETE
        SET NULL ON UPDATE CASCADE,
            CONSTRAINT item_computer_id_foreign FOREIGN key(computer_item_id) REFERENCES computer_item(id) ON DELETE
        SET NULL ON UPDATE CASCADE,
            CONSTRAINT item_processor_id_foreign FOREIGN key(processor_item_id) REFERENCES processor_item(id) ON DELETE
        SET NULL ON UPDATE CASCADE,
            CONSTRAINT item_network_item_id_foreign FOREIGN key(network_item_id) REFERENCES network_item(id) ON DELETE
        SET NULL ON UPDATE CASCADE,
            CONSTRAINT item_microphone_id_foreign FOREIGN key(microphone_item_id) REFERENCES microphone_item(id) ON DELETE
        SET NULL ON UPDATE CASCADE,
            CONSTRAINT item_radio_item_id_foreign FOREIGN key(radio_item_id) REFERENCES rfitem(id) ON DELETE
        SET NULL ON UPDATE CASCADE,
            CONSTRAINT item_speaker_item_id_foreign FOREIGN key(speaker_item_id) REFERENCES speaker_item(id) ON DELETE
        SET NULL ON UPDATE CASCADE,
            CONSTRAINT item_monitoring_item_id_foreign FOREIGN key(monitoring_item_id) REFERENCES monitoring_item(id) ON DELETE
        SET NULL ON UPDATE CASCADE
    );";

    const AMPLIFIER_ITEM_TABLE: &str = "
            CREATE TABLE IF NOT EXISTS amplifier_item (
                id integer NOT NULL PRIMARY KEY autoincrement,
                total_inputs integer NOT NULL,
                total_outputs integer NOT NULL,
                midi integer NOT NULL,
                physical_connectivity TEXT NULL,
                network_connectivity TEXT NULL,
                signal_protocol integer NOT NULL,
                max_sample_rate text CHECK (
                    max_sample_rate in (
                        'SD',
                        'HD',
                        'UHD'
                    )
                ) NOT NULL,
                power TEXT NULL
            );";
    const COMPUTER_ITEM_TABLE: &str = "CREATE TABLE IF NOT EXISTS computer_item (
        id integer NOT NULL PRIMARY KEY autoincrement,
        cpu_processor text NOT NULL,
        ram_size integer NOT NULL,
        total_storage integer NOT NULL,
        model_year text NULL,
        operating_system text NULL,
        dedicated_graphics integer NOT NULL,
        network_connectivity TEXT NULL,
        computer_ports TEXT NULL,
        power TEXT NULL
      );";

    const CONSOLE_ITEM_TABLE: &str = "CREATE TABLE IF NOT EXISTS console_item (
            id integer NOT NULL PRIMARY KEY autoincrement,
            total_inputs integer NOT NULL,
            total_outputs integer NOT NULL,
            total_busses integer NOT NULL,
            physical_inputs integer NOT NULL,
            physical_outputs integer NOT NULL,
            aux_inputs integer NOT NULL,
            physical_aux_inputs integer NOT NULL,
            phantom_power_inputs integer NOT NULL,
            faders integer NOT NULL,
            motorized integer NOT NULL,
            midi integer NOT NULL,
            protocol_inputs integer NULL,
            signal_protocol integer NOT NULL,
            can_expand integer NULL DEFAULT NULL,
            max_sample_rate text CHECK (
                max_sample_rate in (
                    'SD',
                    'HD',
                    'UHD'
                )
            ) NOT NULL,
            power TEXT NULL
        );";

    const MICROPHONE_ITEM_TABLE: &str = "CREATE TABLE IF NOT EXISTS microphone_item (
            id integer NOT NULL PRIMARY KEY autoincrement,
            max_spl integer NOT NULL,
            phantom integer NULL,
            low_cut integer NULL,
            pad integer NULL,
            diaphragm_size integer NULL,
            output_impedance integer NULL,
            frequency_response text NULL,
            connector integer NOT NULL,
            microphone_type text NOT NULL
        );";

    const MONITORING_ITEM: &str = "CREATE TABLE IF NOT EXISTS monitoring_item (
            id integer NOT NULL PRIMARY KEY autoincrement,
            distro integer NULL,
            network_connectivity TEXT NULL,
            physical_connectivity TEXT NULL,
            power TEXT NULL
        );";

    const NETWORK_ITEM: &str = "CREATE TABLE IF NOT EXISTS network_item (
            id integer NOT NULL PRIMARY KEY autoincrement,
            network_type integer NOT NULL,
            poe_ports integer NOT NULL,
            max_speed integer NOT NULL,
            fiber integer NULL,
            network_connectivity TEXT NULL,
            power TEXT NULL
        );";

    const PROCESSING_ITEM_TABLE: &str = "CREATE TABLE IF NOT EXISTS processor_item (
        id integer NOT NULL PRIMARY KEY autoincrement,
        total_inputs integer NOT NULL,
        total_outputs integer NOT NULL,
        physical_inputs integer NOT NULL,
        physical_outputs integer NOT NULL,
        midi integer NULL,
        protocol_inputs integer NULL,
        signal_protocol integer NOT NULL,
        max_sample_rate text CHECK (
            max_sample_rate in (
                'SD',
                'HD',
                'UHD'
            )
        ) NOT NULL,
        network_connectivity TEXT NULL,
        physical_connectivity TEXT NULL,
        power TEXT NULL
        );";

    const RADIO_ITEM: &str = "CREATE TABLE IF NOT EXISTS rfitem (
            id integer NOT NULL PRIMARY KEY autoincrement,
            physical_range integer NOT NULL,
            lower_frequency_response integer NOT NULL,
            upper_frequency_response integer NOT NULL,
            transmitter TEXT NULL,
            reciever TEXT NULL
        );";
    const SPEAKER_ITEM: &str = "CREATE TABLE IF NOT EXISTS speaker_item (
            id integer NOT NULL PRIMARY KEY autoincrement,
            driver TEXT NOT NULL,
            built_in_processing integer NOT NULL,
            wireless integer NOT NULL,
            max_spl integer NOT NULL,
            power TEXT NOT NULL,
            lower_frequency_response integer NOT NULL,
            upper_frequency_response integer NOT NULL,
            mounting_options text NOT NULL,
            physical_connectivity TEXT NULL,
            network_connectivity TEXT NULL
        );";

    const RFBAND: &str = "CREATE TABLE IF NOT EXISTS rfband (
        id integer NOT NULL PRIMARY KEY autoincrement,
        rf_item_id integer NOT NULL,
        band_name text NOT NULL,
        lower_frequency_range integer NOT NULL,
        upper_frequency_range integer NOT NULL,
        manufacturer text NOT NULL,
        nation_code text CHECK (
            nation_code in (
                'Afghanistan',
                'Åland Islands',
                'Albania',
                'Algeria',
                'American Samoa',
                'AndorrA',
                'Angola',
                'Anguilla',
                'Antarctica',
                'Antigua and Barbuda',
                'Argentina',
                'Armenia',
                'Aruba',
                'Australia',
                'Austria',
                'Azerbaijan',
                'Bahamas',
                'Bahrain',
                'Bangladesh',
                'Barbados',
                'Belarus',
                'Belgium',
                'Belize',
                'Benin',
                'Bermuda',
                'Bhutan',
                'Bolivia',
                'Bosnia and Herzegovina',
                'Botswana',
                'Bouvet Island',
                'Brazil',
                'British Indian Ocean Territory',
                'Brunei Darussalam',
                'Bulgaria',
                'Burkina Faso',
                'Burundi',
                'Cambodia',
                'Cameroon',
                'Canada',
                'Cape Verde',
                'Cayman Islands',
                'Central African Republic',
                'Chad',
                'Chile',
                'China',
                'Christmas Island',
                'Cocos (Keeling) Islands',
                'Colombia',
                'Comoros',
                'Congo',
                'Congo, The Democratic Republic of the',
                'Cook Islands',
                'Costa Rica',
                'Cote D''Ivoire',
                'Croatia',
                'Cuba',
                'Cyprus',
                'Czech Republic',
                'Denmark',
                'Djibouti',
                'Dominica',
                'Dominican Republic',
                'Ecuador',
                'Egypt',
                'El Salvador',
                'Equatorial Guinea',
                'Eritrea',
                'Estonia',
                'Ethiopia',
                'Falkland Islands (Malvinas)',
                'Faroe Islands',
                'Fiji',
                'Finland',
                'France',
                'French Guiana',
                'French Polynesia',
                'French Southern Territories',
                'Gabon',
                'Gambia',
                'Georgia',
                'Germany',
                'Ghana',
                'Gibraltar',
                'Greece',
                'Greenland',
                'Grenada',
                'Guadeloupe',
                'Guam',
                'Guatemala',
                'Guernsey',
                'Guinea',
                'Guinea-Bissau',
                'Guyana',
                'Haiti',
                'Heard Island and Mcdonald Islands',
                'Holy See (Vatican City State)',
                'Honduras',
                'Hong Kong',
                'Hungary',
                'Iceland',
                'India',
                'Indonesia',
                'Iran, Islamic Republic Of',
                'Iraq',
                'Ireland',
                'Isle of Man',
                'Israel',
                'Italy',
                'Jamaica',
                'Japan',
                'Jersey',
                'Jordan',
                'Kazakhstan',
                'Kenya',
                'Kiribati',
                'Korea, Democratic People''s Republic of',
                'Korea, Republic of',
                'Kuwait',
                'Kyrgyzstan',
                'Lao People''s Democratic Republic',
                'Latvia',
                'Lebanon',
                'Lesotho',
                'Liberia',
                'Libyan Arab Jamahiriya',
                'Liechtenstein',
                'Lithuania',
                'Luxembourg',
                'Macao',
                'Macedonia, The Former Yugoslav Republic of',
                'Madagascar',
                'Malawi',
                'Malaysia',
                'Maldives',
                'Mali',
                'Malta',
                'Marshall Islands',
                'Martinique',
                'Mauritania',
                'Mauritius',
                'Mayotte',
                'Mexico',
                'Micronesia, Federated States of',
                'Moldova, Republic of',
                'Monaco',
                'Mongolia',
                'Montserrat',
                'Morocco',
                'Mozambique',
                'Myanmar',
                'Namibia',
                'Nauru',
                'Nepal',
                'Netherlands',
                'Netherlands Antilles',
                'New Caledonia',
                'New Zealand',
                'Nicaragua',
                'Niger',
                'Nigeria',
                'Niue',
                'Norfolk Island',
                'Northern Mariana Islands',
                'Norway',
                'Oman',
                'Pakistan',
                'Palau',
                'Palestinian Territory, Occupied',
                'Panama',
                'Papua New Guinea',
                'Paraguay',
                'Peru',
                'Philippines',
                'Pitcairn',
                'Poland',
                'Portugal',
                'Puerto Rico',
                'Qatar',
                'Reunion',
                'Romania',
                'Russian Federation',
                'RWANDA',
                'Saint Helena',
                'Saint Kitts and Nevis',
                'Saint Lucia',
                'Saint Pierre and Miquelon',
                'Saint Vincent and the Grenadines',
                'Samoa',
                'San Marino',
                'Sao Tome and Principe',
                'Saudi Arabia',
                'Senegal',
                'Serbia and Montenegro',
                'Seychelles',
                'Sierra Leone',
                'Singapore',
                'Slovakia',
                'Slovenia',
                'Solomon Islands',
                'Somalia',
                'South Africa',
                'South Georgia and the South Sandwich Islands',
                'Spain',
                'Sri Lanka',
                'Sudan',
                'Suriname',
                'Svalbard and Jan Mayen',
                'Swaziland',
                'Sweden',
                'Switzerland',
                'Syrian Arab Republic',
                'Taiwan, Province of China',
                'Tajikistan',
                'Tanzania, United Republic of',
                'Thailand',
                'Timor-Leste',
                'Togo',
                'Tokelau',
                'Tonga',
                'Trinidad and Tobago',
                'Tunisia',
                'Turkey',
                'Turkmenistan',
                'Turks and Caicos Islands',
                'Tuvalu',
                'Uganda',
                'Ukraine',
                'United Arab Emirates',
                'United Kingdom',
                'United States',
                'United States Minor Outlying Islands',
                'Uruguay',
                'Uzbekistan',
                'Vanuatu',
                'Venezuela',
                'Viet Nam',
                'Virgin Islands, British',
                'Virgin Islands, U.S.',
                'Wallis and Futuna',
                'Western Sahara',
                'Yemen',
                'Zambia',
                'Zimbabwe'
            )
        ) NOT NULL,
        deprecated integer NOT NULL,
        CONSTRAINT rfband_rf_item_id_foreign FOREIGN key(rf_item_id) REFERENCES rfitem(id) ON UPDATE CASCADE
    );";
}

pub mod insertion {
    use crate::sql::{
        database_setup::sql_setup::get_connection,
        entities::{creation_structs::CreateItem, enums::Categories, structs::Item},
    };
    use serde::de::value::Error;

    const _ITEM_INSERT: &str = "INSERT INTO item (id, created_at, updated_at, public_notes, cost, weight, dimensions, model, category, amplifier_item_id,
        console_item_id, computer_item_id, processor_item_id, network_item_id, microphone_item_id, radio_item_id, speaker_item_id, monitoring_item_id,  
         notes)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)";
    const _AMPLIFIER_INSERT: &str = "INSERT INTO amplifier_item (id, total_inputs, total_outputs, midi, physical_connectivity, network_connectivity, signal_protocol, max_sample_rate, power)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);";
    const _CONSOLE_INSERT: &str = "INSERT INTO console_item (
        id, total_inputs, total_outputs, total_busses, physical_inputs, physical_outputs, aux_inputs, physical_aux_inputs, phantom_power_inputs, faders, motorized, midi, protocol_inputs, signal_protocol, can_expand, max_sample_rate, power)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17);";
    const COMPUTER_INSERT: &str = "INSERT INTO computer_item (id, cpu_processor, ram_size, total_storage, model_year, operating_system, dedicated_graphics, network_connectivity, computer_ports, power)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10);";
    const NETWORK_INSERT: &str = "INSERT INTO network_item (id, network_type, poe_ports, max_speed, fiber, network_connectivity, power)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);";
    const PROCESSOR_ITEM: &str = "INSERT INTO processor_item (id, total_inputs, total_outputs, physical_inputs, physical_outputs, midi, protocol_inputs, signal_protocol, max_sample_rate, network_connectivity, physical_connectivity, power)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12);";
    const MONITORING_INSERT: &str = "INSERT INTO monitoring_item (id, distro, network_connectivity, physical_connectivity, power)
        VALUES (?1, ?2, ?3, ?4, ?5);";
    const MICROPHONE_INSERT: &str = "INSERT INTO microphone_item (id, max_spl, phantom, low_cut, pad, diaphragm_size, output_impedance, frequency_response, connector, microphone_type)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10);";
    const SPEAKER_INSERT: &str = "INSERT INTO speaker_item (id, driver, built_in_processing, wireless, max_spl, power, lower_frequency_response, upper_frequency_response, mounting_options, physical_connectivity, network_connectivity)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11);";
    const RF_INSERT: &str = "INSERT INTO rfitem (id, physical_range, lower_frequency_response, upper_frequency_response, transmitter, receiver)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6);";

    pub async fn insert_item(insert: &Item, connection_path: &str) -> Result<(), Error> {
        let mut connection = get_connection(connection_path)
            .await
            .expect("insert_item: Connection failed");

        match &insert.category {
            Categories::GENERIC => (),
            Categories::AMPLIFIER => match &insert.amplifier {
                Some(amplifier) => {
                    let power_bind =
                        serde_json::to_value(amplifier.power.to_owned()).unwrap_or_default();
                    let net_conn_bind =
                        serde_json::to_value(amplifier.network_connectivity.to_owned())
                            .unwrap_or_default();
                    let phys_conn_bind =
                        serde_json::to_value(amplifier.physical_connectivity.to_owned())
                            .unwrap_or_default();

                    sqlx::query!("INSERT INTO amplifier_item (id, total_inputs, total_outputs, midi, physical_connectivity, network_connectivity, signal_protocol, max_sample_rate, power)
                        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);",
                        amplifier.id,
                        amplifier.total_inputs,
                        amplifier.total_outputs,
                        amplifier.midi,
                        phys_conn_bind,
                        net_conn_bind,
                        amplifier.signal_protocol,
                        amplifier.max_sample_rate,
                        power_bind)
                        // .bind(amplifier.id)
                        // .bind(amplifier.total_inputs)
                        // .bind(amplifier.total_outputs)
                        // .bind(amplifier.midi)
                        // .bind(phys_conn_bind)
                        // .bind(net_conn_bind)
                        // .bind(amplifier.signal_protocol)
                        // .bind(amplifier.max_sample_rate)
                        // .bind(power_bind)
                        .execute(&mut connection)
                        .await
                        .unwrap();
                }
                None => (),
            },
            Categories::CONSOLE => match &insert.console {
                Some(console) => {
                    let power_bind = serde_json::to_string(&console.power).unwrap_or_default();
                    match sqlx::query!("INSERT INTO console_item (
                        id, total_inputs, total_outputs, total_busses, physical_inputs, physical_outputs, aux_inputs, physical_aux_inputs, phantom_power_inputs, faders, motorized, midi, protocol_inputs, signal_protocol, can_expand, max_sample_rate, power)
                        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17);",
                        console.id,
                        console.total_inputs,
                        console.total_outputs,
                        console.total_busses,
                        console.physical_inputs,
                        console.physical_outputs,
                        console.aux_inputs,
                        console.physical_aux_inputs,
                        console.phantom_power_inputs,
                        console.faders,
                        console.motorized,
                        console.midi,
                        console.protocol_inputs,
                        console.signal_protocol,
                        console.can_expand,
                        console.max_sample_rate,
                        power_bind)
                        // .bind(console.id)
                        // .bind(console.total_inputs)
                        // .bind(console.total_outputs)
                        // .bind(console.total_busses)
                        // .bind(console.physical_inputs)
                        // .bind(console.physical_outputs)
                        // .bind(console.aux_inputs)
                        // .bind(console.physical_aux_inputs)
                        // .bind(console.phantom_power_inputs)
                        // .bind(console.faders)
                        // .bind(console.motorized)
                        // .bind(console.midi)
                        // .bind(console.protocol_inputs)
                        // .bind(console.signal_protocol)
                        // .bind(console.can_expand)
                        // .bind(console.max_sample_rate)
                        // .bind(power_bind)
                        .execute(&mut connection)
                        .await
                    {
                        Ok(_) => (),
                        Err(err) => println!("{err}"),
                    }
                }
                None => (),
            },
            Categories::COMPUTER => match &insert.computer {
                Some(computer) => {
                    let power_bind =
                        serde_json::to_value(computer.power.to_owned()).unwrap_or_default();
                    let net_conn = serde_json::to_value(computer.network_connectivity.to_owned())
                        .unwrap_or_default();
                    sqlx::query(COMPUTER_INSERT)
                        .bind(computer.id)
                        .bind(computer.cpu_processor.to_owned())
                        .bind(computer.ram_size)
                        .bind(computer.total_storage)
                        .bind(computer.model_year.to_owned())
                        .bind(computer.operating_system.to_owned())
                        .bind(computer.dedicated_graphics)
                        .bind(net_conn)
                        .bind(
                            serde_json::to_value(computer.computer_ports.to_owned())
                                .unwrap_or_default(),
                        )
                        .bind(power_bind)
                        .execute(&mut connection)
                        .await
                        .unwrap();
                }
                None => (),
            },
            Categories::PROCESSOR => match &insert.processor {
                Some(processor) => {
                    let power_bind =
                        serde_json::to_value(processor.power.to_owned()).unwrap_or_default();
                    let net_conn_bind =
                        serde_json::to_value(processor.network_connectivity.to_owned())
                            .unwrap_or_default();
                    let phys_conn_bind =
                        serde_json::to_value(processor.physical_connectivity.to_owned())
                            .unwrap_or_default();
                    sqlx::query(PROCESSOR_ITEM)
                        .bind(processor.id)
                        .bind(processor.total_inputs)
                        .bind(processor.total_outputs)
                        .bind(processor.physical_inputs)
                        .bind(processor.physical_outputs)
                        .bind(processor.midi)
                        .bind(processor.protocol_inputs)
                        .bind(processor.signal_protocol)
                        .bind(processor.max_sample_rate)
                        .bind(net_conn_bind)
                        .bind(phys_conn_bind)
                        .bind(power_bind)
                        .execute(&mut connection)
                        .await
                        .unwrap();
                }
                None => (),
            },
            Categories::MONITORING => match &insert.monitoring_item {
                Some(monitor) => {
                    let power_bind =
                        serde_json::to_value(monitor.power.to_owned()).unwrap_or_default();
                    let net_conn_bind =
                        serde_json::to_value(monitor.network_connectivity.to_owned())
                            .unwrap_or_default();
                    let phys_conn_bind =
                        serde_json::to_value(monitor.physical_connectivity.to_owned())
                            .unwrap_or_default();
                    sqlx::query(MONITORING_INSERT)
                        .bind(monitor.id)
                        .bind(monitor.distro)
                        .bind(net_conn_bind)
                        .bind(phys_conn_bind)
                        .bind(power_bind)
                        .execute(&mut connection)
                        .await
                        .unwrap();
                }
                None => (),
            },
            Categories::SPEAKER => match &insert.speaker_item {
                Some(speaker) => {
                    let power_bind =
                        serde_json::to_value(speaker.power.to_owned()).unwrap_or_default();
                    let net_conn_bind =
                        serde_json::to_value(speaker.network_connectivity.to_owned())
                            .unwrap_or_default();
                    let phys_conn_bind =
                        serde_json::to_value(speaker.physical_connectivity.to_owned())
                            .unwrap_or_default();
                    sqlx::query(SPEAKER_INSERT)
                        .bind(speaker.id)
                        .bind(serde_json::to_value(speaker.driver.to_owned()).unwrap_or_default())
                        .bind(speaker.built_in_processing)
                        .bind(speaker.wireless)
                        .bind(speaker.max_spl)
                        .bind(power_bind)
                        .bind(speaker.lower_frequency_response)
                        .bind(speaker.upper_frequency_response)
                        .bind(
                            serde_json::to_value(speaker.mounting_options.to_owned())
                                .unwrap_or_default(),
                        )
                        .bind(phys_conn_bind)
                        .bind(net_conn_bind)
                        .execute(&mut connection)
                        .await
                        .unwrap();
                }
                None => (),
            },
            // Categories::SPK_HARDWARE => match &insert.spk_hardware {},
            Categories::NETWORK => match &insert.network_item {
                Some(net) => {
                    let power_bind = serde_json::to_value(net.power.to_owned()).unwrap_or_default();
                    let net_conn_bind = serde_json::to_value(net.network_connectivity.to_owned())
                        .unwrap_or_default();
                    sqlx::query(NETWORK_INSERT)
                        .bind(net.id)
                        .bind(net.network_type)
                        .bind(net.poe_ports)
                        .bind(net.max_speed)
                        .bind(net.fiber)
                        .bind(net_conn_bind)
                        .bind(power_bind)
                        .execute(&mut connection)
                        .await
                        .unwrap();
                }
                None => (),
            },
            Categories::RADIO => match &insert.radio_item {
                Some(radio) => {
                    let transmitter_bind =
                        serde_json::to_value(radio.transmitter.to_owned()).unwrap_or_default();
                    let reciver_bind =
                        serde_json::to_value(radio.reciever.to_owned()).unwrap_or_default();
                    sqlx::query(RF_INSERT)
                        .bind(radio.id)
                        .bind(radio.physical_range)
                        .bind(radio.lower_frequency_response)
                        .bind(radio.upper_frequency_response)
                        .bind(transmitter_bind)
                        .bind(reciver_bind)
                        .execute(&mut connection)
                        .await
                        .unwrap();
                }
                None => (),
            },
            Categories::MICROPHONES => match &insert.microphone {
                Some(ref microphone) => {
                    let mic_type_bind = serde_json::to_value(microphone.microphone_type.to_owned())
                        .unwrap_or_default();
                    sqlx::query(MICROPHONE_INSERT)
                        .bind(microphone.id)
                        .bind(microphone.max_spl)
                        .bind(microphone.phantom)
                        .bind(microphone.low_cut)
                        .bind(microphone.pad)
                        .bind(microphone.diaphragm_size)
                        .bind(microphone.output_impedance)
                        .bind(microphone.frequency_response.to_owned())
                        .bind(microphone.connector)
                        .bind(mic_type_bind)
                        .execute(&mut connection)
                        .await
                        .unwrap();
                }
                None => (),
            },
        }

        // let test =
        let table = CreateItem::new(&insert);
        match sqlx::query!("INSERT INTO item (id, created_at, updated_at, public_notes, cost, weight, dimensions, model, category, amplifier_item_id,
            console_item_id, computer_item_id, processor_item_id, network_item_id, microphone_item_id, radio_item_id, speaker_item_id, monitoring_item_id,
             notes)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)"
            ,
            table.id,
            table.created_at,
            table.updated_at,
            table.public_notes,
            table.cost,
            table.weight,
            table.dimensions,
            table.model,
            table.category,
            table.amplifier_item_id,
            table.console_item_id,
            table.computer_item_id,
            table.processor_item_id,
            table.network_item_id,
            table.microphone_item_id,
            table.radio_item_id,
            table.speaker_item_id,
            table.monitoring_item_id,
            table.notes)
            .execute(&mut connection)
            .await
        {
            Ok(res) => println!("{:#?}", res),
            Err(error) => println!("{error}"),
        }

        Ok(())
    }
}

pub mod find {}

pub mod update {}

pub mod delete {}
