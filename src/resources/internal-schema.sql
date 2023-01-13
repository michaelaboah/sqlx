CREATE TABLE IF NOT EXISTS item (
    id INTEGER NOT NULL PRIMARY KEY autoincrement,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    public_notes TEXT NULL,
    cost DOUBLE NOT NULL,
    weight DOUBLE NOT NULL,
    dimensions TEXT NULL,
    model TEXT UNIQUE NOT NULL,
    category INTEGER NOT NULL,
    amplifier_item_id INTEGER NULL,
    console_item_id INTEGER NULL,
    computer_item_id INTEGER NULL,
    processor_item_id INTEGER NULL,
    network_item_id INTEGER NULL,
    microphone_item_id INTEGER NULL,
    radio_item_id INTEGER NULL,
    speaker_item_id INTEGER NULL,
    monitoring_item_id INTEGER NULL,
    searchable_model TEXT NULL,
    notes TEXT NULL,
    CONSTRAINT item_amplifier_id_foreign FOREIGN KEY(amplifier_item_id) REFERENCES amplifier_item(id) ON DELETE
    SET NULL ON UPDATE CASCADE,
        CONSTRAINT item_console_id_foreign FOREIGN KEY(console_item_id) REFERENCES console_item(id) ON DELETE
    SET NULL ON UPDATE CASCADE,
        CONSTRAINT item_computer_id_foreign FOREIGN KEY(computer_item_id) REFERENCES computer_item(id) ON DELETE
    SET NULL ON UPDATE CASCADE,
        CONSTRAINT item_processor_id_foreign FOREIGN KEY(processor_item_id) REFERENCES processor_item(id) ON DELETE
    SET NULL ON UPDATE CASCADE,
        CONSTRAINT item_network_item_id_foreign FOREIGN KEY(network_item_id) REFERENCES network_item(id) ON DELETE
    SET NULL ON UPDATE CASCADE,
        CONSTRAINT item_microphone_id_foreign FOREIGN KEY(microphone_item_id) REFERENCES microphone_item(id) ON DELETE
    SET NULL ON UPDATE CASCADE,
        CONSTRAINT item_radio_item_id_foreign FOREIGN KEY(radio_item_id) REFERENCES rfitem(id) ON DELETE
    SET NULL ON UPDATE CASCADE,
        CONSTRAINT item_speaker_item_id_foreign FOREIGN KEY(speaker_item_id) REFERENCES speaker_item(id) ON DELETE
    SET NULL ON UPDATE CASCADE,
        CONSTRAINT item_monitoring_item_id_foreign FOREIGN KEY(monitoring_item_id) REFERENCES monitoring_item(id) ON DELETE
    SET NULL ON UPDATE CASCADE
);
-- CREATE TABLE IF NOT EXISTS amplifier_item (
--     amplifier_id INTEGER NOT NULL PRIMARY KEY autoincrement,
--     total_inputs INTEGER NOT NULL,
--     total_outputs INTEGER NOT NULL,
--     midi INTEGER NOT NULL,
--     physical_connectivity TEXT NULL,
--     network_connectivity TEXT NULL,
--     signal_protocol INTEGER NOT NULL,
--     max_sample_rate TEXT NOT NULL,
--     power TEXT NULL
-- );
-- CREATE TABLE IF NOT EXISTS computer_item (
--     computer_id INTEGER NOT NULL PRIMARY KEY autoincrement,
--     cpu_processor TEXT NOT NULL,
--     ram_size INTEGER NOT NULL,
--     total_storage INTEGER NOT NULL,
--     model_year TEXT NULL,
--     operating_system TEXT NULL,
--     dedicated_graphics INTEGER NOT NULL,
--     network_connectivity TEXT NULL,
--     computer_ports TEXT NULL,
--     power TEXT NULL
-- );
-- CREATE TABLE IF NOT EXISTS console_item (
--     console_id INTEGER NOT NULL PRIMARY KEY autoincrement,
--     total_inputs INTEGER NOT NULL,
--     total_outputs INTEGER NOT NULL,
--     total_busses INTEGER NOT NULL,
--     physical_inputs INTEGER NOT NULL,
--     physical_outputs INTEGER NOT NULL,
--     aux_inputs INTEGER NOT NULL,
--     physical_aux_inputs INTEGER NOT NULL,
--     phantom_power_inputs INTEGER NOT NULL,
--     faders INTEGER NOT NULL,
--     motorized INTEGER NOT NULL,
--     midi INTEGER NOT NULL,
--     protocol_inputs INTEGER NULL,
--     signal_protocol INTEGER NOT NULL,
--     can_expand INTEGER NULL DEFAULT NULL,
--     max_sample_rate TEXT NOT NULL,
--     power TEXT NULL
-- );
-- CREATE TABLE IF NOT EXISTS microphone_item (
--     id INTEGER NOT NULL PRIMARY KEY autoincrement,
--     max_spl INTEGER NOT NULL,
--     phantom INTEGER NULL,
--     low_cut INTEGER NULL,
--     pad INTEGER NULL,
--     diaphragm_size INTEGER NULL,
--     output_impedance INTEGER NULL,
--     frequency_response TEXT NULL,
--     connector INTEGER NOT NULL,
--     microphone_type TEXT NOT NULL
-- );
-- CREATE TABLE IF NOT EXISTS monitoring_item (
--     monitoring_id INTEGER NOT NULL PRIMARY KEY autoincrement,
--     distro INTEGER NULL,
--     network_connectivity TEXT NULL,
--     physical_connectivity TEXT NULL,
--     power TEXT NULL
-- );
-- CREATE TABLE IF NOT EXISTS network_item (
--     network_id INTEGER NOT NULL PRIMARY KEY autoincrement,
--     network_type INTEGER NOT NULL,
--     poe_ports INTEGER NOT NULL,
--     max_speed INTEGER NOT NULL,
--     fiber INTEGER NULL,
--     network_connectivity TEXT NULL,
--     power TEXT NULL
-- );
-- CREATE TABLE IF NOT EXISTS processor_item (
--     processor_id INTEGER NOT NULL PRIMARY KEY autoincrement,
--     total_inputs INTEGER NOT NULL,
--     total_outputs INTEGER NOT NULL,
--     physical_inputs INTEGER NOT NULL,
--     physical_outputs INTEGER NOT NULL,
--     midi INTEGER NULL,
--     protocol_inputs INTEGER NULL,
--     signal_protocol INTEGER NOT NULL,
--     max_sample_rate TEXT NOT NULL,
--     network_connectivity TEXT NULL,
--     physical_connectivity TEXT NULL,
--     power TEXT NULL
-- );
-- CREATE TABLE IF NOT EXISTS rf_item (
--     rf_id INTEGER NOT NULL PRIMARY KEY autoincrement,
--     physical_range INTEGER NOT NULL,
--     lower_frequency_response INTEGER NOT NULL,
--     upper_frequency_response INTEGER NOT NULL,
--     transmitter TEXT NULL,
--     reciever TEXT NULL
-- );
-- CREATE TABLE IF NOT EXISTS speaker_item (
--     speaker_id INTEGER NOT NULL PRIMARY KEY autoincrement,
--     driver TEXT NOT NULL,
--     built_in_processing INTEGER NOT NULL,
--     wireless INTEGER NOT NULL,
--     max_spl INTEGER NOT NULL,
--     lower_frequency_response INTEGER NOT NULL,
--     upper_frequency_response INTEGER NOT NULL,
--     mounting_options TEXT NOT NULL,
--     physical_connectivity TEXT NULL,
--     network_connectivity TEXT NULL,
--     power TEXT NOT NULL
-- );
-- CREATE TABLE IF NOT EXISTS rfband (
--     band_id INTEGER NOT NULL PRIMARY KEY autoincrement,
--     rf_item_id INTEGER NOT NULL,
--     band_name TEXT NOT NULL,
--     lower_frequency_range INTEGER NOT NULL,
--     upper_frequency_range INTEGER NOT NULL,
--     manufacturer TEXT NOT NULL,
--     nation_code INTEGER NOT NULL,
--     deprecated INTEGER NOT NULL,
--     CONSTRAINT rfband_rf_item_id_foreign FOREIGN KEY(rf_item_id) REFERENCES rfitem(id) ON UPDATE CASCADE
-- );
-- CREATE INDEX IF NOT EXISTS rfband_rf_item_id_index ON rfband (rf_item_id);
-- CREATE UNIQUE INDEX IF NOT EXISTS rfband_band_name_unique ON rfband (band_name);
-- CREATE UNIQUE INDEX IF NOT EXISTS item_model_unique ON item (model);
-- CREATE UNIQUE INDEX IF NOT EXISTS item_amplifier_item_id_unique ON item (amplifier_item_id);
-- CREATE UNIQUE INDEX IF NOT EXISTS item_console_item_id_unique ON item (console_item_id);
-- CREATE UNIQUE INDEX IF NOT EXISTS item_computer_item_id_unique ON item (computer_item_id);
-- CREATE UNIQUE INDEX IF NOT EXISTS item_processor_item_id_unique ON item (processor_item_id);
-- CREATE UNIQUE INDEX IF NOT EXISTS item_network_item_id_unique ON item (network_item_id);
-- CREATE UNIQUE INDEX IF NOT EXISTS item_microphone_item_id_unique ON item (microphone_item_id);
-- CREATE UNIQUE INDEX IF NOT EXISTS item_radio_item_id_unique ON item (radio_item_id);
-- CREATE UNIQUE INDEX IF NOT EXISTS item_speaker_item_id_unique ON item (speaker_item_id);
-- CREATE UNIQUE INDEX IF NOT EXISTS item_monitoring_item_id_unique ON item (monitoring_item_id);
-- PRAGMA integrity_check;