
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as GMsg};

#[tokio::main]
async fn main() {
    // 🛡️ .envから環境変数を読み込む
    dotenvy::dotenv().ok();
    
    // 🏠 Rustサーバーは 5000番ポートで待機（Goから転送される先）
    let addr = "127.0.0.1:5000";
    let app = Router::new().route("/ws", get(ws_handler));

    println!("🛡️ Gemini Live Secure Proxy: {} で起動中...", addr);
    println!("🛡️ Rust Backend: 鉄壁の防衛体制で待機中 ({})", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    // handle_socket 等は既存の高性能なロジックを維持
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl axum::response::IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut browser_ws: WebSocket) {
    // 🛡️ APIキーを環境変数から取得
    let api_key = env::var("GEMINI_API_KEY").expect("APIキーが未設定です");
    
    let gemini_url = format!(
        "wss://generativelanguage.googleapis.com/ws/google.ai.generativelanguage.v1alpha.GenerativeService.BidiGenerateContent?key={}",
        api_key
    );

    println!("🔗 Gemini Live サーバーへ接続を試みています...");
    let (mut gemini_ws, _) = match connect_async(&gemini_url).await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("❌ Gemini接続失敗: {}", e);
            return;
        }
    };
    println!("✅ Gemini との接続が確立されました");

    loop {
        tokio::select! {
            // 📥 ブラウザ(React)からメッセージが届いた時
            Some(result) = browser_ws.next() => {
                match result {
                    Ok(msg) => {
                        match msg {
                            Message::Binary(bin) => {
                                // 💡 可視化：ブラウザから音声データが届いているか
                                // 頻繁に出すぎないよう、サイズだけ表示
                                println!("📥 [Browser -> Rust] Binary: {} bytes", bin.len());
                                let _ = gemini_ws.send(GMsg::Binary(bin)).await;
                            }
                            Message::Text(txt) => {
                                println!("💬 [Browser -> Rust] Text: {}", txt);
                                let _ = gemini_ws.send(GMsg::Text(txt)).await;
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        println!("❌ ブラウザとの通信エラー: {}", e);
                        break;
                    }
                }
            }
            // 🤖 Gemini から返答が届いた時
            Some(result) = gemini_ws.next() => {
                match result {
                    Ok(gemini_msg) => {
                        match gemini_msg {
                            GMsg::Text(txt) => {
                                // 💡 超重要：Geminiが「何かつぶやいている（エラー等）」のを可視化
                                println!("🤖 [Gemini -> Rust] Text: {}", txt);
                                let _ = browser_ws.send(Message::Text(txt)).await;
                            }
                            GMsg::Binary(bin) => {
                                // 💡 可視化：Geminiから音声が返ってきているか
                                println!("🔊 [Gemini -> Rust] Binary: {} bytes", bin.len());
                                let _ = browser_ws.send(Message::Binary(bin)).await;
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        println!("❌ Geminiとの通信エラー: {}", e);
                        break;
                    }
                }
            }
        }
    }
    println!("📴 接続が終了しました");
}

// use axum::{
//     extract::ws::{Message, WebSocket, WebSocketUpgrade},
//     routing::get,
//     Router,
// };
// use futures_util::{SinkExt, StreamExt};
// use std::env;
// use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as GMsg};

// #[tokio::main]
// async fn main() {
//     dotenvy::dotenv().ok();
//     // let port = "127.0.0.1:3000";
//     // 修正後
// let addr = "127.0.0.1:5000";

//     let app = Router::new().route("/ws", get(ws_handler));

//     println!("🛡️ Gemini Live Secure Proxy: {} で起動中...", port);
//     let listener = tokio::net::TcpListener::bind(port).await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

// async fn ws_handler(ws: WebSocketUpgrade) -> impl axum::response::IntoResponse {
//     ws.on_upgrade(handle_socket)
// }

// async fn handle_socket(mut browser_ws: WebSocket) {
//     // 🛡️ 金庫(.env)からキーを取り出す
//     let api_key = env::var("GEMINI_API_KEY").expect("APIキーが未設定です");
    
//     // Gemini Live API (WebSocket) のエンドポイント
//     // ※ v1alpha などの最新バージョンを使用
//     let gemini_url = format!(
//         "wss://generativelanguage.googleapis.com/ws/google.ai.generativelanguage.v1alpha.GenerativeService.BidiGenerateContent?key={}",
//         api_key
//     );

//     println!("🔗 Gemini Live サーバーへ接続を試みています...");
//     let (mut gemini_ws, _) = connect_async(&gemini_url).await.expect("Gemini接続失敗");
//     println!("✅ Gemini との接続が確立されました");

//     loop {
//         tokio::select! {
//             // 🎤 ブラウザ(React)から届いた音声データを Gemini へ
//             Some(Ok(msg)) = browser_ws.next() => {
//                 match msg {
//                     Message::Binary(bin) => { let _ = gemini_ws.send(GMsg::Binary(bin)).await; }
//                     Message::Text(txt) => { let _ = gemini_ws.send(GMsg::Text(txt)).await; }
//                     _ => {}
//                 }
//             }
//             // 🤖 Gemini から届いた返答(音声)を ブラウザ(React) へ
//             Some(Ok(msg)) = gemini_ws.next() => {
//                 match msg {
//                     GMsg::Binary(bin) => { let _ = browser_ws.send(Message::Binary(bin)).await; }
//                     GMsg::Text(txt) => { let _ = browser_ws.send(Message::Text(txt)).await; }
//                     _ => {}
//                 }
//             }
//         }
//     }
// }

// use axum::{
//     extract::ws::{Message, WebSocket, WebSocketUpgrade},
//     routing::get,
//     Router,
// };
// use futures_util::{SinkExt, StreamExt};
// use std::env;
// use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as GMsg};

// #[tokio::main]
// async fn main() {
//     dotenvy::dotenv().ok();
    
//     // 🛡️ ポートを5000番に固定（Goの3000番と衝突しないように）
//     let addr = "127.0.0.1:5000";

//     let app = Router::new().route("/ws", get(ws_handler));

//     println!("🛡️ Gemini Live Secure Proxy: {} で起動中...", addr);
    
//     // 変数名を addr に統一
//     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

// async fn ws_handler(ws: WebSocketUpgrade) -> impl axum::response::IntoResponse {
//     ws.on_upgrade(handle_socket)
// }

// async fn handle_socket(mut browser_ws: WebSocket) {
//     // 🛡️ 金庫(.env)からキーを取り出す
//     let api_key = env::var("GEMINI_API_KEY").expect("APIキーが未設定です");
    
//     let gemini_url = format!(
//         "wss://generativelanguage.googleapis.com/ws/google.ai.generativelanguage.v1alpha.GenerativeService.BidiGenerateContent?key={}",
//         api_key
//     );

//     println!("🔗 Gemini Live サーバーへ接続を試みています...");
//     let (mut gemini_ws, _) = connect_async(&gemini_url).await.expect("Gemini接続失敗");
//     println!("✅ Gemini との接続が確立されました");

//     loop {
//         tokio::select! {
//             // 🤖 Gemini から届いた返答(音声)を ブラウザ(React) へ
//             Some(Ok(msg)) = browser_ws.next() => {
//                 match msg {
//                     Message::Binary(bin) => { let _ = gemini_ws.send(GMsg::Binary(bin)).await; }
//                     Message::Text(txt) => { let _ = gemini_ws.send(GMsg::Text(txt)).await; }
//                     _ => {}
//                 }
//             }
//             Some(Ok(msg)) = gemini_ws.next() => {
//                 match msg {
//                     GMsg::Binary(bin) => { let _ = browser_ws.send(Message::Binary(bin)).await; }
//                     GMsg::Text(txt) => { let _ = browser_ws.send(Message::Text(txt)).await; }
//                     _ => {}
//                 }
//             }
//         }
//     }
// }

// main.rs の handle_socket ループ内を修正
// loop {
//     tokio::select! {
//         // 📥 ブラウザ(React)から届いたメッセージ
//         Some(Ok(msg)) = browser_ws.next() => {
//             match msg {
//                 Message::Binary(bin) => {
//                     // 💡 ログ追加：届いたデータのサイズを表示
//                     println!("📥 受信(Browser): {} bytes", bin.len());
//                     let _ = gemini_ws.send(GMsg::Binary(bin)).await;
//                 }
//                 Message::Text(txt) => {
//                     println!("💬 設定送信: {}", txt);
//                     let _ = gemini_ws.send(GMsg::Text(txt)).await;
//                 }
//                 _ => {}
//             }
//         }
//         // 🤖 Gemini から届いたメッセージ
//         Some(Ok(gemini_msg)) = gemini_ws.next() => {
//             match gemini_msg {
//                 GMsg::Text(txt) => {
//                     // 💡 超重要：Geminiがエラーをテキストで返している場合に気づけます
//                     println!("🤖 Geminiからの通知: {}", txt);
//                     let _ = browser_ws.send(Message::Text(txt)).await;
//                 }
//                 GMsg::Binary(bin) => {
//                     // 💡 ログ追加：返ってきた音声のサイズを表示
//                     println!("🔊 返答(Gemini): {} bytes", bin.len());
//                     let _ = browser_ws.send(Message::Binary(bin)).await;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }