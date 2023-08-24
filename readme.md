# Web Search CLI

Web Search CLI is a tool that trigger web search from command line that inspired by Microsoft PowerToys' PowerToys Run.
PowerToys Run is a utility that is similar to MacOS's spotlight feature and web search is one of it. However, PowerToys
Run's web search feature is broken on Firefox due to a [a bug in Firefox](https://bugzilla.mozilla.org/show_bug.cgi?id=1667577)
which always open the search terms in a new window.

This cli tool is a replacement of it via the PowerToys Run's run shell command feature. It can be use to trigger search
from a command line too. Triggered search will be opened in default browser.

## Usage

1. Add the executable to path and made sure it can be found in command line. Can use the 'Get-Command' cmdlet to verify
   it if in powershell.
2. In PowerToys Run shell command mode, type ws `<search terms>` and press enter.

### Command line args

Usage: `ws <service (optional)> <keyword>`

Example:

- `ws g rust encode url`
- `ws github tauri`
- `ws duckduckgo best linux distro`

Services available:

- g, google: Google Search
- d, duckduckgo: DuckDuckGo Search

If service is omitted, Web Search CLI will trigger search with default search engine, currently Google.

## Build

1. Follow the official documentations to install [Rust](https://www.rust-lang.org/tools/install) and Cargo.
2. Clone or download the repo
3. In terminal, navigate to the downloaded project folder.
4. Run `cargo build -r` to build the project.
5. Copy the `ws.exe` binary to `Windows\System32` folder or add it to environment variable.

## Related Links

- [#15288 Firefox open a new tab instead of a new window when using powertoys run web search plugin](https://github.com/microsoft/PowerToys/issues/15288)
- [Bug 1667577, Command line option to start a search should follow the user's preferences](https://bugzilla.mozilla.org/show_bug.cgi?id=1667577)
