import { useEffect, useState } from "react";
import { api } from "./api";
import type { AiResult } from "./types";
import { todayIso } from "./health";

export default function CommandBar({ onChanged }: { onChanged: () => void }) {
  const [input, setInput] = useState("");
  const [result, setResult] = useState<AiResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const [keyInput, setKeyInput] = useState("");
  const [hasKey, setHasKey] = useState(false);

  useEffect(() => {
    api.hasAiKey().then((s: any) => {
      console.log("KEYRING:", s);
      setHasKey(String(s).startsWith("found"));
    });
  }, []);

  async function saveKey() {
    if (!keyInput.trim()) return;
    try {
      await api.setAiKey(keyInput.trim());
      setKeyInput("");
      setHasKey(true);
      setError(null);
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  async function run(text?: string) {
    const instruction = (text ?? input).trim();
    if (!instruction) return;

    setBusy(true);
    setError(null);
    try {
      const r = await api.runAiCommand(instruction, todayIso());
      setResult(r);
      setInput("");
      onChanged();
    } catch (e: any) {
      setError(e?.message ?? String(e));
      setResult(null);
    } finally {
      setBusy(false);
    }
  }

  return (
    <div style={{ marginBottom: 24 }}>
      {!hasKey && (
        <div style={{ display: "flex", gap: 8, marginBottom: 8 }}>
          <input
            type="password"
            value={keyInput}
            onChange={(e) => setKeyInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") saveKey();
            }}
            placeholder="NVIDIA API key — stored in Windows Credential Manager"
            style={{
              flex: 1,
              padding: 8,
              fontSize: 13,
              borderRadius: 6,
              border: "1px solid #444",
              background: "#1a1a1a",
              color: "inherit",
            }}
          />
          <button onClick={saveKey} style={{ padding: "8px 16px" }}>
            Save key
          </button>
        </div>
      )}

      <div style={{ display: "flex", gap: 8 }}>
        <input
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") run();
          }}
          placeholder="Tell Stark what you did or what you need…"
          disabled={busy}
          style={{
            flex: 1,
            padding: "10px 12px",
            fontSize: 14,
            borderRadius: 6,
            border: "1px solid #444",
            background: "#1a1a1a",
            color: "inherit",
          }}
        />
        <button
          onClick={() => run()}
          disabled={busy}
          style={{ padding: "10px 20px" }}
        >
          {busy ? "…" : "Run"}
        </button>
      </div>

      {error && (
        <p style={{ color: "#e88", fontSize: 13, marginTop: 8 }}>{error}</p>
      )}

      {result && (
        <div
          style={{
            marginTop: 8,
            padding: "8px 12px",
            borderRadius: 6,
            background: "#1e2a1e",
            border: "1px solid #2d6a4f",
            fontSize: 13,
          }}
        >
          {result.outcome.kind === "NEEDS_CLARIFICATION" ? (
            <>
              <div style={{ marginBottom: 6 }}>{result.outcome.question}</div>
              <div style={{ display: "flex", gap: 6, flexWrap: "wrap" }}>
                {result.outcome.candidates.slice(0, 6).map((c) => (
                  <button
                    key={c}
                    onClick={() => setInput(c)}
                    style={{ fontSize: 12, padding: "2px 8px" }}
                  >
                    {c}
                  </button>
                ))}
              </div>
            </>
          ) : (
            <div>{result.outcome.summary}</div>
          )}

          <div style={{ fontSize: 11, opacity: 0.5, marginTop: 6 }}>
            {result.action_name} · {result.tier}
            {result.tier === "cloud" ? ` · ${result.latency_ms} ms` : " · instant"}
          </div>
        </div>
      )}
    </div>
  );
}