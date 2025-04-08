use {
    crate::{user::SessionUser, AppData},
    actix_session::Session,
    actix_web::{error::ErrorNotFound, get, post, rt, web, Error, HttpRequest, Responder},
    actix_ws::Message,
    futures_util::StreamExt,
    serde::{Deserialize, Serialize},
    std::sync::atomic::AtomicU32,
    tokio::sync::RwLock,
};

static ROOM_ID_INCREMENT: AtomicU32 = AtomicU32::new(1);
static SOCKET_ID_INCREMENT: AtomicU32 = AtomicU32::new(1);

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct RoomClient {
    id: u32,
    user_id: u32,
    name: String,
    #[serde(skip)]
    socket: actix_ws::Session,
}

impl RoomClient {
    async fn send_message(&mut self, message: &str) {
        self.socket.text(message).await.unwrap_or_else(|_| {
            log::error!(
                "Failed to send message to client {}: {}",
                self.user_id,
                message
            );
        });
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    id: u32,
    name: String,
    members: Vec<RoomClient>,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Room {}

impl PartialOrd for Room {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Room {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl Room {
    async fn send_message(&mut self, message: &str, author: Option<u32>) {
        for member in &mut self.members {
            if Some(member.id) != author {
                member.send_message(message).await;
            }
        }
    }

    #[must_use = "this `bool` must be used to delete the room if it's empty"]
    async fn remove_member(&mut self, id: u32) -> bool {
        if let Some(index) = self.members.iter().position(|member| member.id == id) {
            let member = self.members.remove(index);
            let _ = member.socket.close(None).await;
        }
        self.members.is_empty()
    }
}

#[get("/api/rooms/{id}/ws")]
async fn connect(
    req: HttpRequest,
    body: web::Payload,
    id: web::Path<u32>,
    session: Session,
) -> Result<impl Responder, Error> {
    let user = SessionUser::try_from(&session)?;
    let room_id = id.into_inner();
    let (res, mut socket, mut stream) = actix_ws::handle(&req, body)?;

    let rooms_guard = AppData::get().rooms.read().await;
    let room = rooms_guard
        .get(&room_id)
        .ok_or_else(|| ErrorNotFound(format!("Room with id {room_id} not found")))?;

    let ws_id = SOCKET_ID_INCREMENT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    {
        let mut room = room.write().await;
        room.members.push(RoomClient {
            id: ws_id,
            user_id: user.id,
            name: user.name,
            socket: socket.clone(),
        });
    }

    rt::spawn(async move {
        loop {
            let msg = tokio::select! {
                _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => break,
                msg = stream.next() => match msg {
                    Some(Ok(msg)) => msg,
                    _ => break,
                }
            };
            match msg {
                Message::Text(text) => {
                    // ping
                    if text.starts_with("0;") {
                        continue;
                    }
                    let rooms_guard = AppData::get().rooms.read().await;
                    let room = match rooms_guard.get(&room_id) {
                        Some(room) => room,
                        None => break,
                    };
                    let mut room = room.write().await;
                    room.send_message(&text, Some(ws_id)).await;
                }
                Message::Ping(msg) => {
                    if socket.pong(&msg).await.is_err() {
                        break;
                    };
                }
                Message::Binary(_) | Message::Nop | Message::Continuation(_) | Message::Pong(_) => {
                    ()
                }
                Message::Close(_) => break,
            }
        }

        let rooms_guard = AppData::get().rooms.read().await;
        if let Some(room) = rooms_guard.get(&room_id) {
            if room.write().await.remove_member(ws_id).await {
                drop(rooms_guard);
                AppData::get().rooms.write().await.remove(&room_id);
            }
        }
    });

    Ok(res)
}

#[get("/api/rooms/{id}")]
async fn get(session: Session, id: web::Path<u32>) -> Result<impl Responder, Error> {
    SessionUser::try_from(&session)?;
    let id = id.into_inner();
    
    let rooms_guard = AppData::get().rooms.read().await;
    if let Some(room) = rooms_guard.get(&id) {
        return Ok(web::Json(room.read().await.clone()));
    }
    Err(ErrorNotFound(format!("Room with id {id} not found")))
}

#[derive(Deserialize, Serialize, Debug)]
struct NewRoom {
    name: String,
}

#[post("/api/rooms")]
async fn new(session: Session, new_room: web::Json<NewRoom>) -> Result<impl Responder, Error> {
    SessionUser::try_from(&session)?;

    let id = ROOM_ID_INCREMENT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let new_room = Room {
        id,
        name: new_room.name.clone(),
        members: Vec::new(),
    };

    let mut rooms_guard = AppData::get().rooms.write().await;
    rooms_guard.insert(id, RwLock::new(new_room.clone()));

    Ok(web::Json(new_room))
}

#[get("/api/rooms")]
async fn list(session: Session) -> Result<impl Responder, Error> {
    SessionUser::try_from(&session)?;

    let rooms_guard = AppData::get().rooms.read().await;
    let mut rooms: Vec<Room> = Vec::with_capacity(rooms_guard.len());
    for room in rooms_guard.values() {
        rooms.push(room.read().await.clone());
    }
    rooms.sort();
    Ok(web::Json(rooms))
}

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(connect);
    cfg.service(new);
    cfg.service(list);
    cfg.service(get);
}
