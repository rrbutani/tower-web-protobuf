//! Some auxiliary stuff for end-to-end tests:

#[macro_use] extern crate prost;
#[macro_use] extern crate tower_web;

use tower_web_protobuf::Proto;
use std::collections::HashMap;

// Messages:

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct Track {
    #[prost(string, tag="1")]
    pub name: String,
    #[prost(float)]
    pub length: f32,
    #[prost(float)]
    pub number: f32,
    #[prost(uint32)]
    pub id: u32,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct Album {
    #[prost(string, tag="1")]
    pub name: String,
    #[prost(uint32)]
    pub id: u32,
    #[prost(enumeration="AlbumType")]
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

#[derive(Clone, Debug)]
pub struct MusicService {
    db: HashMap<String, Album>,
}

impl MusicService {
    fn new() -> Self {
        Self {
            db: HashMap::new()
        }
    }
}

type In<M> = Proto<M>;
type Out<M> = Result<Proto<M>, ()>;

impl_web! {
    impl MusicService {
        #[get("/identity/track/")]
        fn track_ident(&self, track: In<Track>) -> Out<Track> { Ok(track) }

        #[get("/identity/album/")]
        fn album_ident(&self, album: In<Album>) -> Out<Album> { Ok(album) }

        #[get("/add/album/")]
        fn add_album(&mut self, album: In<Album>) -> Result<Proto<Album>, String> {
            self.db.insert(album.name, album.into())
                .map(|a| a.into())
                .ok_or("This is a new album!".into())
        }
    }
}



