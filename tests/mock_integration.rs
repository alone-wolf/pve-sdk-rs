use std::sync::{Arc, Mutex};
use std::time::Duration;

use pve_sdk_rs::{ClientOption, PveError, WaitTaskOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;
use tokio::time::timeout;

struct MockServer {
    port: u16,
    handle: JoinHandle<()>,
}

impl MockServer {
    fn port(&self) -> u16 {
        self.port
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

struct MockResponse {
    status_code: u16,
    reason_phrase: &'static str,
    content_type: &'static str,
    body: String,
}

impl MockResponse {
    fn json(status_code: u16, reason_phrase: &'static str, body: &str) -> Self {
        Self {
            status_code,
            reason_phrase,
            content_type: "application/json",
            body: body.to_string(),
        }
    }

    fn text(status_code: u16, reason_phrase: &'static str, body: &str) -> Self {
        Self {
            status_code,
            reason_phrase,
            content_type: "text/plain",
            body: body.to_string(),
        }
    }
}

async fn spawn_mock_server<F>(responder: F) -> MockServer
where
    F: Fn(&str, &str) -> MockResponse + Send + Sync + 'static,
{
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind listener");
    let port = listener.local_addr().expect("listener addr").port();
    let responder = Arc::new(responder);

    let handle = tokio::spawn(async move {
        while let Ok((mut socket, _addr)) = listener.accept().await {
            let responder = Arc::clone(&responder);
            tokio::spawn(async move {
                let Some(raw_request) = read_request(&mut socket).await else {
                    return;
                };

                let (method, path) = parse_request_line(&raw_request);
                let response = responder(&method, &path);
                write_response(&mut socket, response).await;
            });
        }
    });

    MockServer { port, handle }
}

async fn read_request(socket: &mut TcpStream) -> Option<String> {
    let mut buffer = Vec::with_capacity(1024);
    let mut chunk = [0_u8; 1024];

    loop {
        let read = timeout(Duration::from_secs(2), socket.read(&mut chunk))
            .await
            .ok()?
            .ok()?;

        if read == 0 {
            return None;
        }

        buffer.extend_from_slice(&chunk[..read]);
        if buffer.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }

        if buffer.len() > 64 * 1024 {
            return None;
        }
    }

    Some(String::from_utf8_lossy(&buffer).into_owned())
}

fn parse_request_line(raw_request: &str) -> (String, String) {
    let line = raw_request.lines().next().unwrap_or_default();
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or_default().to_string();
    let path = parts.next().unwrap_or_default().to_string();
    (method, path)
}

async fn write_response(socket: &mut TcpStream, response: MockResponse) {
    let body_len = response.body.len();
    let head = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        response.status_code, response.reason_phrase, response.content_type, body_len
    );

    let _ = socket.write_all(head.as_bytes()).await;
    let _ = socket.write_all(response.body.as_bytes()).await;
    let _ = socket.shutdown().await;
}

async fn build_client(port: u16) -> pve_sdk_rs::PveClient {
    ClientOption::new("127.0.0.1")
        .port(port)
        .https(false)
        .auth_none()
        .build()
        .await
        .expect("build client")
}

#[tokio::test]
async fn version_parses_data_envelope() {
    let requested_paths = Arc::new(Mutex::new(Vec::<String>::new()));
    let paths = Arc::clone(&requested_paths);

    let server = spawn_mock_server(move |_method, path| {
        paths
            .lock()
            .expect("capture request path")
            .push(path.to_string());
        MockResponse::json(200, "OK", r#"{"data":{"version":"8.2.1","release":"8.2"}}"#)
    })
    .await;

    let client = build_client(server.port()).await;
    let version = client.version().await.expect("version response");

    assert_eq!(version.version, "8.2.1");
    assert_eq!(version.release.as_deref(), Some("8.2"));

    let paths = requested_paths.lock().expect("paths lock");
    assert!(paths.iter().any(|path| path == "/api2/json/version"));
}

#[tokio::test]
async fn api_status_error_surfaces_401_body() {
    let server = spawn_mock_server(|_method, _path| {
        MockResponse::json(401, "Unauthorized", r#"{"error":"token rejected"}"#)
    })
    .await;

    let client = build_client(server.port()).await;
    let err = client.version().await.expect_err("expected auth error");

    match err {
        PveError::ApiStatus { status, body } => {
            assert_eq!(status, 401);
            assert!(body.contains("token rejected"));
        }
        other => panic!("expected ApiStatus for 401, got: {other:?}"),
    }
}

#[tokio::test]
async fn api_status_error_surfaces_5xx_body() {
    let server = spawn_mock_server(|_method, _path| {
        MockResponse::text(500, "Internal Server Error", "temporary upstream failure")
    })
    .await;

    let client = build_client(server.port()).await;
    let err = client.version().await.expect_err("expected server error");

    match err {
        PveError::ApiStatus { status, body } => {
            assert_eq!(status, 500);
            assert!(body.contains("temporary upstream failure"));
        }
        other => panic!("expected ApiStatus for 500, got: {other:?}"),
    }
}

#[tokio::test]
async fn wait_for_task_returns_timeout_when_status_never_stops() {
    let server = spawn_mock_server(|_method, _path| {
        MockResponse::json(200, "OK", r#"{"data":{"status":"running"}}"#)
    })
    .await;

    let client = build_client(server.port()).await;
    let err = client
        .wait_for_task_with_options(
            "node1",
            "UPIDTEST",
            &WaitTaskOptions {
                poll_interval: Duration::from_millis(5),
                timeout: Some(Duration::from_millis(30)),
            },
        )
        .await
        .expect_err("expected timeout");

    match err {
        PveError::TaskTimeout { upid, .. } => {
            assert_eq!(upid, "UPIDTEST");
        }
        other => panic!("expected TaskTimeout, got: {other:?}"),
    }
}
