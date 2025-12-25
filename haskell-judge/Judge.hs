{-# LANGUAGE OverloadedStrings #-}
import Network.Wai
import Network.HTTP.Types
import Network.Wai.Handler.Warp (run)

-- 現在動かしたいメインのプログラム
main :: IO ()
main = do
    putStrLn "---------------------------------------"
    putStrLn " 鉄壁のHaskell審判所 (Port 8000) 起動！"
    putStrLn "---------------------------------------"
    run 8000 app

app :: Application
app req respond = do
    let query = queryString req
    -- どちらかの合言葉なら許可するルール
    if ("pass", Just "open-sesame") `elem` query || ("pass", Just "abracadabra") `elem` query
        then do
            putStrLn ">>> 【判定：許可】魔法の言葉を確認！"
            respond $ responseLBS status200 [("Content-Type", "text/plain")] "ALLOWED"
   
   -----------------------------------------------------------
-- ここから下はすべて「メモ」です（プログラムとしては無視されます）
-----------------------------------------------------------

{- 
    ; if ("pass", Just "open-sesame") `elem` query
    ; ;    then do
      ; ;      putStrLn ">>> 【判定：許可】合言葉を確認しました。"
        ; ;    respond $ responseLBS status200 [("Content-Type", "text/plain")] "ALLOWED"
       ; ; else do
      ; ;      putStrLn ">>> 【判定：拒否】合言葉が違う、または存在しません。"
       ; ;     respond $ responseLBS status200 [("Content-Type", "text/plain")] "DENIED"
-}
-----------------------------------------------------------
-- ここから下はすべて「メモ」です（プログラムとしては無視されます）
-----------------------------------------------------------

{- 
-- メモ１：以前のコード
; ; {-# LANGUAGE OverloadedStrings #-}
; ; import Network.Wai
; ; import Network.HTTP.Types
; ; import Network.Wai.Handler.Warp (run)

; ; main :: IO ()
; ; main = do
; ;     putStrLn "---------------------------------------"
; ;     putStrLn " Haskell 審判サーバー (Port 8000) 起動！"
; ;     putStrLn "---------------------------------------"
; ;     run 8000 app

; ; app :: Application
; ; app _ respond = do
; ;     putStrLn ">>> リクエストを受信しました。判定：ALLOWED"
; ;     respond $ responseLBS status200 [("Content-Type", "text/plain")] "ALLOWED"

-- メモ２：セミコロン混じりのコード
; {-# LANGUAGE OverloadedStrings #-}
; import Network.Wai
; import Network.HTTP.Types
; import Network.Wai.Handler.Warp (run)

; main :: IO ()
; main = do
;     putStrLn "---------------------------------------"
;     putStrLn " 鉄壁のHaskell審判所 (Port 8000) 起動！"
;     putStrLn "---------------------------------------"
;     run 8000 app

; app :: Application
; app req respond = do
;     let query = queryString req
;     if ("pass", Just "open-sesame") `elem` query
;         then do
;             putStrLn ">>> 【判定：許可】合言葉を確認しました。"
;             respond $ responseLBS status200 [("Content-Type", "text/plain")] "ALLOWED"
;         else do
;             putStrLn ">>> 【判定：拒否】合言葉が違う、または存在しません。"
;             respond $ responseLBS status200 [("Content-Type", "text/plain")] "DENIED"
-}