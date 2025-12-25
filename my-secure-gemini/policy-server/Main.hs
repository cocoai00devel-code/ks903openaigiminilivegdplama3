{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE DeriveGeneric #-}

module Main where

import Web.Scotty
import Data.Aeson (FromJSON, ToJSON)
import GHC.Generics
import Network.HTTP.Types (status403)

-- 会話の状態を定義（未認証か、許可済みか）
data AuthStatus = Pending | Approved String deriving (Show, Generic)

-- 判定リクエストの構造体
data CheckRequest = CheckRequest { userId :: String, cmd :: String } deriving (Generic)
instance FromJSON CheckRequest

main :: IO ()
main = scotty 8000 $ do
    post "/check" $ do
        req <- unsafeJsonBody :: ActionM CheckRequest
        -- 🛡️ 数学的な厳格判定：特定のコマンド以外は「型」レベルで拒否に近い扱いにする
        if cmd req == "INIT_SECURE_LIVE"
            then json $ object ["status" .= ("OK" :: String), "token" .= ("HS-PROOF-99" :: String)]
            else do
                status status403
                json $ object ["error" .= ("POLICY_VIOLATION" :: String)]