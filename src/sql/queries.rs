pub mod insertion {
    use crate::sql::entities::{creation_structs::CreateItem, enums::Categories, structs::Item};

    use serde::de::value::Error;
    use sqlx::sqlite::Sqlite;
    use sqlx::Pool;

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
    const RF_INSERT: &str = "INSERT INTO rf_item (id, physical_range, lower_frequency_response, upper_frequency_response, transmitter, receiver)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6);";

    pub async fn insert_single_item(insert: &Item, pool: &Pool<Sqlite>) -> Result<(), Error> {
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

                    sqlx::query!("INSERT INTO amplifier_item (amplifier_id, total_inputs, total_outputs, midi, physical_connectivity, network_connectivity, signal_protocol, max_sample_rate, power)
                        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);",
                        amplifier.id,
                        amplifier.total_inputs,
                        amplifier.total_outputs,
                        amplifier.midi,
                        phys_conn_bind,
                        net_conn_bind,
                        amplifier.signal_protocol,
                        amplifier.max_sample_rate,
                        power_bind
                    )
                        // .bind(amplifier.id)
                        // .bind(amplifier.total_inputs)
                        // .bind(amplifier.total_outputs)
                        // .bind(amplifier.midi)
                        // .bind(phys_conn_bind)
                        // .bind(net_conn_bind)
                        // .bind(amplifier.signal_protocol)
                        // .bind(amplifier.max_sample_rate)
                        // .bind(power_bind)
                        .execute(pool)
                        .await
                        .unwrap();
                }
                None => (),
            },
            Categories::CONSOLE => match &insert.console {
                Some(console) => {
                    let power_bind = serde_json::to_string(&console.power).unwrap_or_default();
                    match sqlx::query!("INSERT INTO console_item (
                        console_id, total_inputs, total_outputs, total_busses, physical_inputs, physical_outputs, aux_inputs, physical_aux_inputs, phantom_power_inputs, faders, motorized, midi, protocol_inputs, signal_protocol, can_expand, max_sample_rate, power)
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
                        power_bind
                    )
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
                        .execute(pool)
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
                        .execute(pool)
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
                        .execute(pool)
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
                        .execute(pool)
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
                        .execute(pool)
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
                        .execute(pool)
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
                        .execute(pool)
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
                        .execute(pool)
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
            table.notes
        )

        // .bind(table.id)
        // .bind(table.created_at)
        // .bind(table.updated_at)
        // .bind(table.public_notes)
        // .bind(table.cost)
        // .bind(table.weight)
        // .bind(table.dimensions)
        // .bind(table.model)
        // .bind(table.category)
        // .bind(table.amplifier_item_id)
        // .bind(table.console_item_id)
        // .bind(table.computer_item_id)
        // .bind(table.processor_item_id)
        // .bind(table.network_item_id)
        // .bind(table.microphone_item_id)
        // .bind(table.radio_item_id)
        // .bind(table.speaker_item_id)
        // .bind(table.monitoring_item_id)
        // .bind(table.notes)
            .execute(pool)
            .await
        {
            Ok(res) => println!("{:#?}", res),
            Err(error) => println!("{error}"),
        }

        Ok(())
    }

    pub async fn insert_multiple_items(
        inserts: Vec<Item>,
        pool: &Pool<Sqlite>,
    ) -> Result<(), Error> {
        for insert in inserts.iter() {
            insert_single_item(insert, pool);
        }
        Ok(())
    }
}

pub mod find {
    use crate::sql::entities::creation_structs::CreateItem;
    use sqlx::sqlite::Sqlite;
    use sqlx::Pool;

    pub async fn find_similar_item(model: &str, pool: &Pool<Sqlite>) -> Vec<CreateItem> {
        let formatted = format!("%{model}%");
        let similar_items = sqlx::query_as!(
            CreateItem,
            "SELECT * from item WHERE model LIKE ?",
            formatted
        )
        // .bind(formatted)
        .fetch_all(pool)
        .await
        .expect("Fetch error at find_one_item.");

        similar_items
    }

    pub async fn find_all_items(pool: &Pool<Sqlite>) -> Vec<CreateItem> {
        let all_items = sqlx::query_as!(CreateItem, "SELECT * FROM item;")
            .fetch_all(pool)
            .await
            .expect("Fetch error at find_all_items;");
        all_items
    }

    pub async fn find_single_item(id: i64, pool: &Pool<Sqlite>) -> CreateItem {
        let single_item = sqlx::query_as!(CreateItem, "SELECT * FROM item WHERE id = ?", id)
            // .bind(id)
            .fetch_one(pool)
            .await
            .expect("Fetch error at find_one_item;");
        single_item
    }

    pub async fn fuzzy_find_single_item(model: &str, pool: &Pool<Sqlite>) -> CreateItem {
        let formatted = format!("%{model}%");

        let single_item = sqlx::query_as!(
            CreateItem,
            "SELECT * FROM item WHERE model LIKE ?",
            formatted
        )
        // .bind(formatted)
        .fetch_one(pool)
        .await
        .expect("Fetch error at fuzzy_find_one_item;");
        single_item
    }
}

pub mod update {}

pub mod delete {
    use sqlx::sqlite::{Sqlite, SqliteQueryResult};
    use sqlx::Pool;

    pub async fn delete_all_items(pool: &Pool<Sqlite>) -> sqlx::Result<SqliteQueryResult> {
        sqlx::query!("DELETE FROM item;").execute(pool).await
    }

    pub async fn delete_single_item(
        id: i64,
        pool: &Pool<Sqlite>,
    ) -> sqlx::Result<SqliteQueryResult> {
        sqlx::query!("DELETE FROM item WHERE id = ?", id)
            // .bind(id)
            .execute(pool)
            .await
    }

    pub async fn fuzzy_delete_single_item(
        model: &str,
        pool: &Pool<Sqlite>,
    ) -> sqlx::Result<SqliteQueryResult> {
        let formatted = format!("%{model}%");
        sqlx::query!("DELETE FROM item WHERE model LIKE ?", formatted)
            // .bind(formatted)
            .execute(pool)
            .await
    }
}
