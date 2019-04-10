//! Tries some roundtrip end-to-end tests that should succeed.
//!
//! Checks that content-type headers are set correctly too.

mod common;

use common::*;
use reqwest::{Client, StatusCode};
use std::io::Read;
use std::net::SocketAddr;
use tower_web_protobuf::types::MessagePlus;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Format {
    Protobuf,
    Json,
}

impl Format {
    fn get_header(self) -> &'static str {
        match self {
            Format::Protobuf => "application/protobuf",
            Format::Json => "application/json",
        }
    }

    fn encode<M: MessagePlus>(self, data: &M) -> Vec<u8> {
        match self {
            Format::Protobuf => {
                let mut buf = Vec::with_capacity(data.encoded_len());
                data.encode(&mut buf).unwrap();

                buf
            }
            Format::Json => serde_json::to_vec_pretty(&data).unwrap(),
        }
    }

    fn decode<M: MessagePlus>(self, data: &[u8]) -> M {
        match self {
            Format::Protobuf => M::decode(data).unwrap(),
            Format::Json => serde_json::from_slice(data).unwrap(),
        }
    }
}

fn identity_test<T: MessagePlus + PartialEq + Clone>(
    uri: String,
    send: Format,
    receive: Format,
    socket: SocketAddr,
) -> impl Fn(T) {
    move |data: T| {
        let mut buf = Vec::with_capacity(data.encoded_len());
        data.encode(&mut buf).unwrap();

        let mut resp = Client::new()
            .get(format!("http://{}:{}{}", socket.ip(), socket.port(), uri).as_str())
            .header("Content-Type", send.get_header())
            .header("Accept", receive.get_header())
            .body(send.encode(&data))
            .send()
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers()
                .get("Content-Type")
                .unwrap()
                .to_str()
                .unwrap(),
            receive.get_header()
        );

        let mut buf = Vec::new();
        assert!(resp.read_to_end(&mut buf).is_ok());
        assert_eq!(data, receive.decode(&mut buf));
    }
}

#[test]
fn identity_tests() {
    run_service_test((true, true), |socket| {
        use Format::*;

        const FORMATS: [Format; 2] = [Json, Protobuf];

        fn endpoint_test<T: MessagePlus + PartialEq + Clone>(
            endpoint: &'static str,
            socket: SocketAddr,
            val: T,
        ) {
            FORMATS.iter().for_each(|inp| {
                FORMATS
                    .iter()
                    .for_each(|out| identity_test(endpoint.into(), *inp, *out, socket)(val.clone()))
            });
        }

        endpoint_test(
            "/identity/track/",
            *socket,
            Track {
                name: "4′33″".into(),
                length: (4.0 * 60.0 + 33.333),
                number: 1,
                id: 0,
            },
        );

        endpoint_test(
            "/identity/album/",
            *socket,
            Album {
                name: "In Colour".into(),
                id: 2015,
                album_type: 2,
                tracks: vec![
                    Track {
                        name: "Sleep Sound".into(),
                        length: (3 * 60 + 52) as f32,
                        number: 2,
                        id: 947,
                    },
                    Track {
                        name: "Loud Places".into(),
                        length: (4 * 60 + 43) as f32,
                        number: 8,
                        id: 1056,
                    },
                ],
            },
        );
    });
}
