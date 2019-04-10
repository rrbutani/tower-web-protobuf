//! Some auxiliary stuff for end-to-end tests:

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::panic;
use std::sync::Mutex;

use ::prost::{Enumeration, Message};
use tower_web::ServiceBuilder;
use tower_web::{impl_web, Deserialize, Serialize, *};
use tower_web_protobuf::{Proto, ProtobufMiddleware};

// Messages:

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct Track {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(float)]
    pub length: f32,
    #[prost(uint32)]
    pub number: u32,
    #[prost(uint32)]
    pub id: u32,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct Album {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(uint32)]
    pub id: u32,
    #[prost(enumeration = "AlbumType")]
    pub album_type: i32,
    #[prost(message, repeated)]
    pub tracks: Vec<Track>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enumeration, Serialize, Deserialize)]
pub enum AlbumType {
    Single = 0,
    Ep = 1,
    Lp = 2,
    Playlist = 3, // 🙄
}

// A silly service:

#[derive(Debug)]
pub struct MusicService {
    db: Mutex<HashMap<String, Album>>,
}

impl MusicService {
    fn new() -> Self {
        Self {
            db: Mutex::new(HashMap::new()),
        }
    }
}

type In<M> = Proto<M>;
type Out<M> = Result<Proto<M>, ()>;
type Res<M, E> = Result<Proto<M>, E>;

impl_web! {
    impl MusicService {
        #[get("/identity/track/")]
        fn track_ident(&self, track: In<Track>) -> Out<Track> { Ok(track) }

        #[get("/identity/album/")]
        fn album_ident(&self, album: In<Album>) -> Out<Album> { Ok(album) }

        #[post("/add/album/")]
        fn add_album(&self, album: In<Album>) -> Res<Album, String> {
            self.db.lock()
                .unwrap()
                .insert(album.name.clone(), album.move_inner())
                .map(|a| a.into())
                .ok_or("This is a new album!".into())
        }

        #[get("/query/album/:album_name")]
        fn get_album(&self, album_name: String) -> Res<Album, String> {
            self.db.lock()
                .unwrap()
                .get(&album_name)
                .map(|a| a.clone().into())
                .ok_or("No such album found!".into())
        }

        #[get("/track/length/")]
        fn track_length(&self, track: In<Track>) -> Result<String, ()> {
            Ok(format!("{}", track.length))
        }
    }
}

// Some handy helper functions:

fn setup(options: (bool, bool), socket: &SocketAddr) {
    ServiceBuilder::new()
        .resource(MusicService::new())
        .middleware(ProtobufMiddleware::new(options.0, options.1))
        .run(&socket)
        .unwrap();
}

fn get_next_unused_port() -> Option<u16> {
    (1025..65535).find(|p| TcpListener::bind(("127.0.0.1", *p)).is_ok())
}

pub fn run_service_test(
    options: (bool, bool),
    test_fn: impl FnOnce(&SocketAddr) + panic::UnwindSafe,
) {
    // Setup:
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket = SocketAddr::new(ip, get_next_unused_port().unwrap());

    std::thread::spawn(move || setup(options, &socket));

    let result = panic::catch_unwind(|| test_fn(&socket));

    // <teardown>

    assert!(result.is_ok())
}
