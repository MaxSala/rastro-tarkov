<p align="center">
  <a href="../../README.zh-CN.md">返回 README</a> · <a href="FEATURES.md">功能</a> · <b>指南</b> · <a href="FAQ.md">常见问题与安全</a>
</p>

# 指南

## 安装

1. **[下载 `Rastro-windows-x64.exe`](https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe)**。这个链接始终指向最新版本。
2. 放在任何地方都行：桌面、文档里的某个文件夹，随你喜欢。它旁边不需要任何其他文件。
3. 双击运行。如果 Windows 显示蓝色警告，见[安全吗？](FAQ.md#安全吗)

需要 64 位 Windows 10 或 11。

### 首次启动

第一次运行时会打开一个简短的引导：

1. **截图文件夹。** Rastro 会自动找到 `Documents\Escape from Tarkov\Screenshots`，即使 Windows 把文档移到了 OneDrive 也没问题。如果找不到，粘贴路径后它会告诉你看到了多少张带坐标的截图。你也可以在这里开启战局检测。
2. **地图。** 从 tarkov.dev 下载社区地图（推荐）、导入地图包，或者试试演示地图。
3. 完成。在游戏里截一张图，你的箭头就会出现在地图上。

要使用市场、弹药表和钥匙名称，请先打开一次市场标签页并点击 *下载价格*。

### 更新

如果你允许了检查更新，有新版本时顶部栏会出现一个按钮，带你到下载页面。否则，就通过[上面的链接](#安装)下载新的 `.exe`，关闭 Rastro，用新文件替换旧文件。你的设置、标点、地图和价格都[单独保存](#数据存放位置)，不会丢失。

更新后，Rastro 会告诉你改了什么。完整历史见 *设置 → 关于 → 新功能* 和 [CHANGELOG.md](../../CHANGELOG.md)。如果更新带来了新的地图数据，请在地图面板中使用 *从 tarkov.dev 更新* 获取。

### 快捷键

| 操作 | 方式 |
|---|---|
| 更新你的位置 | 在游戏中截图 |
| 切换版块（地图、市场、弹药、任务、藏身处） | `Ctrl+1`、`Ctrl+2`、`Ctrl+3`、`Ctrl+4`、`Ctrl+5` |
| 命令面板 | `Ctrl+K` |
| 撤离点、标点、小队、地图和任务面板 | `1`、`2`、`3`、`4`、`5` |
| 新战局（清除轨迹和标记） | `N` |
| 查看整张地图 / 跟随你的位置 | `F` / `G` |
| 缩放 | `+` / `-` 或鼠标滚轮 |
| 添加标点、导航到某处 | 在地图上右键 |
| 给小队标点 | `Shift` + 点击 |
| 搜索市场 | `/` |
| 设置 | `,` |

### 数据存放位置

设置、你的标点、任务进度、下载的地图以及价格和任务缓存都放在一个文件夹里，与 `.exe` 分开：

| 系统 | 文件夹 |
|---|---|
| Windows | `%APPDATA%\Rastro\data\` |
| Linux | `~/.local/share/rastro/` |
| macOS | `~/Library/Application Support/Rastro/` |

*设置 → 日志* 里有一个打开它的按钮。

### 遇到问题

- **在浏览器里打开，而不是自己的窗口。** Rastro 使用 Microsoft WebView2 绘制窗口。Windows 10 和 11 自带它，但如果被删除了，Rastro 会改用你的浏览器打开。安装 [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) 后窗口就会恢复。
- **错误页面提示端口已被占用。** 可能还有另一个 Rastro 在运行。从任务栏或任务管理器关闭它（查找 `rastro.exe`），再重新打开。
- **我的位置一直不动。** 检查 *设置 → 常规* 里的截图文件夹是否就是游戏实际保存截图的位置，以及你按的是不是游戏内的截图键。
- **队友进不了我的房间。** 他们需要能连接到你的电脑。创建房间时使用组网 VPN（Tailscale、ZeroTier）提供的地址，或者在路由器上开放小队端口。
- **还是不行？** 到 [Discord](https://discord.gg/H4pRxGBGDb) 提问。*设置 → 日志* 里有 *复制* 按钮，贴出这份日志会很有帮助。

## 工作原理

Rastro 读取《逃离塔科夫》本来就会保存在你电脑上的两类文件，除此之外什么都不需要。

| 读取什么 | 位置 | Rastro 从中获得什么 |
|---|---|---|
| **截图的文件名** | `Documents\Escape from Tarkov\Screenshots` | 你的位置和朝向。塔科夫会把你的坐标写进每张截图的文件名里。Rastro 只读文件名，从不读图片内容，也从不移动、修改或删除你的截图。 |
| **游戏日志**（可选，默认关闭，需要你手动开启） | `Battlestate Games\Escape from Tarkov\Logs` | 正在加载哪张地图、战局何时开始，以及你在玩 PvP 还是 PvE。Rastro 据此自动切换到正确的地图，每场战局开始新的轨迹，并显示剩余战局时间。只保存地图名称和游戏模式。它还会读取商人消息，用来标记你接受、提交或失败的任务。 |

实际使用时：你在游戏里按下截图键，一秒后你的箭头就会在地图上移动。

Rastro **不会**做的事：不读取游戏内存，不注入任何东西，不挂钩游戏进程，不抓取网络流量，不在游戏画面上绘制任何内容，也不会替你按键。它是一个独立程序，只查看硬盘上的文件。

## 从源码构建

只有想修改代码或自己构建 `.exe` 时才需要这一步。你需要 [Rust](https://rustup.rs/) 1.85 或更新版本，以及 [Node.js](https://nodejs.org/) LTS（Linux 上还需要 `libwebkit2gtk-4.1-dev` 和 `libgtk-3-dev`）：

```sh
git clone https://github.com/MaxSala/rastro-tarkov.git
cd rastro-tarkov
./build.sh        # Windows 上用 build.bat
```

构建结果是 `target/release/` 下的单个可执行文件。开发环境配置、项目结构、代码规范以及如何添加翻译，见 **[CONTRIBUTING.md](../../CONTRIBUTING.md)**（英文）。
