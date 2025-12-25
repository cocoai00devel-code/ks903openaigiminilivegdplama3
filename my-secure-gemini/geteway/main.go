// package main

// import (
//     "log"
//     "net/http"
//     "net/http/httputil"
//     "net/url"
// )

// func main() {
//     // Rustサーバーの住所
//     target, _ := url.Parse("http://backend:8080")
//     proxy := httputil.NewSingleHostReverseProxy(target)

//     http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
//         log.Println("🛡️ Go Gateway: 通信を検閲中...")
//         // ここで認証やアクセス制限を行う（Goの得意分野！）
//         proxy.ServeHTTP(w, r)
//     })

//     log.Println("🚀 Go Gateway: 3000番ポートで検問開始...")
//     log.Fatal(http.ListenAndServe(":3000", nil))
// }
package main

import (
	"bytes"
	"encoding/json"
	"log"
	"net/http"
	"net/http/httputil"
	"net/url"
)

// Haskell（審判）への判定依頼
// Haskellに送るリクエストの構造
type PolicyCheckRequest struct {
	UserID  string `json:"userId"`
	Command string `json:"cmd"`
}

// Haskell（審判）からの回答
// Haskellから返ってくるレスポンスの構造
type PolicyResponse struct {
	Status string `json:"status"`
	Token  string `json:"token"`
}

func main() {
	// 🏠 送り先（Rust）と ⚖️ 審判（Haskell）の住所を設定 // 🛡️ 送り先（Rust金庫）の住所
	// 🏠 送り先（Rust backend）と ⚖️ 審判所（Haskell policy-engine）の住所
	// Docker Composeのサービス名に合わせて修正
	rustURL, _ := url.Parse("http://rust-backend:5000")
	haskellURL := "http://policy-engine:8000/check"
    // 🔄 プロキシ（右から左へ受け流す）の設定
	proxy := httputil.NewSingleHostReverseProxy(rustURL)

	// すべてのアクセスをここで受け止める
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		log.Printf("📥 検問所(Go)通過中: %s %s", r.Method, r.URL.Path)
		log.Printf("📥 検問所通過: %s %s", r.Method, r.URL.Path)
        log.Println("⚖️ Go Gateway: Haskell審判所にアクセス許可を確認中...")
		// 🛡️ ステップ1: Haskell審判所に許可を求める
		checkReq := PolicyCheckRequest{
			UserID:  "user-123",
			Command: "INIT_SECURE_LIVE", 
		}
		jsonData, _ := json.Marshal(checkReq)
        // 2. Haskell (Policy Engine) に判定を仰ぐ
		resp, err := http.Post(haskellURL, "application/json", bytes.NewBuffer(jsonData))
		
		// HaskellがNOと言った、あるいはHaskellが落ちている場合は即座に遮断
		if err != nil || resp.StatusCode != http.StatusOK {
			log.Printf("🚫 拒否: Haskell審判所が許可しませんでした")
			log.Printf("🚫 拒否: HaskellがNOと言っています (Status: %v)", resp.StatusCode)
			http.Error(w, "Policy Violation: Access Denied by Haskell", http.StatusForbidden)
			http.Error(w, "Access Denied by Haskell", http.StatusForbidden)
			return
		}

		// 🛡️ ステップ2: 許可証（Token）を読み取る
		// 3. Haskellからの許可証（トークン）を読み取る
		var pResp PolicyResponse
		json.NewDecoder(resp.Body).Decode(&pResp)
		log.Printf("✅ 許可されました。Token: %s", pResp.Token)

		// 🛡️ ステップ3: 許可されたので、Rustへデータを渡す準備をして実行！
		r.Header.Set("X-Haskell-Token", pResp.Token)

        // 4. 許可されたので、Rustバックエンドへ中継
		// ここでヘッダーを検証したり、ログを取ったりできる（セキュリティ層）
		r.Host = rustURL.Host
		log.Printf("✅ 許可: Rustへリレーします (Token: %s)", pResp.Token)
		proxy.ServeHTTP(w, r)
	})
    
	log.Println("🚀 5段階要塞・第2層(Go Gateway): 3000番ポートで検問中...")
	log.Println("🚀 Go Gateway: 3000番ポートで検問中（Rustへ転送します）...")
	log.Println("🚀 5段階要塞・玄関口(Go): 3000番ポートで監視中...")
	log.Println("🚀 5段階要塞・玄関(Go): 3000番ポートで検問中...")
	log.Fatal(http.ListenAndServe(":3000", nil))
}

// package main

// import (
// 	"log"
// 	"net/http"
// 	"net/http/httputil"
// 	"net/url"
// )

// func main() {
// 	// 🛡️ 送り先（Rust金庫）の住所
// 	remote, err := url.Parse("http://127.0.0.1:5000")
// 	if err != nil {
// 		panic(err)
// 	}

// 	// 🔄 プロキシ（右から左へ受け流す）の設定
// 	proxy := httputil.NewSingleHostReverseProxy(remote)

// 	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
// 		log.Printf("📥 検問所通過: %s %s", r.Method, r.URL.Path)
		
// 		// ここでヘッダーを検証したり、ログを取ったりできる（セキュリティ層）
// 		r.Host = remote.Host
// 		proxy.ServeHTTP(w, r)
// 	})

// 	log.Println("🚀 Go Gateway: 3000番ポートで検問中（Rustへ転送します）...")
// 	err = http.ListenAndServe(":3000", nil)
// 	if err != nil {
// 		log.Fatal(err)
// 	}
// }
// package main

// import (
// 	"bytes"
// 	"encoding/json"
// 	"fmt"
// 	"io"
// 	"log"
// 	"net/http"
// 	"net/http/httputil"
// 	"net/url"
// )

// // Haskellに送るリクエストの構造
// type PolicyCheckRequest struct {
// 	UserID  string `json:"userId"`
// 	Command string `json:"cmd"`
// }

// // Haskellから返ってくるレスポンスの構造
// type PolicyResponse struct {
// 	Status string `json:"status"`
// 	Token  string `json:"token"`
// }

// func main() {
// 	// 🏠 各コンテナの住所（Docker-composeでのサービス名を使用）
// 	rustURL, _ := url.Parse("http://rust-backend:5000")
// 	haskellURL := "http://policy-engine:8000/check"

// 	proxy := httputil.NewSingleHostReverseProxy(rustURL)

// 	http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
// 		log.Println("⚖️ Go Gateway: Haskell審判所にアクセス許可を確認中...")

// 		// 1. Haskellへの問い合わせデータ作成
// 		checkReq := PolicyCheckRequest{
// 			UserID:  "user-123",        // 本来はCookieやヘッダーから取得
// 			Command: "INIT_SECURE_LIVE", 
// 		}
// 		jsonData, _ := json.Marshal(checkReq)

// 		// 2. Haskell (Policy Engine) に判定を仰ぐ
// 		resp, err := http.Post(haskellURL, "application/json", bytes.NewBuffer(jsonData))
// 		if err != nil || resp.StatusCode != http.StatusOK {
// 			log.Printf("🚫 拒否: HaskellがNOと言っています (Status: %v)", resp.StatusCode)
// 			http.Error(w, "Policy Violation: Access Denied by Haskell", http.StatusForbidden)
// 			return
// 		}

// 		// 3. Haskellからの許可証（トークン）を読み取る
// 		var pResp PolicyResponse
// 		json.NewDecoder(resp.Body).Decode(&pResp)
// 		log.Printf("✅ 許可されました。Token: %s", pResp.Token)

// 		// 4. 許可されたので、Rustバックエンドへ中継
// 		r.Header.Set("X-Haskell-Token", pResp.Token) // Rust側に許可証を渡す
// 		r.Host = rustURL.Host
// 		proxy.ServeHTTP(w, r)
// 	})

// 	log.Println("🚀 5段階要塞・玄関口(Go): 3000番ポートで監視中...")
// 	log.Fatal(http.ListenAndServe(":3000", nil))
// }