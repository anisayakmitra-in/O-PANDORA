import { useState, useEffect, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog'
import { Send, Plus, History, Cpu, FolderOpen, FileText, Box, Terminal, Download } from 'lucide-react'

interface SessionMeta {
  id: string
  created: string
  task?: string
}

interface ModelInfo {
  name: string
  provider: string
  endpoint?: string
  healthy: boolean
  context_size?: number
}

interface ProjectInfo {
  path: string
  name: string
  branch: string
  dirty: boolean
}

interface FileEntry {
  name: string
  path: string
  is_dir: boolean
  children?: FileEntry[]
}

interface ApprovalItem {
  id: string
  tool: string
  reason: string
  status: string
  created_ms: number
}

interface GovernanceSummary {
  pending: number
  approved: number
  rejected: number
  total: number
  recent: ApprovalItem[]
}

interface Message {
  id: string
  role: 'user' | 'system' | 'tool' | 'error'
  content: string
  details?: string
  duration?: number
  expanded?: boolean
}

function FileTree({ entries, onSelect, depth = 0 }: {
  entries: FileEntry[]
  onSelect: (entry: FileEntry) => void
  depth?: number
}) {
  return (
    <>
      {entries.map(entry => (
        <div key={entry.path}>
          <button
            className={`file-tree-entry ${entry.is_dir ? 'file-tree-directory' : ''}`}
            style={{ paddingLeft: `${10 + depth * 14}px` }}
            onClick={() => onSelect(entry)}
          >
            <span className="file-tree-icon">{entry.is_dir ? <FolderOpen size={13} /> : <FileText size={13} />}</span>
            {entry.name}
          </button>
          {entry.is_dir && entry.children && (
            <FileTree entries={entry.children} onSelect={onSelect} depth={depth + 1} />
          )}
        </div>
      ))}
    </>
  )
}

type Tab = 'chat' | 'project' | 'sessions' | 'models'

export default function App() {
  const [messages, setMessages] = useState<Message[]>([])
  const [input, setInput] = useState('')
  const [running, setRunning] = useState(false)
  const [tab, setTab] = useState<Tab>('chat')
  const [sessions, setSessions] = useState<SessionMeta[]>([])
  const [activeSession, setActiveSession] = useState<SessionMeta | null>(null)
  const [models, setModels] = useState<ModelInfo[]>([])
  const [governance, setGovernance] = useState<GovernanceSummary | null>(null)
  const [project, setProject] = useState<ProjectInfo | null>(null)
  const [projectInput, setProjectInput] = useState('')
  const [projectError, setProjectError] = useState('')
  const [fileTree, setFileTree] = useState<FileEntry[]>([])
  const [selectedFile, setSelectedFile] = useState<FileEntry | null>(null)
  const [fileContent, setFileContent] = useState('')
  const [fileError, setFileError] = useState('')
  const [selectedModel, setSelectedModel] = useState('')
  const [profiles, setProfiles] = useState<string[]>([])
  const [selectedProfile, setSelectedProfile] = useState('')
  const [streamOutput, setStreamOutput] = useState('')
  const [lastEvent, setLastEvent] = useState('Ready')
  const [runtimeStatus, setRuntimeStatus] = useState<'ready' | 'running' | 'error'>('ready')
  const chatRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    loadSessions()
    loadModels()
    loadProfiles()
    loadGovernance()
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
        setLastEvent(content || type)
        switch (type) {
          case 'execution.started':
            setStreamOutput(`• ${content}`)
            break
          case 'gene.executed':
            addToolMessage(metadata?.gene || 'tool', content, metadata?.duration_ms)
            break
          case 'execution.completed':
            setRuntimeStatus('ready')
            void loadGovernance()
            setStreamOutput('')
            addSystemMessage(content, metadata?.duration_ms)
            setRunning(false)
            break
          case 'execution.failed':
            setRuntimeStatus('error')
            void loadGovernance()
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

  async function loadProfiles() {
    try {
      const names = await invoke<string[]>('list_profiles')
      setProfiles(names)
    } catch {}
  }

  async function loadGovernance() {
    try {
      const summary = await invoke<GovernanceSummary>('governance_summary')
      setGovernance(summary)
    } catch {}
  }

  async function resolveApproval(id: string, action: 'approve_pending' | 'reject_pending') {
    try {
      await invoke<string>(action, { id })
      await loadGovernance()
    } catch (error) {
      setLastEvent(error instanceof Error ? error.message : String(error))
      setRuntimeStatus('error')
    }
  }

  async function selectProject() {
    try {
      const selected = await openDialog({ directory: true, multiple: false, title: 'Open project' })
      if (typeof selected === 'string') {
        setProjectInput(selected)
        await openProject(selected)
      }
    } catch (error) {
      setProjectError(error instanceof Error ? error.message : String(error))
    }
  }

  async function openProject(selectedPath?: string) {
    const path = (selectedPath ?? projectInput).trim()
    if (!path) return
    try {
      const opened = await invoke<ProjectInfo>('open_project', { path })
      setProject(opened)
      setProjectInput(opened.path)
      setProjectError('')
      await loadFileTree()
    } catch (error) {
      setProjectError(error instanceof Error ? error.message : String(error))
    }
  }

  async function loadFileTree() {
    try {
      const entries = await invoke<FileEntry[]>('get_file_tree', {})
      setFileTree(entries)
      setFileError('')
    } catch (error) {
      setFileError(error instanceof Error ? error.message : String(error))
    }
  }

  async function openFile(entry: FileEntry) {
    if (entry.is_dir) return
    try {
      const content = await invoke<string>('read_file_content', { path: entry.path })
      setSelectedFile(entry)
      setFileContent(content)
      setFileError('')
    } catch (error) {
      setFileError(error instanceof Error ? error.message : String(error))
    }
  }

  async function createNewSession(): Promise<SessionMeta | null> {
    const name = `Session ${sessions.length + 1}`
    try {
      const s = await invoke<SessionMeta>('create_session', { task: name })
      setActiveSession(s)
      setMessages([])
      setSessions(prev => [s, ...prev])
      return s
    } catch (e: any) {
      console.error('Create session failed:', e)
      return null
    }
  }

  async function exportSessions(format: 'json' | 'markdown') {
    try {
      const content = await invoke<string>('export_sessions', { format, redact: true })
      const path = await saveDialog({
        title: `Export Pandora sessions as ${format}`,
        defaultPath: `pandora-sessions.${format === 'json' ? 'json' : 'md'}`,
        filters: [{ name: format === 'json' ? 'JSON' : 'Markdown', extensions: [format === 'json' ? 'json' : 'md'] }],
      })
      if (typeof path === 'string') {
        const { writeTextFile } = await import('@tauri-apps/plugin-fs')
        await writeTextFile(path, content)
        setLastEvent(`Exported sessions to ${path}`)
      }
    } catch (error) {
      setLastEvent(error instanceof Error ? error.message : String(error))
    }
  }

  async function resumeSession(sessionId: string) {
    try {
      const s = await invoke<SessionMeta>('resume_session', { sessionId })
      setActiveSession(s)
      setMessages([{
        id: crypto.randomUUID(),
        role: 'system',
        content: `Resumed session: ${s.task ?? s.id}`,
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

    const session = activeSession ?? await createNewSession()
    if (!session) return

    setMessages(prev => [...prev, {
      id: crypto.randomUUID(),
      role: 'user',
      content: text,
    }])
    setRunning(true)

    try {
      await invoke('send_message', { message: text, profile: selectedProfile || null })
    } catch (e: any) {
      const error = e instanceof Error ? e.message : String(e)
      setLastEvent(error)
      setRuntimeStatus('error')
      addErrorMessage(error)
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
    <div className="app-shell" style={{
      display: 'flex', height: '100vh',
      background: 'linear-gradient(135deg, #0a0a14 0%, #0d0d20 40%, #0f0a1e 100%)',
      fontFamily: "'Inter', system-ui, sans-serif",
    }}>
      {/* Sidebar */}
      <div className="sidebar" style={{
        width: 240, display: 'flex', flexDirection: 'column',
        padding: '12px 10px', margin: 6, borderRadius: 14,
        background: 'rgba(15,15,28,0.75)', backdropFilter: 'blur(20px)',
        border: '1px solid rgba(255,255,255,0.06)',
      }}>
        <div className="sidebar-brand" style={{ fontSize: 17, fontWeight: 700, color: '#a78bfa', marginBottom: 16, padding: '0 8px' }}>
          <img src="/pandora-logo.png" alt="" className="brand-mark" />
          <span>Pandora</span>
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
            { id: 'project', icon: FolderOpen, label: 'Project' },
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
            {profiles.length > 0 && (
              <>
                <div style={{ fontSize: 10, color: 'rgba(255,255,255,0.3)', marginBottom: 4, padding: '0 8px' }}>
                  PROFILE
                </div>
                <select value={selectedProfile} onChange={e => setSelectedProfile(e.target.value)} style={{
                  width: '100%', padding: '6px 8px', marginBottom: 8, borderRadius: 6,
                  background: 'rgba(255,255,255,0.04)', border: '1px solid rgba(255,255,255,0.08)',
                  color: 'rgba(255,255,255,0.8)', fontSize: 12,
                }}>
                  <option value="">Default</option>
                  {profiles.map(profile => <option key={profile} value={profile}>{profile}</option>)}
                </select>
              </>
            )}
            <div style={{ fontSize: 10, color: 'rgba(255,255,255,0.3)', marginBottom: 4, padding: '0 8px' }}>
              MODEL
            </div>
            <select value={selectedModel} onChange={e => {
              const model = models.find(item => item.name === e.target.value)
              if (model) handleModelSwitch(model.name, model.provider)
            }} style={{
              width: '100%', padding: '6px 8px', borderRadius: 6,
              background: 'rgba(255,255,255,0.04)', border: '1px solid rgba(255,255,255,0.08)',
              color: 'rgba(255,255,255,0.8)', fontSize: 12,
              fontFamily: 'inherit',
            }}>
              {models.map(m => (
                <option key={`${m.provider}|${m.name}`} value={m.name}>
                  {m.healthy ? '● ' : '○ '}{m.name} ({m.provider})
                </option>
              ))}
            </select>
          </div>
        )}

        <div style={{ fontSize: 10, color: 'rgba(255,255,255,0.2)', padding: '8px', borderTop: '1px solid rgba(255,255,255,0.05)', marginTop: 'auto' }}>
          v0.5.0 · Local runtime
        </div>
      </div>

      {/* Main */}
      <div className="main" style={{ flex: 1, display: 'flex', flexDirection: 'column', margin: '6px 6px 6px 0' }}>
        <div style={{
          padding: '6px 14px', borderRadius: '14px 14px 0 0',
          background: 'rgba(20,20,38,0.6)', border: '1px solid rgba(255,255,255,0.06)',
          display: 'flex', alignItems: 'center', gap: 10, fontSize: 11,
          color: 'rgba(255,255,255,0.4)',
        }}>
          <span style={{ width: 8, height: 8, borderRadius: '50%', background: '#22c55e' }} />
          <span style={{ flex: 1 }}>
            {activeSession ? `${activeSession.task ?? activeSession.id}  -  ${selectedModel || 'no model'}` : 'No active session'}
          </span>
          {running && <span style={{ color: '#eab308' }}>● running</span>}
        </div>

        <div className="content" style={{ flex: 1, display: 'flex', flexDirection: 'column', background: 'rgba(10,10,20,0.45)' }}>
          {tab === 'chat' ? (
            <>
              <div ref={chatRef} className="messages" style={{
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

              <div className="input-bar" style={{ padding: '10px 16px', borderTop: '1px solid rgba(255,255,255,0.05)', display: 'flex', gap: 8 }}>
                <input
                  value={input}
                  onChange={e => setInput(e.target.value)}
                  onKeyDown={e => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); send() } }}
                  disabled={running}
                  placeholder={running ? 'Executing...' : 'Describe a task or type /command...'}
                  style={{
                    flex: 1, padding: '9px 14px',
                    background: 'rgba(255,255,255,0.03)', border: '1px solid rgba(255,255,255,0.08)',
                    borderRadius: 8, color: 'rgba(255,255,255,0.9)', fontSize: 13,
                    outline: 'none', fontFamily: 'inherit',
                  }}
                />
                <button onClick={send} disabled={running || !input.trim()} style={{
                  padding: '9px 18px', background: 'rgba(124,58,237,0.8)', border: 'none',
                  borderRadius: 8, color: '#fff', cursor: running ? 'default' : 'pointer',
                  fontSize: 13, fontWeight: 600, opacity: running || !input.trim() ? 0.4 : 1,
                }}>
                  <Send size={15} />
                </button>
              </div>
            </>
          ) : tab === 'project' ? (
            <div className="project-panel">
              <div className="file-tree-panel">
                <div className="panel-title">Project files</div>
                {!project && <p className="project-empty">Choose a project from the inspector.</p>}
                {project && fileTree.length === 0 && <p className="project-empty">No readable files found.</p>}
                {project && fileTree.length > 0 && <FileTree entries={fileTree} onSelect={entry => void openFile(entry)} />}
                {fileError && <p className="inspector-error">{fileError}</p>}
              </div>
              <div className="file-preview-panel">
                {selectedFile ? (
                  <>
                    <div className="file-preview-heading">
                      <strong>{selectedFile.path}</strong>
                      <span>{fileContent.split('\n').length} lines</span>
                    </div>
                    <pre className="file-preview-content">{fileContent}</pre>
                  </>
                ) : (
                  <p className="project-empty">Select a file to preview it.</p>
                )}
              </div>
            </div>
          ) : tab === 'sessions' ? (
            <div className="panel" style={{ padding: 20, overflowY: 'auto', flex: 1 }}>
              <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 12 }}>
                <h3 style={{ fontSize: 14, fontWeight: 600, color: 'rgba(255,255,255,0.5)', margin: 0 }}>
                  Sessions
                </h3>
                <div style={{ display: 'flex', gap: 6 }}>
                  <button className="ghost-button" onClick={() => exportSessions('markdown')} title="Export redacted Markdown">
                    <Download size={13} /> MD
                  </button>
                  <button className="ghost-button" onClick={() => exportSessions('json')} title="Export redacted JSON">
                    <Download size={13} /> JSON
                  </button>
                </div>
              </div>
              {sessions.map(s => (
                <div key={s.id} onClick={() => resumeSession(s.id)} style={{
                  padding: '10px 14px', borderRadius: 8, marginBottom: 6,
                  background: activeSession?.id === s.id ? 'rgba(124,58,237,0.1)' : 'rgba(255,255,255,0.02)',
                  border: '1px solid rgba(255,255,255,0.06)', cursor: 'pointer',
                }}>
                  <div style={{ fontWeight: 600, fontSize: 13 }}>{s.task ?? s.id}</div>
                  <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.4)', marginTop: 2 }}>
                    {selectedModel || 'no model'}  -  {s.created.slice(0, 19)}
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
            <div className="panel" style={{ padding: 20, overflowY: 'auto', flex: 1 }}>
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
                    {m.endpoint ?? 'unknown endpoint'}  -  {(m.context_size ?? 0).toLocaleString()} ctx
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      <aside className="inspector" aria-label="Execution inspector">
        <div className="inspector-card inspector-status">
          <div className="inspector-label">Runtime</div>
          <div className="inspector-status-row">
            <span className={`status-dot status-${runtimeStatus}`} />
            <strong>{runtimeStatus === 'running' ? 'Executing' : runtimeStatus === 'error' ? 'Needs attention' : 'Ready'}</strong>
          </div>
          <p className="inspector-event">{lastEvent}</p>
        </div>

        <div className="inspector-card">
          <div className="inspector-label">Project</div>
          {project ? (
            <>
              <strong>{project.name}</strong>
              <p className="inspector-muted">{project.branch}{project.dirty ? ' ? changes present' : ' ? clean'}</p>
              <p className="inspector-path">{project.path}</p>
            </>
          ) : (
            <>
              <input
                className="inspector-input"
                value={projectInput}
                onChange={event => setProjectInput(event.target.value)}
                onKeyDown={event => { if (event.key === 'Enter') void openProject() }}
                placeholder="Local project path"
                aria-label="Local project path"
              />
              <button className="inspector-action" onClick={() => void selectProject()}>
                Choose project
              </button>
              <button className="inspector-action inspector-action-secondary" onClick={() => void openProject()} disabled={!projectInput.trim()}>
                Open typed path
              </button>
              {projectError && <p className="inspector-error">{projectError}</p>}
            </>
          )}
        </div>

        <div className="inspector-card">
          <div className="inspector-label">Session</div>
          <strong>{activeSession?.task ?? 'No active session'}</strong>
          <p className="inspector-muted">{activeSession?.id ?? 'Create a session by sending a task.'}</p>
        </div>

        <div className="inspector-card">
          <div className="inspector-label">Model</div>
          <strong>{selectedModel || 'Not configured'}</strong>
          <p className="inspector-muted">Profile: {selectedProfile || 'default'}</p>
          <p className="inspector-muted">{models.length ? `${models.length} provider model${models.length === 1 ? '' : 's'}` : 'Configure a provider from the CLI.'}</p>
        </div>


        <div className="inspector-card">
          <div className="inspector-label">Governance</div>
          <strong>{governance ? governance.pending : 0} pending approval{governance && governance.pending === 1 ? '' : 's'}</strong>
          <p className="inspector-muted">{governance ? `${governance.approved} approved ? ${governance.rejected} rejected` : 'Approval state unavailable.'}</p>
          {governance?.recent.filter(item => item.status === 'Pending').slice(0, 2).map(item => (
            <div className="approval-item" key={item.id}>
              <strong>{item.tool}</strong>
              <p>{item.reason}</p>
              <div className="approval-actions">
                <button className="approval-button approval-allow" onClick={() => void resolveApproval(item.id, 'approve_pending')}>Approve</button>
                <button className="approval-button approval-deny" onClick={() => void resolveApproval(item.id, 'reject_pending')}>Reject</button>
              </div>
            </div>
          ))}
        </div>
      </aside>
    </div>
  )
}
