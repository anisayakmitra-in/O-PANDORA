import { useState, useEffect, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { Send, Terminal, Settings, Activity, Box, Code2, Shield, GitBranch, X, Maximize2, Minimize2 } from 'lucide-react'

interface Message {
  id: string
  role: 'user' | 'system' | 'tool' | 'error'
  content: string
  duration?: number
}

type Tab = 'chat' | 'harnesses' | 'genes' | 'settings'

export default function App() {
  const [messages, setMessages] = useState<Message[]>([{
    id: 'welcome',
    role: 'system',
    content: 'Pandora is ready. Type a task or use / for commands.'
  }])
  const [input, setInput] = useState('')
  const [running, setRunning] = useState(false)
  const [tab, setTab] = useState<Tab>('chat')
  const [harnesses, setHarnesses] = useState<string[]>([])
  const [genes, setGenes] = useState<string[]>([])
  const [sessionId, setSessionId] = useState('')
  const chatRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    invoke<string>('get_session').then(setSessionId)
    invoke<string[]>('get_harnesses').then(setHarnesses)
    invoke<string[]>('get_genes').then(setGenes)
  }, [])

  useEffect(() => {
    chatRef.current?.scrollTo({ top: chatRef.current.scrollHeight, behavior: 'smooth' })
  }, [messages])

  async function send() {
    const task = input.trim()
    if (!task || running) return
    setInput('')

    const userMsg: Message = { id: crypto.randomUUID(), role: 'user', content: task }
    const loadingMsg: Message = { id: 'loading', role: 'system', content: '• Executing...' }
    setMessages(prev => [...prev, userMsg, loadingMsg])
    setRunning(true)

    try {
      const result: any = await invoke('run_task', { task, domain: 'general' })
      setMessages(prev => prev.filter(m => m.id !== 'loading'))
      if (result.success) {
        const output = result.output || '(no output)'
        setMessages(prev => [...prev, {
          id: crypto.randomUUID(),
          role: 'system',
          content: output,
          duration: result.duration_ms
        }])
      } else {
        setMessages(prev => [...prev, {
          id: crypto.randomUUID(),
          role: 'error',
          content: result.error || 'Unknown error'
        }])
      }
    } catch (e: any) {
      setMessages(prev => prev.filter(m => m.id !== 'loading'))
      setMessages(prev => [...prev, {
        id: crypto.randomUUID(),
        role: 'error',
        content: `Connection error: ${e}`
      }])
    }
    setRunning(false)
  }

  function handleKeyDown(e: React.KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      send()
    }
  }

  return (
    <div style={{
      display: 'flex',
      height: '100vh',
      background: 'linear-gradient(135deg, #0a0a14 0%, #0d0d20 40%, #0f0a1e 100%)',
      fontFamily: "'Inter', system-ui, sans-serif",
    }}>
      {/* Sidebar */}
      <div className="glass-panel" style={{
        width: 240,
        display: 'flex',
        flexDirection: 'column',
        padding: '16px 12px',
        margin: 8,
        borderRadius: 16,
      }}>
        <div style={{ fontSize: 18, fontWeight: 700, color: '#a78bfa', marginBottom: 24, padding: '0 8px' }}>
          ⚡ Pandora
        </div>

        <nav style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: 2 }}>
          {([
            { id: 'chat', icon: Terminal, label: 'Chat' },
            { id: 'harnesses', icon: Box, label: 'Harnesses' },
            { id: 'genes', icon: Code2, label: 'Genes' },
            { id: 'settings', icon: Settings, label: 'Settings' },
          ] as const).map(item => (
            <button
              key={item.id}
              onClick={() => setTab(item.id)}
              className="glass-hover"
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 10,
                padding: '8px 12px',
                borderRadius: 8,
                border: 'none',
                background: tab === item.id ? 'rgba(124, 58, 237, 0.15)' : 'transparent',
                color: tab === item.id ? '#c4b5fd' : 'rgba(255,255,255,0.5)',
                cursor: 'pointer',
                fontSize: 13,
                fontWeight: 500,
                textAlign: 'left',
                width: '100%',
              }}
            >
              <item.icon size={16} />
              {item.label}
            </button>
          ))}
        </nav>

        <div style={{
          padding: '8px 12px',
          fontSize: 11,
          color: 'rgba(255,255,255,0.25)',
          borderTop: '1px solid rgba(255,255,255,0.05)',
          marginTop: 8,
        }}>
          <div>v0.5.0</div>
          <div style={{ marginTop: 2 }}>{sessionId?.slice(0, 20)}...</div>
        </div>
      </div>

      {/* Main */}
      <div style={{ flex: 1, display: 'flex', flexDirection: 'column', margin: '8px 8px 8px 0' }}>
        {/* Title bar */}
        <div className="glass-panel" style={{
          display: 'flex',
          alignItems: 'center',
          padding: '8px 16px',
          borderRadius: '16px 16px 0 0',
          gap: 12,
          fontSize: 12,
          color: 'rgba(255,255,255,0.4)',
        }}>
          <div style={{ display: 'flex', gap: 6, marginRight: 8 }}>
            <div style={{ width: 10, height: 10, borderRadius: '50%', background: '#22c55e' }} title="Provider connected" />
            <div style={{ width: 10, height: 10, borderRadius: '50%', background: '#eab308' }} />
            <div style={{ width: 10, height: 10, borderRadius: '50%', background: '#ef4444' }} />
          </div>
          <span style={{ flex: 1 }}>{harnesses.length} harnesses · {genes.length} genes</span>
          <Shield size={12} style={{ color: '#22c55e' }} />
          <span>governed</span>
          <GitBranch size={12} />
          <span>main</span>
        </div>

        {/* Content area */}
        <div style={{
          flex: 1,
          display: 'flex',
          flexDirection: 'column',
          background: 'rgba(12, 12, 24, 0.5)',
          backdropFilter: 'blur(12px)',
        }}>
          {tab === 'chat' ? (
            <>
              {/* Messages */}
              <div ref={chatRef} style={{
                flex: 1,
                overflowY: 'auto',
                padding: '20px 24px',
                display: 'flex',
                flexDirection: 'column',
                gap: 12,
              }}>
                {messages.map(msg => (
                  <div key={msg.id} style={{
                    maxWidth: msg.role === 'user' ? '75%' : '90%',
                    alignSelf: msg.role === 'user' ? 'flex-end' : 'flex-start',
                    padding: '10px 16px',
                    borderRadius: 12,
                    fontSize: 14,
                    lineHeight: 1.6,
                    background: msg.role === 'user'
                      ? 'rgba(124, 58, 237, 0.2)'
                      : msg.role === 'error'
                        ? 'rgba(239, 68, 68, 0.15)'
                        : 'rgba(255, 255, 255, 0.04)',
                    border: '1px solid',
                    borderColor: msg.role === 'user'
                      ? 'rgba(124, 58, 237, 0.3)'
                      : msg.role === 'error'
                        ? 'rgba(239, 68, 68, 0.3)'
                        : 'rgba(255, 255, 255, 0.06)',
                    whiteSpace: 'pre-wrap',
                    wordBreak: 'break-word',
                  }}>
                    <div style={{ color: msg.role === 'error' ? '#fca5a5' : 'rgba(255,255,255,0.9)' }}>
                      {msg.content}
                    </div>
                    {msg.duration !== undefined && (
                      <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)', marginTop: 6 }}>
                        {msg.duration}ms
                      </div>
                    )}
                  </div>
                ))}
              </div>

              {/* Input */}
              <div style={{
                padding: '12px 20px',
                borderTop: '1px solid rgba(255,255,255,0.05)',
              }}>
                <div style={{
                  display: 'flex',
                  gap: 8,
                }}>
                  <input
                    value={input}
                    onChange={e => setInput(e.target.value)}
                    onKeyDown={handleKeyDown}
                    disabled={running}
                    placeholder={
                      running ? 'Executing...' : 'Type a task or /command...'
                    }
                    style={{
                      flex: 1,
                      padding: '10px 16px',
                      background: 'rgba(255, 255, 255, 0.04)',
                      border: '1px solid rgba(255, 255, 255, 0.08)',
                      borderRadius: 10,
                      color: 'rgba(255,255,255,0.9)',
                      fontSize: 14,
                      outline: 'none',
                      fontFamily: "'Inter', system-ui, sans-serif",
                    }}
                  />
                  <button
                    onClick={send}
                    disabled={running || !input.trim()}
                    style={{
                      padding: '10px 20px',
                      background: running ? 'rgba(124, 58, 237, 0.3)' : 'rgba(124, 58, 237, 0.8)',
                      border: 'none',
                      borderRadius: 10,
                      color: '#fff',
                      cursor: running ? 'default' : 'pointer',
                      fontSize: 14,
                      fontWeight: 600,
                      display: 'flex',
                      alignItems: 'center',
                      gap: 6,
                    }}
                  >
                    <Send size={16} />
                    Send
                  </button>
                </div>
              </div>
            </>
          ) : tab === 'harnesses' ? (
            <div style={{ padding: 24, overflowY: 'auto' }}>
              <h2 style={{ fontSize: 16, fontWeight: 600, marginBottom: 12, color: 'rgba(255,255,255,0.5)' }}>
                Installed Harnesses
              </h2>
              <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
                {harnesses.map(h => (
                  <div key={h} className="glass" style={{
                    padding: '12px 16px',
                    fontSize: 14,
                  }}>
                    <div style={{ fontWeight: 600 }}>{h}</div>
                  </div>
                ))}
                {harnesses.length === 0 && (
                  <div style={{ color: 'rgba(255,255,255,0.3)', fontSize: 13 }}>
                    No harnesses installed.
                  </div>
                )}
              </div>
            </div>
          ) : tab === 'genes' ? (
            <div style={{ padding: 24, overflowY: 'auto' }}>
              <h2 style={{ fontSize: 16, fontWeight: 600, marginBottom: 12, color: 'rgba(255,255,255,0.5)' }}>
                Installed Genes
              </h2>
              <div style={{ display: 'flex', flexWrap: 'wrap', gap: 8 }}>
                {genes.map(g => (
                  <span key={g} className="glass" style={{
                    padding: '6px 12px',
                    fontSize: 13,
                    borderRadius: 20,
                  }}>
                    {g}
                  </span>
                ))}
                {genes.length === 0 && (
                  <div style={{ color: 'rgba(255,255,255,0.3)', fontSize: 13 }}>
                    No genes loaded.
                  </div>
                )}
              </div>
            </div>
          ) : (
            <div style={{ padding: 24, color: 'rgba(255,255,255,0.5)', fontSize: 13 }}>
              <h2 style={{ fontSize: 16, fontWeight: 600, marginBottom: 12, color: 'rgba(255,255,255,0.5)' }}>
                Settings
              </h2>
              <p>Configuration is managed through environment variables and config files.</p>
              <p style={{ marginTop: 8 }}>Run <code>pandora doctor</code> for full diagnostics.</p>
              <div style={{ marginTop: 16, padding: 12, background: 'rgba(0,0,0,0.3)', borderRadius: 8 }}>
                <div style={{ color: 'rgba(255,255,255,0.4)', fontSize: 11 }}>Session ID</div>
                <code style={{ fontSize: 12 }}>{sessionId}</code>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
