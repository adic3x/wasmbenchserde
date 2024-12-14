use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rust() {
    if let Err(e) = process() {
        web_sys::console::log_1(&format!("{e:#?}").into());
    }
}

fn process() -> Result<(), Box<dyn std::error::Error>> {
    let object = Object {
        id: 1,
        name: "Tester".into(),
        data: 0.333,
        x: 1258,
        y: 5589,
        z: 5103,
        description: "Test Object Test Object Test Object \"1337\"".into(),
        active: true,
        posts_id_1: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 500000, 600000, 7000000],
        posts_id_2: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 500000, 600000, 7000000],
        posts_id_3: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 500000, 600000, 7000000],
        posts_id_4: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 500000, 600000, 7000000],
        x_data: vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 500000, 600000, 7000000,1, 2, 3, 4, 5,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 500000, 600000, 7000000,1, 2, 3, 4, 5,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 500000, 600000, 7000000,1, 2, 3, 4, 5,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 500000, 600000, 7000000,1, 2, 3, 4, 5,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 500000, 600000, 7000000,1, 2, 3, 4, 5,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 500000, 600000, 7000000,1, 2, 3, 4, 5,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 500000, 600000, 7000000,1, 2, 3, 4, 5,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 500000, 600000, 7000000,1, 2, 3, 4, 5,
        ],
        names: vec!["Albert".into(), "Grigor".into(), "Tor".into(), "Jonh".into(), "Henry 25".into()],
    };

    let array = vec![&object; 65536];

    let json_data = serde_json::to_string(&array)?;
    let msgpack_data = rmp_serde::to_vec_named(&array)?;

    web_sys::console::log_1(&"Rust console.time".into());
    web_sys::console::time();
    web_sys::console::time_end();

    web_sys::console::log_1(&"Rust JSON".into());
    web_sys::console::time();
    let json_deserialized = serde_json::from_str::<Vec<Object>>(&json_data)?;
    web_sys::console::time_end();
   
    web_sys::console::log_1(&"Rust MsgPack".into());
    web_sys::console::time();
    let msgpack_deserialized = rmp_serde::from_slice::<Vec<Object>>(&msgpack_data)?;
    web_sys::console::time_end();

    web_sys::console::log_1(&"Rust MsgPack ZeroCopy".into());
    web_sys::console::time();
    let msgpack_deserialized_zero_copy = rmp_serde::from_slice::<Vec<ObjectZeroCopy>>(&msgpack_data)?;
    web_sys::console::time_end();

    // web_sys::console::log_1(&json_data[0..511].into());
    // web_sys::console::log_1(&format!("{:?}", &msgpack_data[0..511]).into());
    web_sys::console::log_1(&json_deserialized[0].name.as_str().into());
    web_sys::console::log_1(&msgpack_deserialized[0].name.as_str().into());
    web_sys::console::log_1(&msgpack_deserialized_zero_copy[0].name.into());

    web_sys::console::log_1(&serde_json::to_string(&[&object])?.into());
    web_sys::console::log_1(&format!("{:?}", rmp_serde::to_vec_named(&[&object])?).into());

    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Object {
    id: u64,
    name: String,
    data: f64,
    x: u64,
    y: u64,
    z: u64,
    description: String,
    active: bool,
    posts_id_1: Vec<u64>,
    posts_id_2: Vec<u64>,
    posts_id_3: Vec<u64>,
    posts_id_4: Vec<u64>,
    x_data: Vec<u64>,
    names: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct ObjectZeroCopy<'a> {
    id: u64,
    name: &'a str,
    data: f64,
    x: u64,
    y: u64,
    z: u64,
    description: &'a str,
    active: bool,
    posts_id_1: Vec<u64>,
    posts_id_2: Vec<u64>,
    posts_id_3: Vec<u64>,
    posts_id_4: Vec<u64>,
    x_data: Vec<u64>,
    names: Vec<&'a str>,
}