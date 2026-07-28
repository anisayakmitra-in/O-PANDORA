import { useState, useEffect, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { Send, Plus, History, Cpu, Layers, Box, Terminal, XCircle } from 'lucide-react'

interface SessionMeta {
  id: string
  name: string
  created: string
  model: string
  provider: string
}

interface ModelInfo {
  name: string
  provider: string
  endpoint: string
  healthy: boolean
  context_size: number
}

interface Message {
  id: string
  role: 'user' | 'system' | 'tool' | 'error'
  content: string
  details?: string
  duration?: number
  expanded?: boolean
}

type Tab = 'chat' | 'sessions' | 'models'

export default function App() {
  const [messages, setMessages] = useState<Message[]>([])
  const [input, setInput] = useState('')
  const [running, setRunning] = useState(false)
  const [tab, setTab] = useState<Tab>('chat')
  const [sessions, setSessions] = useState<SessionMeta[]>([])
  const [activeSession, setActiveSession] = useState<SessionMeta | null>(null)
  const [models, setModels] = useState<ModelInfo[]>([])
  const [selectedModel, setSelectedModel] = useState('')
  const [streamOutput, setStreamOutput] = useState('')
  const chatRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    loadSessions()
    loadModels()
    const unlisten = setupStreamListener()
    return () => { unlisten.then(fn => fn?.()) }
  }, [])

  useEffect(() => {
    chatRef.current?.scrollTo({ top: chatRef.current.scrollHeight, behavior: 'smooth' })
  }, [messages, streamOutput])

  async function setupStreamListener(): Promise<UnlistenFn | undefined> {
    try {
      return await listen<{type: string; content: string; metadata?: any}>('stream-event', (event) => {
        const { type, content, metadata } = event.payload
        switch (type) {
          case 'execution.started':
            setStreamOutput(`• ${content}`)
            break
          case 'gene.executed':
            addToolMessage(metadata?.gene || 'tool', content, metadata?.duration_ms)
            break
          case 'execution.completed':
            setStreamOutput('')
            addSystemMessage(content, metadata?.duration_ms)
            setRunning(false)
            break
          case 'execution.failed':
            setStreamOutput('')
            addErrorMessage(content)
            setRunning(false)
            break
        }
      })
    } catch { return undefined }
  }

  function addToolMessage(gene: string, content: string, duration?: number) {
    setMessages(prev => [...prev, {
      id: crypto.randomUUID(),
      role: 'tool',
      content: gene,
      details: content,
      duration,
      expanded: false,
    }])
  }

  function addSystemMessage(content: string, duration?: number) {
    if (!content.trim()) return
    setMessages(prev => [...prev, {
      id: crypto.randomUUID(),
      role: 'system',
      content,
      duration,
    }])
  }

  function addErrorMessage(content: string) {
    setMessages(prev => [...prev, {
      id: crypto.randomUUID(),
      role: 'error',
      content,
    }])
  }

  async function loadSessions() {
    try {
      const s = await invoke<SessionMeta[]>('list_sessions')
      setSessions(s)
    } catch {}
  }

  async function loadModels() {
    try {
      const m = await invoke<ModelInfo[]>('list_models')
      setModels(m)
      if (m.length > 0) setSelectedModel(m[0].name)
    } catch {}
  }

  async function createNewSession() {
    const name = `Session ${sessions.length + 1}`
    try {
      const s = await invoke<SessionMeta>('create_session', { name })
      setActiveSession(s)
      setMessages([])
      setSessions(prev => [s, ...prev])
    } catch (e: any) {
      console.error('Create session failed:', e)
    }
  }

  async function resumeSession(sessionId: string) {
    try {
      const s = await invoke<SessionMeta>('resume_session', { sessionId })
      setActiveSession(s)
      setMessages([{
        id: crypto.randomUUID(),
        role: 'system',
        content: `Resumed session: ${s.name}`,
      }])
      setTab('chat')
    } catch (e: any) {
      console.error('Resume failed:', e)
    }
  }

  async function send() {
    const text = input.trim()
    if (!text || running) return
    setInput('')

    if (!activeSession) await createNewSession()
    if (!activeSession) return

    setMessages(prev => [...prev, {
      id: crypto.randomUUID(),
      role: 'user',
      content: text,
    }])
    setRunning(true)

    try {
      await invoke('send_message', { message: text })
    } catch (e: any) {
      addErrorMessage(e.toString())
      setRunning(false)
    }
  }

  function toggleTool(msg: Message) {
    setMessages(prev => prev.map(m =>
      m.id === msg.id ? { ...m, expanded: !m.expanded } : m
    ))
  }

  async function handleModelSwitch(model: string, provider: string) {
    setSelectedModel(model)
    try {
      await invoke('switch_model', { provider, model })
    } catch {}
  }

  return (
    <div style={{
      display: 'flex', height: '100vh',
      background: 'linear-gradient(135deg, #0a0a14 0%, #0d0d20 40%, #0f0a1e 100%)',
      fontFamily: "'Inter', system-ui, sans-serif",
    }}>
      {/* Sidebar */}
      <div style={{
        width: 240, display: 'flex', flexDirection: 'column',
        padding: '12px 10px', margin: 6, borderRadius: 14,
        background: 'rgba(15,15,28,0.75)', backdropFilter: 'blur(20px)',
        border: '1px solid rgba(255,255,255,0.06)',
      }}>
        <div style={{ fontSize: 17, fontWeight: 700, color: '#a78bfa', marginBottom: 16, padding: '0 8px' }}>
          ⚡ Pandora
        </div>

        <button onClick={createNewSession} style={{
          display: 'flex', alignItems: 'center', gap: 8, padding: '8px 12px',
          borderRadius: 8, border: 'none', background: 'rgba(124,58,237,0.15)',
          color: '#c4b5fd', cursor: 'pointer', fontSize: 13, fontWeight: 500,
          marginBottom: 12, width: '100%',
        }}>
          <Plus size={16} /> New Session
        </button>

        <nav style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: 2, marginBottom: 12 }}>
          {([
            { id: 'chat', icon: Terminal, label: 'Chat' },
            { id: 'sessions', icon: History, label: 'Sessions' },
            { id: 'models', icon: Cpu, label: 'Models' },
          ] as const).map(item => (
            <button key={item.id} onClick={() => setTab(item.id)} style={{
              display: 'flex', alignItems: 'center', gap: 8, padding: '8px 10px',
              borderRadius: 8, border: 'none',
              background: tab === item.id ? 'rgba(124,58,237,0.1)' : 'transparent',
              color: tab === item.id ? '#c4b5fd' : 'rgba(255,255,255,0.45)',
              cursor: 'pointer', fontSize: 12, fontWeight: 500,
              width: '100%', textAlign: 'left',
            }}>
              <item.icon size={15} /> {item.label}
            </button>
          ))}
        </nav>

        {/* Model selector */}
        {models.length > 0 && (
          <div style={{ padding: '8px 0', borderTop: '1px solid rgba(255,255,255,0.05)' }}>
            <div style={{ fontSize: 10, color: 'rgba(255,255,255,0.3)', marginBottom: 4, padding: '0 8px' }}>
              MODEL
            </div>
            <select value={selectedModel} onChange={e => {
              const parts = e.target.value.split('|')
              handleModelSwitch(parts[1] || '', parts[0] || '')
            }} style={{
              width: '100%', padding: '6px 8px', borderRadius: 6,
              background: 'rgba(255,255,255,0.04)', border: '1px solid rgba(255,255,255,0.08)',
              color: 'rgba(255,255,255,0.8)', fontSize: 12,
              fontFamily: 'inherit',
            }}>
              {models.map(m => (
                <option key={`${m.provider}|${m.name}`} value={`${m.provider}|${m.name}`}>
                  {m.healthy ? '● ' : '○ '}{m.name} ({m.provider})
                </option>
              ))}
            </select>
          </div>
        )}

        <div style={{ fontSize: 10, color: 'rgba(255,255,255,0.2)', padding: '8px', borderTop: '1px solid rgba(255,255,255,0.05)', marginTop: 'auto' }}>
          v0.5.0 · Phase B
        </div>
      </div>

      {/* Main */}
      <div style={{ flex: 1, display: 'flex', flexDirection: 'column', margin: '6px 6px 6px 0' }}>
        <div style={{
          padding: '6px 14px', borderRadius: '14px 14px 0 0',
          background: 'rgba(20,20,38,0.6)', border: '1px solid rgba(255,255,255,0.06)',
          display: 'flex', alignItems: 'center', gap: 10, fontSize: 11,
          color: 'rgba(255,255,255,0.4)',
        }}>
          <span style={{ width: 8, height: 8, borderRadius: '50%', background: '#22c55e' }} />
          <span style={{ flex: 1 }}>
            {activeSession ? `${activeSession.name} · ${activeSession.model}` : 'No active session'}
          </span>
          {running && <span style={{ color: '#eab308' }}>● running</span>}
        </div>

        <div style={{ flex: 1, display: 'flex', flexDirection: 'column', background: 'rgba(10,10,20,0.45)' }}>
          {tab === 'chat' ? (
            <>
              <div ref={chatRef} style={{
                flex: 1, overflowY: 'auto', padding: '16px 20px',
                display: 'flex', flexDirection: 'column', gap: 10,
              }}>
                {messages.map(msg => (
                  <div key={msg.id} style={{
                    maxWidth: msg.role === 'user' ? '75%' : '90%',
                    alignSelf: msg.role === 'user' ? 'flex-end' : 'flex-start',
                    padding: '8px 14px', borderRadius: 10, fontSize: 13,
                    lineHeight: 1.5, whiteSpace: 'pre-wrap', wordBreak: 'break-word',
                    background: msg.role === 'user'
                      ? 'rgba(124,58,237,0.2)' : msg.role === 'error'
                      ? 'rgba(239,68,68,0.12)' : msg.role === 'tool'
                      ? 'rgba(234,179,8,0.08)' : 'rgba(255,255,255,0.03)',
                    border: '1px solid',
                    borderColor: msg.role === 'user'
                      ? 'rgba(124,58,237,0.25)' : msg.role === 'error'
                      ? 'rgba(239,68,68,0.25)' : msg.role === 'tool'
                      ? 'rgba(234,179,8,0.2)' : 'rgba(255,255,255,0.06)',
                    cursor: msg.role === 'tool' ? 'pointer' : 'default',
                  }} onClick={() => msg.role === 'tool' && toggleTool(msg)}>
                    {msg.role === 'tool' ? (
                      <>
                        <div style={{ display: 'flex', alignItems: 'center', gap: 6, color: '#fbbf24' }}>
                          <Box size={12} />
                          <span style={{ fontWeight: 600, fontSize: 11 }}>{msg.content}</span>
                          {msg.duration !== undefined && (
                            <span style={{ fontSize: 10, color: 'rgba(255,255,255,0.3)', marginLeft: 'auto' }}>
                              {msg.duration}ms
                            </span>
                          )}
                          <span style={{ fontSize: 10, color: 'rgba(255,255,255,0.3)' }}>
                            {msg.expanded ? '▾' : '▸'}
                          </span>
                        </div>
                        {msg.expanded && msg.details && (
                          <div style={{
                            marginTop: 6, padding: '6px 8px',
                            background: 'rgba(0,0,0,0.3)', borderRadius: 6,
                            fontSize: 11, fontFamily: 'monospace', color: 'rgba(255,255,255,0.6)',
                          }}>
                            {msg.details}
                          </div>
                        )}
                      </>
                    ) : (
                      <div style={{ color: msg.role === 'error' ? '#fca5a5' : 'rgba(255,255,255,0.9)' }}>
                        {msg.content}
                      </div>
                    )}
                    {msg.duration !== undefined && msg.role !== 'tool' && (
                      <div style={{ fontSize: 10, color: 'rgba(255,255,255,0.25)', marginTop: 3 }}>
                        {msg.duration}ms
                      </div>
                    )}
                  </div>
                ))}
                {running && streamOutput && (
                  <div style={{
                    alignSelf: 'flex-start', padding: '8px 14px', borderRadius: 10,
                    fontSize: 13, background: 'rgba(255,255,255,0.02)',
                    border: '1px solid rgba(255,255,255,0.04)',
                    color: 'rgba(255,255,255,0.5)',
                  }}>
                    {streamOutput}
                  </div>
                )}
              </div>

              <div style={{ padding: '10px 16px', borderTop: '1px solid rgba(255,255,255,0.05)', display: 'flex', gap: 8 }}>
                <input
                  value={input}
                  onChange={e => setInput(e.target.value)}
                  onKeyDown={e => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); send() } }}
                  disabled={running}
                  placeholder={running ? 'Executing...' : activeSession ? 'Type a task or /command...' : 'Create a session first'}
                  style={{
                    flex: 1, padding: '9px 14px',
                    background: 'rgba(255,255,255,0.03)', border: '1px solid rgba(255,255,255,0.08)',
                    borderRadius: 8, color: 'rgba(255,255,255,0.9)', fontSize: 13,
                    outline: 'none', fontFamily: 'inherit',
                  }}
                />
                <button onClick={send} disabled={running || !input.trim() || !activeSession} style={{
                  padding: '9px 18px', background: 'rgba(124,58,237,0.8)', border: 'none',
                  borderRadius: 8, color: '#fff', cursor: running ? 'default' : 'pointer',
                  fontSize: 13, fontWeight: 600, opacity: running || !activeSession ? 0.4 : 1,
                }}>
                  <Send size={15} />
                </button>
              </div>
            </>
          ) : tab === 'sessions' ? (
            <div style={{ padding: 20, overflowY: 'auto', flex: 1 }}>
              <h3 style={{ fontSize: 14, fontWeight: 600, color: 'rgba(255,255,255,0.5)', marginBottom: 12 }}>
                Sessions
              </h3>
              {sessions.map(s => (
                <div key={s.id} onClick={() => resumeSession(s.id)} style={{
                  padding: '10px 14px', borderRadius: 8, marginBottom: 6,
                  background: activeSession?.id === s.id ? 'rgba(124,58,237,0.1)' : 'rgba(255,255,255,0.02)',
                  border: '1px solid rgba(255,255,255,0.06)', cursor: 'pointer',
                }}>
                  <div style={{ fontWeight: 600, fontSize: 13 }}>{s.name}</div>
                  <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.4)', marginTop: 2 }}>
                    {s.model} · {s.created.slice(0, 19)}
                  </div>
                </div>
              ))}
              {sessions.length === 0 && (
                <div style={{ color: 'rgba(255,255,255,0.3)', fontSize: 13 }}>
                  No sessions yet. Create one above.
                </div>
              )}
            </div>
          ) : (
            <div style={{ padding: 20, overflowY: 'auto', flex: 1 }}>
              <h3 style={{ fontSize: 14, fontWeight: 600, color: 'rgba(255,255,255,0.5)', marginBottom: 12 }}>
                Models & Providers
              </h3>
              {models.map(m => (
                <div key={`${m.provider}-${m.name}`} onClick={() => handleModelSwitch(m.name, m.provider)} style={{
                  padding: '10px 14px', borderRadius: 8, marginBottom: 6,
                  background: selectedModel === m.name ? 'rgba(124,58,237,0.1)' : 'rgba(255,255,255,0.02)',
                  border: '1px solid rgba(255,255,255,0.06)', cursor: 'pointer',
                }}>
                  <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                    <span style={{ width: 6, height: 6, borderRadius: '50%', background: m.healthy ? '#22c55e' : '#ef4444' }} />
                    <span style={{ fontWeight: 600, fontSize: 13 }}>{m.name}</span>
                    <span style={{ fontSize: 11, color: 'rgba(255,255,255,0.4)' }}>{m.provider}</span>
                  </div>
                  <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)', marginTop: 2 }}>
                    {m.endpoint} · {m.context_size.toLocaleString()} ctx
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
