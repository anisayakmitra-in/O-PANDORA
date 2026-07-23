## CLI Screenshot

```bash
$ pandora --version

         /\_/\
    ____/ o o \
   /~____  =  /
  (______)__m_m)
     |        |
     |  ╔══╗ |
     |  ║◇◇║ |
     |  ║◇◇║ |
     |  ╚══╝ |
     |___|||||

pandora 0.1.0 (d31211f)
Platform: linux
Arch: x86_64
```

```bash
$ pandora --help

Pandora — governed execution runtime

USAGE:
    pandora <command> [args]

COMMANDS:
    Execution:
        run <task>            Execute a task through the pipeline
        shell                 Start interactive operator shell
        resume [id]           Resume interrupted execution
        replay <id>           Replay an execution
        explain <id>          Explain execution decisions

    Packages:
        install <pkg>         Install a package (local or Palace)
        search <query>        Search Palace registry
        publish               Publish current package

    Providers:
        providers             List available providers
        connections           List configured connections

    Security:
        doctor                System diagnostics
        keygen                Generate Ed25519 keypair
        verify <pkg>          Verify package signature

    Dev:
        new gene <name>       Scaffold a new gene
        new harness <name>    Scaffold a new harness
        new package <name>    Scaffold a new package
```

```bash
$ pandora doctor

=== Pandora Doctor ===

Ollama... OK
Git... OK
Docker... OK
cargo... OK
python3... OK
rustc... OK

Sessions: 25 stored
Architecture: v0.1.0 — frozen
```

```bash
$ pandora shell

         /\_/\
    ____/ o o \
   /~____  =  /
  (______)__m_m)
     |  ╔══╗ |
     |  ║◇◇║ |
     |  ╚══╝ |
     |___|||||

PANDORA v0.1.0 Interactive Shell
Commands: /run, /sessions, /session, /replay, /providers, /genres, /help, /quit
pandora> /quit
Goodbye.
```