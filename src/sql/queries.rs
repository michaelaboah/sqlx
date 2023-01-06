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
        created_at datetime NOT NULL DEFAULT 'NOW()',
        updated_at datetime NOT NULL,
        public_notes text NULL,
        cost DOUBLE NULL,
        weight DOUBLE NULL,
        dimensions JSON NULL,
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
        notes JSON NULL,
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
                physical_connectivity JSON NULL,
                network_connectivity JSON NULL,
                signal_protocol integer NOT NULL,
                max_sample_rate text CHECK (
                    max_sample_rate in (
                        'SD',
                        'HD',
                        'UHD'
                    )
                ) NOT NULL,
                power JSON NULL
            );";
    const COMPUTER_ITEM_TABLE: &str = "CREATE TABLE IF NOT EXISTS computer_item (
        id integer NOT NULL PRIMARY KEY autoincrement,
        cpu_processor text NOT NULL,
        ram_size integer NOT NULL,
        total_storage integer NOT NULL,
        model_year text NULL,
        operating_system text NULL,
        dedicated_graphics integer NOT NULL,
        network_connectivity JSON NULL,
        computer_ports JSON NULL,
        power JSON NULL
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
            power JSON NULL
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
            network_connectivity JSON NULL,
            physical_connectivity JSON NULL,
            power JSON NULL
        );";

    const NETWORK_ITEM: &str = "CREATE TABLE IF NOT EXISTS network_item (
            id integer NOT NULL PRIMARY KEY autoincrement,
            network_type integer NOT NULL,
            poe_ports integer NOT NULL,
            max_speed integer NOT NULL,
            fiber integer NULL,
            network_connectivity JSON NULL,
            power JSON NULL
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
        network_connectivity JSON NULL,
        physical_connectivity JSON NULL,
        power JSON NULL
        );";

    const RADIO_ITEM: &str = "CREATE TABLE IF NOT EXISTS rfitem (
            id integer NOT NULL PRIMARY KEY autoincrement,
            physical_range integer NOT NULL,
            lower_frequency_response integer NOT NULL,
            upper_frequency_response integer NOT NULL,
            transmitter JSON NULL,
            reciever JSON NULL
        );";
    const SPEAKER_ITEM: &str = "CREATE TABLE IF NOT EXISTS speaker_item (
            id integer NOT NULL PRIMARY KEY autoincrement,
            driver JSON NOT NULL,
            built_in_processing integer NOT NULL,
            wireless integer NOT NULL,
            max_spl integer NOT NULL,
            power JSON NOT NULL,
            lower_frequency_response integer NOT NULL,
            upper_frequency_response integer NOT NULL,
            mounting_options text NOT NULL,
            physical_connectivity JSON NULL,
            network_connectivity JSON NULL
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
    use serde::de::value::Error;

    use crate::{
        sql::{database_setup::sql_setup::get_connection, entities::structs::Item},
        DB_PATH,
    };

    pub async fn insert_item(insert: &mut Item) -> Result<(), Error> {
        let mut connection = get_connection(DB_PATH)
            .await
            .expect("insert_item: Connection failed");

        sqlx::query("INSERT INTO item (created_at, updated_at, public_notes, cost, weight, dimensions, model, category, amplifier_item_id, console_item_id, computer_item_id, processor_item_id, network_item_id, microphone_item_id, radio_item_id, speaker_item_id, monitoring_item_id, notes)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)").execute(&mut connection).await.unwrap();
        Ok(())
    }
}

pub mod find {}

pub mod update {}

pub mod delete {}
