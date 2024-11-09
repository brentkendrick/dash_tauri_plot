# Building the app

## Part 1. Create the app file in the project directory (e.g. dash_electron_graph):

Create a subdirectory in your project to hold the app, e.g.

```bash
mkdir dash_tauri_graph
```

`/dash_tauri_graph/app.py`

```py
from dash import Dash, html, dcc
import plotly.express as px
import pandas as pd


# Sample data
df = pd.DataFrame(
    {
        "Fruit": ["Apples", "Oranges", "Bananas", "Grapes"],
        "Amount": [4, 1, 2, 5],
        "City": ["SF", "SF", "NYC", "NYC"],
    }
)

fig = px.bar(df, x="Fruit", y="Amount", color="City", barmode="group")

app = Dash(__name__)
app.layout = html.Div(
    children=[
        html.H1(children="Hello Dash"),
        html.Div(children="A simple web app with Dash."),
        dcc.Graph(id="example-graph", figure=fig),
    ]
)


if __name__ == "__main__":
    app.run_server(debug=False)

```

Install the dependencies in requirements.txt, then run the app with

```bash
python app.py
```

If there is a port conflict, find the offending PID with:

> MACOS `lsof -i :8050` and stop it with `kill -9 <<PID>>` (replace <\<PID>> with the PID number)

> WINDOWS `netstat -ano | findstr :8050` and stop it with `Stop-Process -Id <PID>` -Force

## Part 2. Create a local executable file with pyinstaller

Before creating a standalone app that doesn't require a browser, we need to bundle python and all the applicable python libraries into a local executable file.

To ensure that the Dash app works correctly when bundled with PyInstaller, it's better to create a spec file that handles how PyInstaller will package the app.

### Step 1. First, in the project root dir, generate a basic spec file:

```bash
pyinstaller --name app --onefile app.py
```

### Step 2. Test the Standalone Executable

After PyInstaller finishes packaging your app, the resulting executable file will be located in the dist/ folder. You can run it directly:

```bash
./dist/app
```

This should launch your Dash app (it may take a minute or so to launch), but it will now be a standalone executable without needing to install Python or any dependencies. After you see a message in the terminal that Dash is running, open a browser at localhost:8050 to verify it is working.

## Part 3. Package the Executable into a Rust Tauri app

To wrap your Dash app in Tauri, it should offer better performance than Electron or PyWebView.

### Step 1. Install Rust

You will need to install Rust:

Install [Rust installation page](https://www.rust-lang.org/tools/install) if you haven't already.

Update it (and check for existing installation) with:

```bash
rustup update
```

### Step 2: Set Up Tauri

Install Tauri's CLI:

```bash
cargo install tauri-cli
```

Create a Tauri Project:
• From your terminal, initialize a Tauri project within your Dash app’s project folder:

```bash
npx create-tauri-app
```

• Choose:
✔ Project name · dash-tauri-plot
✔ Identifier · com.dash-tauri-plot.app
✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm, deno, bun)
✔ Choose your package manager · npm
✔ Choose your UI template · Vanilla
✔ Choose your UI flavor · JavaScript

To get started run:
cd dash-tauri-plot
npm install
npm run tauri android init

For Desktop development, run:
npm run tauri dev

• This command will set up a Tauri project structure and download the necessary dependencies.

Update Tauri Configuration for Dash
In the Tauri project folder, locate the src-tauri/tauri.conf.json file. You’ll configure it to point to your Dash app’s server URL (e.g., http://localhost:8050).

```json
{
  "build": {
    "beforeDevCommand": "python -m dash_app", // Run Dash in dev mode
    "beforeBuildCommand": "python -m dash_app", // Run Dash in prod mode
    "distDir": "../dash_app",
    "devPath": "http://localhost:8050" // Set to Dash app URL
  },
  "tauri": {
    "windows": [
      {
        "label": "main",
        "url": "http://localhost:8050",
        "fullscreen": false,
        "title": "My Dash App"
      }
    ]
  }
}
```

### Step 3. Move app.exe into a dedicated directory

```
DASH_ELECTRON_PLOT/
├── app.py
├── main.js
├── package.json
├── python_dist/          # New folder for app.exe
│   └── app.exe           # Output from PyInstaller
└── requirements.txt
```

### Step 4: Package the Electron App

Now, install Electron Packager to package the app into a standalone executable:

```bash
npm install electron-packager --save-dev
```

Modify your package.json to add a link to main.js (or replace "index.js"), fix the "scripts" and "extraResources" as shown in the example below:

```json
{
  "name": "dash_electron",
  "version": "1.0.0",
  "description": "",
  "main": "main.js",
  "scripts": {
    "start": "electron .",
    "package-win": "electron-packager . MyDashElectronApp --platform=win32 --arch=x64 --out=dist --overwrite --asar"
  },
  "keywords": [],
  "author": "",
  "license": "ISC",
  "devDependencies": {
    "electron": "^33.1.0",
    "electron-packager": "^17.1.2"
  },
  "extraResources": [
    {
      "from": "./python_dist/",
      "to": "python_dist/"
    }
  ]
}
```

To run the Electron app, simply run:

```bash
npm start
```

This will launch the Electron app, which will start the Dash app in a browser window. Sometimes it starts minimized, check the app tray and click on it to open the window. Sometimes, especially with a fresh build, you will get "electron: Failed to load URL: http://localhost:8050/ with error: ERR_CONNECTION_REFUSED". This usually will resolve after letting the app run for a few minutes, and then in the menu bar selecting View - Reload, or close and re-run the app again.

Install electron-packager:

```bash
npm install electron-packager --save-dev
```

Package the app:

```bash
npm run package-win
```
