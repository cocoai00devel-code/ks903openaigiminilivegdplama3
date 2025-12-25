import React, { useEffect, useRef, useState } from 'react';

export default function App() {
  // --- 1. 状態管理（ステータス） ---
  const [status, setStatus] = useState<string>('待機中');
  
  // --- 2. 参照（WebSocketやオーディオ Context） ---
  const ws = useRef<WebSocket | null>(null);
  const audioCtx = useRef<AudioContext | null>(null);
  const nextStartTime = useRef<number>(0);

  // --- 3. 接続設定 (useEffect) ---
  useEffect(() => {
    // Rustプロキシに接続
    const socket = new WebSocket('ws://localhost:3000/ws');
    socket.binaryType = 'arraybuffer';
    ws.current = socket;

    socket.onopen = () => {
  setStatus('安全な接続を確立しました');
  // Gemini Live を起動するための初期設定を送信
  const setup = {
    setup: { 
      model: "models/gemini-2.0-flash-exp" // 👈 Live対応モデルを指定
    }
  };
  socket.send(JSON.stringify(setup));
};
    socket.onmessage = (event: MessageEvent) => {
      if (event.data instanceof ArrayBuffer) {
        handleIncomingAudio(event.data);
      }
    };

    return () => {
      socket.close();
      if (audioCtx.current && audioCtx.current.state !== 'closed') {
        audioCtx.current.close();
      }
    };
  }, []);

  // --- 4. 音声再生ロジック ---
  const handleIncomingAudio = async (data: ArrayBuffer) => {
    if (!audioCtx.current) {
      audioCtx.current = new (window.AudioContext || (window as any).webkitAudioContext)({ sampleRate: 24000 });
    }
    
    const int16Data = new Int16Array(data);
    const float32Data = new Float32Array(int16Data.length);
    for (let i = 0; i < int16Data.length; i++) {
      float32Data[i] = int16Data[i] / 32767;
    }

    const buffer = audioCtx.current.createBuffer(1, float32Data.length, 24000);
    buffer.getChannelData(0).set(float32Data);
    const source = audioCtx.current.createBufferSource();
    source.buffer = buffer;
    source.connect(audioCtx.current.destination);

    const startTime = Math.max(audioCtx.current.currentTime, nextStartTime.current);
    source.start(startTime);
    nextStartTime.current = startTime + buffer.duration;
  };

  // --- 5. 録音・送信ロジック ---
  const startStreaming = async () => {
    try {
      setStatus('録音中...');
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      
      if (!audioCtx.current) {
        audioCtx.current = new (window.AudioContext || (window as any).webkitAudioContext)({ sampleRate: 16000 });
      }

      if (audioCtx.current.state === 'suspended') {
        await audioCtx.current.resume();
      }
      
      const source = audioCtx.current.createMediaStreamSource(stream);
      // 👇 ここ！この2行をセットで記述します
    // @ts-ignore
    // const processor = audioCtx.current.createScriptProcessor(4096, 1, 1);
      const processor = audioCtx.current.createScriptProcessor(4096, 1, 1);

      // processor.onaudioprocess = (e) => {
      //   const input = e.inputBuffer.getChannelData(0);
      //   const pcm16 = new Int16Array(input.length);
      //   for (let i = 0; i < input.length; i++) {
      //     pcm16[i] = Math.max(-1, Math.min(1, input[i])) * 32767;
      //   }
      //   if (ws.current?.readyState === WebSocket.OPEN) {
      //     ws.current.send(pcm16.buffer);
      //   }
      // };
      // App.tsx の onaudioprocess 内を修正
      processor.onaudioprocess = (e) => {
        const input = e.inputBuffer.getChannelData(0);
        const pcm16 = new Int16Array(input.length);
        for (let i = 0; i < input.length; i++) {
          pcm16[i] = Math.max(-1, Math.min(1, input[i])) * 32767;
        }
        if (ws.current?.readyState === WebSocket.OPEN) {
          // 💡 ログ追加：送信サイズと、最初の数サンプルを表示
          // これが 0 ばかりならマイクが音を拾っていません
          if (Math.random() < 0.1) { // 負荷軽減のため10回に1回表示
              console.log("🎤 送信中: ", pcm16.length, "bytes", pcm16[0], pcm16[1]);
          }
          ws.current.send(pcm16.buffer);
        }
      };

      source.connect(processor);
      processor.connect(audioCtx.current.destination);
    } catch (err) {
      console.error("エラー:", err);
      setStatus('エラーが発生しました');
    }
  };

  // --- 6. 画面表示 (HTML/JSX) ---
  return (
    <div style={{ padding: '40px', fontFamily: 'sans-serif', textAlign: 'center' }}>
      <h1>🛡️ Secure Gemini Live</h1>
      <div style={{ margin: '20px', padding: '20px', border: '1px solid #ccc', borderRadius: '10px' }}>
        <p>ステータス: <strong>{status}</strong></p>
        <button 
          onClick={startStreaming} 
          style={{ 
            padding: '15px 30px', 
            fontSize: '18px', 
            backgroundColor: '#007bff', 
            color: 'white', 
            border: 'none', 
            borderRadius: '5px',
            cursor: 'pointer'
          }}>
          対話を開始する
        </button>
      </div>
    </div>
  );
}
