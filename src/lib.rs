extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

extern crate rusqlite;

use rusqlite::{params, Connection};

extern crate libc_sys;


extern crate js_sys;


#[derive(Debug, Clone)]
struct Person {
    id: i32,
    name: String,
    time_created: f64,
    data: Option<Vec<u8>>,
}

#[wasm_bindgen]
pub fn start() {
    wasm_println::hook();
    println!("Sqlite Version {:?}", rusqlite::version());
    println!();

    match Connection::open_in_memory() {
        Ok(conn) => {
            println!("Creating TABLE: person
            CREATE TABLE person (
                          id              INTEGER PRIMARY KEY,
                          name            TEXT NOT NULL,
                          time_created    INTEGER NOT NULL,
                          data            BLOB
                          )");
            println!();
            conn.execute(
                "CREATE TABLE person (
                          id              INTEGER PRIMARY KEY,
                          name            TEXT NOT NULL,
                          time_created    INTEGER NOT NULL,
                           data            BLOB
                           )",
                params![],
            ).expect("CREATE TABLE statement did not execute.");


            let me = Person {
                id: 0,
                name: "Person ".to_string(),
                time_created: js_sys::Date::new_0().value_of(),
                data: None,
            };
            println!("Inserting into TABLE: \
            INSERT INTO person(name,time_created,data) VALUES (?1, ?2, ?3,?4)");

            for i in 0..10 {
                conn.execute(
                    "INSERT INTO person (id,name, time_created, data)
                                   VALUES (?1, ?2, ?3,?4)",
                    params![
                        i,
                        me.name.clone() + &i.to_string().clone(),
                       js_sys::Date::new_0().value_of(),
                        me.data
                    ],
                ).expect("Error inserting record.");
            }
            println!("Querying person table : \
            SELECT id,name,time_created,data FROM person");
            let mut stmt = conn
                .prepare("SELECT id, name, time_created, data FROM person")
                .expect("Error preparing statement.");

            let person_iter = stmt
                .query_map(params![], |row| {
                    Ok(Person {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        time_created: row.get(2)?,
                        data: row.get(3)?,
                    })
                })
                .expect("Select query failed");

            for p in person_iter {
                let person = p.expect("Could not unwrap person").clone();
                println!("{:?}", person);
            }
            println!("Done");
        }
        Err(e) => println!("{:?}", e),
    }
}
